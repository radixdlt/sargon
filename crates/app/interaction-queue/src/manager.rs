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

            drop(locked_queue);
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
                //queue.replace_interaction(updated_item);
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

    #[allow(clippy::upper_case_acronyms)]
    type SUT = InteractionsQueueManager;

    struct MockStorage {
        stubbed_save_queue_result: Result<()>,
        stubbed_load_queue_result: Result<Option<InteractionsQueue>>,
    }

    impl MockStorage {
        fn new_empty() -> Self {
            Self {
                stubbed_save_queue_result: Ok(()),
                stubbed_load_queue_result: Ok(None),
            }
        }

        fn new_with_queue(queue: InteractionsQueue) -> Self {
            Self {
                stubbed_save_queue_result: Ok(()),
                stubbed_load_queue_result: Ok(Some(queue)),
            }
        }

        fn new_with_error() -> Self {
            Self {
                stubbed_save_queue_result: Err(CommonError::Unknown),
                stubbed_load_queue_result: Err(CommonError::Unknown),
            }
        }
    }

    #[async_trait::async_trait]
    impl InteractionsQueueStorage for MockStorage {
        async fn save_queue(&self, _queue: InteractionsQueue) -> Result<()> {
            self.stubbed_save_queue_result.clone()
        }

        async fn load_queue(&self) -> Result<Option<InteractionsQueue>> {
            self.stubbed_load_queue_result.clone()
        }
    }

    struct MockObserver {
        handlded_queue: Arc<RwLock<Vec<InteractionQueueItem>>>,
    }

    impl MockObserver {
        fn new() -> Self {
            Self {
                handlded_queue: Arc::new(RwLock::new(Vec::new())),
            }
        }
    }

    #[async_trait::async_trait]
    impl InteractionsQueueObserver for MockObserver {
        fn handle_update(&self, queue: Vec<InteractionQueueItem>) {
            // TODO: Implement
        }
    }

    #[actix_rt::test]
    async fn add_interaction() {
        let sut = create_and_bootstrap(vec![
            // Req1: TX is submitted successfully
            submit_transaction_response(),
            // Req2: Status endpoint indicates that the TX was committed
            transaction_status_response(
                TransactionStatusResponsePayloadStatus::CommittedSuccess,
            ),
        ])
        .await;

        // Add the interaction to the queue
        let interaction = InteractionQueueItem::new_in_progress(
            false,
            InteractionQueueItemKind::sample(),
        );
        sut.add_interaction(interaction).await;

        // Verify that the queue now has 1 item whose status is `InProgress`
        let queue = sut.queue.read().await;
        assert_eq!(queue.items.len(), 1);
        assert_eq!(
            queue.items[0].status,
            InteractionQueueItemStatus::InProgress
        );
        drop(queue);

        // Wait a bit for the manager to check its status
        async_std::task::sleep(Duration::from_millis(100)).await;

        // Verify that the queue now has the item set to `Success`
        let queue = sut.queue.read().await;
        assert_eq!(queue.items[0].status, InteractionQueueItemStatus::Success);
    }

    async fn create_and_bootstrap(
        responses: Vec<MockNetworkingDriverResponse>,
    ) -> Arc<SUT> {
        let observer = Arc::new(MockObserver::new());
        let storage = Arc::new(MockStorage::new_empty());
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

    // Mock Responses

    fn submit_transaction_response() -> MockNetworkingDriverResponse {
        let response = TransactionSubmitResponse { duplicate: false };
        MockNetworkingDriverResponse::new_success(response)
    }

    fn transaction_status_response(
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
}
