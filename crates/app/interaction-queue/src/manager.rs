use crate::prelude::*;
use async_std::sync::RwLock;
use async_std::task;
use std::time::Duration;

pub struct InteractionsQueueManager {
    /// The queue of interactions.
    queue: RwLock<InteractionsQueue>,

    /// Observer to handle updates to the interactions queue.
    observer: Arc<dyn InteractionsQueueObserver>,

    /// Storage for saving and loading the queue.
    storage: Arc<dyn InteractionsQueueStorage>,

    // TODO: Improve this
    gateway_client: GatewayClient,
}

impl InteractionsQueueManager {
    pub fn new(
        observer: Arc<dyn InteractionsQueueObserver>,
        storage: Arc<dyn InteractionsQueueStorage>,
        gateway_client: GatewayClient,
    ) -> Arc<Self> {
        Arc::new(Self {
            queue: RwLock::new(InteractionsQueue::new()),
            observer,
            storage,
            gateway_client,
        })
    }
}

// Exported methods (called by Hosts)
impl InteractionsQueueManager {
    /// Method to be called by hosts every time the app is started (or whenever we want)
    /// It will remove the stale data (success transactions).
    pub async fn bootstrap(self: Arc<Self>) -> Result<()> {
        // Load queue from local storage.
        if let Some(queue) = self.storage.load_queue().await? {
            let mut locked_queue = self.queue.write().await;
            *locked_queue = queue;

            // Remove stale data from queue.
            locked_queue.removing_stale();
        }

        // Notify observer and save the queue to local storage.
        self.handle_queue_update().await;

        // Monitor the status of interactions in the queue.
        let self_clone = self.clone();
        async_std::task::spawn(async move {
            self_clone.monitor_interactions_status().await;
        });

        // Monitor the batches in the queue.
        let self_clone = self.clone();
        async_std::task::spawn(async move {
            self_clone.monitor_batches().await;
        });

        Ok(())
    }

    pub fn retry_interaction(&self, _interaction_id: Uuid) {
        // Find interaction in `queue.items` and retry it
    }

    pub fn dismiss_interaction(&self, _interaction_id: Uuid) {
        // Remove interaction from `queue.items`
    }

    pub fn cancel_interaction(&self, _interaction_id: Uuid, _batch_id: Uuid) {
        // Find batch in `queue.batches` and remove the interaction with given id from its list.
    }
}

// Internal methods (called by other places inside Sargon)
impl InteractionsQueueManager {
    pub async fn add_interaction(&self, item: InteractionQueueItem) {
        self.enqueue_and_process_interaction(item).await;
        self.handle_queue_update().await;
    }

    pub async fn add_batch(&self, batch: InteractionQueueBatch) {
        self.queue.write().await.add_batch(batch);
        self.handle_queue_update().await;
    }
}

// Private methods
impl InteractionsQueueManager {
    /// Notifies the observer about the updated queue and saves it to the local storage.
    // Note: we probably want this method to not await until the storage is saved
    async fn handle_queue_update(&self) {
        {
            let queue = self.queue.read().await;
            self.observer.handle_update(queue.sorted_items());
        }

        let mut queue = self.queue.write().await;
        let _ = self.storage.save_queue(queue.clone()).await;
    }

    /// Monitors the status of interactions in the queue.
    /// Every 3 seconds, it will check the status of every interaction that is currently `InProgress`,
    /// and those which have finished will have its status updated in the queue.
    async fn monitor_interactions_status(self: Arc<Self>) {
        loop {
            self.check_in_progress_interactions_status().await;
            task::sleep(self.get_sleep_duration()).await;
        }
    }

    /// Monitors the batches in the queue.
    /// Every 3 seconds, it will check which interactions with status `Next` are ready to be processed.
    async fn monitor_batches(self: Arc<Self>) {
        loop {
            self.check_batch_ready_interactions().await;
            task::sleep(self.get_sleep_duration()).await;
        }
    }

    fn get_sleep_duration(&self) -> Duration {
        #[cfg(test)]
        let result = Duration::from_millis(INTERACTION_QUEUE_POLLING_INTERVAL);
        #[cfg(not(test))]
        let result = Duration::from_secs(INTERACTION_QUEUE_POLLING_INTERVAL);

        result
    }

    /// Checks the status of every interaction that is currently `InProgress`.
    /// Every interaction which has finished will be updated in the queue.
    async fn check_in_progress_interactions_status(&self) {
        let queue = self.queue.read().await;
        let items: Vec<_> = queue
            .items
            .iter()
            .filter(|item| {
                item.status == InteractionQueueItemStatus::InProgress
            })
            .cloned()
            .collect();
        drop(queue);

        // Note: Once Gateway supports checking the status of multiple interactions at the same time,
        // this loop will be replaced with one single call to the Gateway.
        // Related discussion: https://rdxworks.slack.com/archives/C071XGF4G6M/p1740041597061639
        let mut updated: Vec<InteractionQueueItem> = Vec::new();
        for item in items {
            if let Some(updated_item) = self.get_interaction_status(item).await
            {
                updated.push(updated_item);
            }
        }

        let mut queue = self.queue.write().await;
        for item in updated {
            queue.replace_interaction(item);
        }
        drop(queue);

        self.handle_queue_update().await;
    }

