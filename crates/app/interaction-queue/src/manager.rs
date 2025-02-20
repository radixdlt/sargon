use crate::prelude::*;
use async_std::task;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct InteractionsQueueManager {
    /// The queue of interactions.
    queue: Mutex<InteractionsQueue>,

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
            queue: Mutex::new(InteractionsQueue::new()),
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
            let mut locked_queue = self.queue.lock().unwrap();
            *locked_queue = queue;

            // Remove stale data from queue.
            locked_queue.removing_stale();
        }

        // Notify observer and save the queue to local storage.
        self.handle_queue_update().await;

        // Monitor the status of interactions in the queue.
        let self_clone = self.clone();
        thread::spawn(move || {
            task::block_on(self_clone.monitor_interactions_status());
        });

        // Monitor the batches in the queue.
        let self_clone = self.clone();
        thread::spawn(move || {
            task::block_on(self_clone.monitor_batches());
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

    pub async fn add_batch(&self, _batch: InteractionQueueBatch) {
        self.queue.lock().unwrap().add_batch(_batch);
        self.handle_queue_update().await;
    }
}

// Private methods
impl InteractionsQueueManager {
    /// Notifies the observer about the updated queue and saves it to the local storage.
    // Note: we probably want this method to not await until the storage is saved
    async fn handle_queue_update(&self) {
        let queue = self.queue.lock().unwrap();
        self.observer.handle_update(queue.sorted_items());
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
        let mut queue = self.queue.lock().unwrap();
        let items: Vec<_> = queue
            .items
            .iter()
            .filter(|item| {
                item.status == InteractionQueueItemStatus::InProgress
            })
            .cloned()
            .collect();

        // Note: Once Gateway supports checking the status of multiple interactions at the same time,
        // this loop will be replaced with one single call to the Gateway.
        // Related discussion: https://rdxworks.slack.com/archives/C071XGF4G6M/p1740041597061639
        for item in items {
            if let Some(updated_item) = self.get_interaction_status(item).await
            {
                queue.replace_interaction(updated_item);
            }
        }

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
        let mut queue = self.queue.lock().unwrap();
        let ready_interactions: Vec<_> = queue
            .batches
            .iter_mut()
            .filter_map(|batch| batch.get_first_if_ready())
            .collect();

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

        self.queue.lock().unwrap().add_interaction(item.clone());

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