    /// Check the status of a single interaction that is `InProgress`.
    /// Returns the updated interaction if it has finished, otherwise returns `None`.
    async fn get_interaction_status(
        &self,
        interaction: InteractionQueueItem,
    ) -> Option<InteractionQueueItem> {
        let mut updated_interaction = interaction.clone();
        match interaction.kind {
            InteractionQueueItemKind::Transaction(tx) => {
                let response = match self
                    .gateway_client
                    .get_transaction_status(tx.transaction_id)
                    .await
                {
                    Ok(response) => response,
                    Err(_) => return None,
                };

                if let Some(payload_status) = response
                    .known_payloads
                    .first()
                    .and_then(|payload| payload.payload_status.clone())
                {
                    let status =
                        InteractionQueueItemStatus::from(payload_status);
                    if status != InteractionQueueItemStatus::InProgress {
                        updated_interaction.status = status;
                        return Some(updated_interaction);
                    }
                }
                None
            }
            InteractionQueueItemKind::PreAuthorization(_) => {
                panic!("Not implemented yet");
            }
        }
    }

    /// Loop over the batches and call `batch.get_first_if_ready()` for each of them.
    /// If they return an interaction, call `process_interaction()` with it.
    /// Call observer to notify of updated queue.
    async fn check_batch_ready_interactions(&self) {
        let mut queue = self.queue.write().await;
        let ready_interactions: Vec<_> = queue
            .batches
            .iter_mut()
            .filter_map(|batch| batch.get_first_if_ready())
            .collect();
        drop(queue);

        for interaction in ready_interactions {
            self.enqueue_and_process_interaction(interaction).await;
        }

        self.handle_queue_update().await;
    }

    /// Adds interaction to the queue and processes it.
    async fn enqueue_and_process_interaction(
        &self,
        item: InteractionQueueItem,
    ) {
        let mut item = item.clone();
        item.status = InteractionQueueItemStatus::InProgress;

        self.queue.write().await.add_interaction(item.clone());

        if let InteractionQueueItemKind::Transaction(transaction) = item.kind {
            let request = TransactionSubmitRequest {
                notarized_transaction_hex: transaction
                    .notarized_transaction_hex,
            };
            let _ = self
                .gateway_client
                .submit_transaction(request, transaction.transaction_id)
                .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::*;

    #[actix_rt::test]
    async fn bootstrap_loads_empty_queue() {
        // Test the case the queue loaded from storage is empty.

        let observer = Arc::new(MockObserver::new());
        let storage = Arc::new(MockStorage::new_empty());
        let _ = create_and_bootstrap(vec![], observer.clone(), storage.clone())
            .await;

        // Verify empty queue update
        let expected_queue = InteractionsQueue::with_items(vec![]);
        verify_queue_update(observer.clone(), storage.clone(), expected_queue);
    }

    #[actix_rt::test]
    async fn bootstrap_loads_queue_with_stale_data() {
        // Test the case the queue loaded from storage has some stale data

        // Set up the Queue with 4 interactions and 2 batches
        let interaction_in_progress =
            InteractionQueueItem::sample_in_progress();
        let interaction_suceess = InteractionQueueItem::sample_success();
        let interaction_failed = InteractionQueueItem::sample_failed();

        let batch_empty = InteractionQueueBatch::empty();
        let batch_non_empty = InteractionQueueBatch::with_items([
            InteractionQueueItem::sample_queued(),
        ]);

        let stored_queue = InteractionsQueue::with_items_and_batches(
            [
                interaction_in_progress.clone(),
                interaction_suceess.clone(),
                interaction_failed.clone(),
            ],
            [batch_empty.clone(), batch_non_empty.clone()],
        );

        let observer = Arc::new(MockObserver::new());
        let storage = Arc::new(MockStorage::new_with_queue(stored_queue));
        let _ = create_and_bootstrap(vec![], observer.clone(), storage.clone())
            .await;

        // Verify queue update doesn't contain stale data
        // This is, the success interaction and the empty batch were removed
        let expected_queue = InteractionsQueue::with_items_and_batches(
            [interaction_in_progress, interaction_failed],
            [batch_non_empty],
        );
        verify_queue_update(observer.clone(), storage.clone(), expected_queue);
    }

    #[actix_rt::test]
    async fn bootstrap_fails_to_load_queue() {
        // Test the case the queue fails to load from storage.

        let observer = Arc::new(MockObserver::new());
        let storage = Arc::new(MockStorage::new_with_error());
        let _ = create_and_bootstrap(vec![], observer.clone(), storage.clone())
            .await;

        // Verify empty queue update
        let expected_queue = InteractionsQueue::with_items(vec![]);
        verify_queue_update(observer.clone(), storage.clone(), expected_queue);
    }

    #[actix_rt::test]
    async fn add_interaction_and_react_to_status_updates() {
        // Test the case an interaction is added to the queue, and after the first check its status is updated.

        let observer = Arc::new(MockObserver::new());
        let storage = Arc::new(MockStorage::new_empty());
        let sut = create_and_bootstrap(
            vec![
                // Req1: TX is submitted successfully
                submit_transaction_response(),
                // Req2: Status endpoint indicates that the TX was committed
                transaction_status_response(
                    TransactionStatusResponsePayloadStatus::CommittedSuccess,
                ),
            ],
            observer.clone(),
            storage.clone(),
        )
        .await;

        // Add the interaction to the queue
        let interaction = InteractionQueueItem::sample_queued();
        sut.add_interaction(interaction.clone()).await;

        // Verify that the queue now has 1 item whose status is `InProgress`
        let queue = sut.queue.read().await;
        assert_eq!(queue.items.len(), 1);
        assert_eq!(
            queue.items[0].status,
            InteractionQueueItemStatus::InProgress
        );
        drop(queue);

        // Verify queue update
        let expected_queue =
            InteractionsQueue::with_items(vec![interaction.clone()]);
        verify_queue_update(observer.clone(), storage.clone(), expected_queue);

        // Wait a bit for the manager to check its status
        async_std::task::sleep(Duration::from_millis(10)).await;

        // Verify that the queue has updated the item status to `Success`
        let queue = sut.queue.read().await;
        assert_eq!(queue.items[0].status, InteractionQueueItemStatus::Success);

        // Verify new queue update
        let expected_queue = InteractionsQueue::with_items(vec![interaction
            .clone()
            .with_status(InteractionQueueItemStatus::Success)]);
        verify_queue_update(observer.clone(), storage.clone(), expected_queue);
    }

    #[actix_rt::test]
    async fn add_batch_and_react_when_next_interaction_is_ready() {
        // Test the case a batch (with three interactions) is added to the queue, and how the manager
        // reacts when the first interaction is ready to be processed.

        let observer = Arc::new(MockObserver::new());
        let storage = Arc::new(MockStorage::new_empty());
        let sut =
            create_and_bootstrap(vec![], observer.clone(), storage.clone())
                .await;

        // Set up the batch with 3 interactions
        let in_half_a_second =
            Timestamp::now_utc() + Duration::from_millis(500);
        let interaction_1 = InteractionQueueItem::sample_next(in_half_a_second);
        let interaction_2 = InteractionQueueItem::sample_queued();
        let interaction_3 = InteractionQueueItem::sample_queued();
        let batch = InteractionQueueBatch::with_items([
            interaction_1.clone(),
            interaction_2.clone(),
            interaction_3.clone(),
        ]);
        sut.add_batch(batch.clone()).await;

        // Verify that the queue now has the given batch and no interactions
        let queue = sut.queue.read().await;
        assert!(queue.items.is_empty());
        assert_eq!(queue.batches, vec![batch.clone()]);
        drop(queue);

        // Verify queue update
        let expected_queue =
            InteractionsQueue::with_batches(vec![batch.clone()]);
        verify_queue_update(observer.clone(), storage.clone(), expected_queue);

        // Wait a bit for the manager to check its status
        async_std::task::sleep(Duration::from_millis(600)).await;

        // Verify that the queue has now one interaction `InProgress` and that it was removed from the batch
        let queue = sut.queue.read().await;
        assert_eq!(
            queue.items[0].status,
            InteractionQueueItemStatus::InProgress
        );
        assert_eq!(
            queue.batches[0].interactions,
            vec![
                interaction_2.with_status(InteractionQueueItemStatus::Next(
                    Timestamp::now_utc().add(Duration::from_secs(35))
                )),
                interaction_3.clone()
            ]
        );

        // Verify new queue update
        let expected_queue =
            InteractionsQueue::with_items_and_batches(
                [interaction_1
                    .with_status(InteractionQueueItemStatus::InProgress)],
                [batch.dropping_first()],
            );
        verify_queue_update(observer.clone(), storage.clone(), expected_queue);
    }
}

#[cfg(test)]
mod test_support {
    use super::*;
    use std::sync::Mutex;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = InteractionsQueueManager;

    // Helper methods

    /// Creates a new instance of the SUT whose `GatewayClient` returns the given responses and
    /// bootstraps it.
    pub(crate) async fn create_and_bootstrap(
        responses: Vec<MockNetworkingDriverResponse>,
        observer: Arc<dyn InteractionsQueueObserver>,
        storage: Arc<dyn InteractionsQueueStorage>,
    ) -> Arc<SUT> {
        let mock_networking_driver =
            MockNetworkingDriver::new_with_responses(responses);
        let gateway_client = GatewayClient::new(
            Arc::new(mock_networking_driver),
            NetworkID::Stokenet,
        );
        let sut = SUT::new(observer, storage, gateway_client);

        let _ = sut.clone().bootstrap().await;
        sut
    }

    /// Verifies that a queue with the given interactions has been notified to the observer and saved in the storage.
    pub(crate) fn verify_queue_update(
        observer: Arc<MockObserver>,
        storage: Arc<MockStorage>,
        expected_queue: InteractionsQueue,
    ) {
        // Verify that the observer has been notified with the corresponding interactions
        let observers_queue = observer.updated_queue.lock().unwrap().clone();
        assert_eq!(observers_queue, expected_queue.clone().sorted_items());

        // Verify that queue is saved in storage
        let saved_queue = storage.saved_queue.lock().unwrap().clone();
        assert_eq!(saved_queue, expected_queue);
    }

    // Mock Responses

    pub(crate) fn submit_transaction_response() -> MockNetworkingDriverResponse
    {
        let response = TransactionSubmitResponse { duplicate: false };
        MockNetworkingDriverResponse::new_success(response)
    }

    pub(crate) fn transaction_status_response(
        status: TransactionStatusResponsePayloadStatus,
    ) -> MockNetworkingDriverResponse {
        let response = match status {
            TransactionStatusResponsePayloadStatus::Unknown => TransactionStatusResponse::sample_unknown(),
            TransactionStatusResponsePayloadStatus::Pending => TransactionStatusResponse::sample_pending(),
            TransactionStatusResponsePayloadStatus::CommitPendingOutcomeUnknown => TransactionStatusResponse::sample_commit_pending_outcome_unknown(),
            TransactionStatusResponsePayloadStatus::CommittedSuccess => TransactionStatusResponse::sample_committed_success(),
            TransactionStatusResponsePayloadStatus::CommittedFailure => TransactionStatusResponse::sample_committed_failure(None),
            TransactionStatusResponsePayloadStatus::PermanentlyRejected => TransactionStatusResponse::sample_permanently_rejected(None),
            TransactionStatusResponsePayloadStatus::TemporarilyRejected => TransactionStatusResponse::sample_temporarily_rejected(),
        };
        MockNetworkingDriverResponse::new_success(response)
    }

    // Mock implementations

    pub(crate) struct MockStorage {
        saved_queue: Arc<Mutex<InteractionsQueue>>,
        stubbed_save_queue_result: Result<()>,
        stubbed_load_queue_result: Result<Option<InteractionsQueue>>,
    }

    impl MockStorage {
        pub(crate) fn new_empty() -> Self {
            Self {
                saved_queue: Arc::new(Mutex::new(InteractionsQueue::new())),
                stubbed_save_queue_result: Ok(()),
                stubbed_load_queue_result: Ok(None),
            }
        }

        pub(crate) fn new_with_queue(queue: InteractionsQueue) -> Self {
            Self {
                saved_queue: Arc::new(Mutex::new(InteractionsQueue::new())),
                stubbed_save_queue_result: Ok(()),
                stubbed_load_queue_result: Ok(Some(queue)),
            }
        }

        pub(crate) fn new_with_error() -> Self {
            Self {
                saved_queue: Arc::new(Mutex::new(InteractionsQueue::new())),
                stubbed_save_queue_result: Err(CommonError::Unknown),
                stubbed_load_queue_result: Err(CommonError::Unknown),
            }
        }
    }

    #[async_trait::async_trait]
    impl InteractionsQueueStorage for MockStorage {
        async fn save_queue(&self, queue: InteractionsQueue) -> Result<()> {
            *self.saved_queue.lock().unwrap() = queue;
            self.stubbed_save_queue_result.clone()
        }

        async fn load_queue(&self) -> Result<Option<InteractionsQueue>> {
            self.stubbed_load_queue_result.clone()
        }
    }

    pub(crate) struct MockObserver {
        updated_queue: Arc<Mutex<Vec<InteractionQueueItem>>>,
    }

    impl MockObserver {
        pub(crate) fn new() -> Self {
            Self {
                updated_queue: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait::async_trait]
    impl InteractionsQueueObserver for MockObserver {
        fn handle_update(&self, queue: Vec<InteractionQueueItem>) {
            *self.updated_queue.lock().unwrap() = queue;
        }
    }
}
