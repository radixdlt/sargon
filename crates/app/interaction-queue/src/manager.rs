use crate::prelude::*;
use async_std::task;
use std::time::Duration;

pub struct InteractionsQueueManager {
    /// The queue of interactions.
    queue: InteractionsQueue,

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
    ) -> Self {
        Self {
            queue: InteractionsQueue::new(),
            observer,
            storage,
            gateway_client,
        }
    }
}

// Exported methods (called by Hosts)
impl InteractionsQueueManager {
    /// Method to be called by hosts every time the app is started (or whenever we want)
    /// It will remove the stale data (success transactions).
    pub async fn bootstrap(&mut self) -> Result<()> {
        // Load queue from local storage.
        if let Some(queue) = self.storage.load_queue().await? {
            self.queue = queue;
        }

        // Remove stale data from it.
        self.queue.removing_stale();

        // Notify observer and save the queue to local storage.
        self.handle_queue_update().await;

        // Monitor the status of interactions in the queue.
        task::spawn(self.monitor_interactions_status());

        // Monitor the batches in the queue.
        task::spawn(self.monitor_batches());

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
    pub fn add_interaction(&self, _item: InteractionQueueItem) {
        // Calls `process_interaction(item)`
        // Call observer to notify of updated queue
    }

    pub fn add_batch(&self, _batch: InteractionQueueBatch) {
        // Adds batch to `queue.batches` and process the first interaction from its list.
        // Call observer to notify of updated queue
    }
}

// Private methods
impl InteractionsQueueManager {
    /// Notifies the observer about the updated queue and saves it to the local storage.
    async fn handle_queue_update(&self) {
        // Call observer to notify of updated queue.
        self.observer.handle_update(self.queue.sorted_items());

        // Update local storage with the cleaned queue.
        let _ = self.storage.save_queue(self.queue.clone()).await;
    }

    /// Monitors the status of interactions in the queue.
    /// Every 3 seconds, it will check the status of every interaction that is currently `InProgress`,
    /// and those that have finished have its status updated in the queue.
    async fn monitor_interactions_status(&mut self) {
        loop {
            self.check_in_progress_interactions_status().await;
            task::sleep(self.get_sleep_duration()).await;
        }
    }

    /// Monitor the batches in the queue, processing the interactions that are ready to be processed.
    async fn monitor_batches(&mut self) {
        loop {
            self.check_batch_ready_interactions().await;
            task::sleep(self.get_sleep_duration()).await;
        }
    }

    fn get_sleep_duration(&self) -> Duration {
        #[cfg(test)]
        let result = Duration::from_millis(INTERACTION_QUEUE_POLLING_INTERVAL); // make it faster for tests
        #[cfg(not(test))]
        let result = Duration::from_secs(INTERACTION_QUEUE_POLLING_INTERVAL);

        result
    }

    /// Checks the status of every interaction that is currently `InProgress`.
    /// Every interaction which has finished will be updated in the queue.
    async fn check_in_progress_interactions_status(&mut self) {
        let items = self
            .queue
            .items
            .iter()
            .filter(|item| {
                item.status == InteractionQueueItemStatus::InProgress
            })
            .cloned()
            .collect::<Vec<_>>();

        // Note: Once Gateway supports checking the status of multiple interactions at the same time,
        // this loop will be replaced with one single call to the Gateway.
        for item in items {
            let updated_item =
                self.check_in_progress_interaction_status(item).await;
            if let Some(updated_item) = updated_item {
                self.queue.replace_interaction(updated_item.clone());
            }
        }

        // Notify observer
        let _ = self.handle_queue_update().await;
    }

    /// Check the status of a single interaction that is `InProgress`.
    /// Returns the updated interaction if it has finished, otherwise returns `None`.
    async fn check_in_progress_interaction_status(
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
                    Err(_) => {
                        return None;
                    }
                };
                match response
                    .known_payloads
                    .first()
                    .and_then(|payload| payload.payload_status.clone())
                {
                    Some(payload_status) => {
                        let status =
                            InteractionQueueItemStatus::from(payload_status);
                        if status != InteractionQueueItemStatus::InProgress {
                            updated_interaction.status = status;
                            Some(updated_interaction)
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            }
            InteractionQueueItemKind::PreAuthorization(_) => {
                panic!("Not implemented yet");
            }
        }
    }

    /// Loop over the batches and call `batch.get_first_if_ready()` for each of them.
    /// If they return an interaction, call `process_interaction()` with it.
    /// Call observer to notify of updated queue.
    async fn check_batch_ready_interactions(&mut self) {
        let mut ready_interactions = Vec::new();

        for batch in self.queue.batches.iter_mut() {
            if let Some(interaction) = batch.get_first_if_ready() {
                ready_interactions.push(interaction);
            }
        }

        for interaction in ready_interactions {
            self.process_interaction(interaction).await;
        }

        self.handle_queue_update().await;
    }

    async fn process_interaction(&mut self, item: InteractionQueueItem) {
        // Set interaction status to `InProgress`
        let mut item = item.clone();
        item.status = InteractionQueueItemStatus::InProgress;

        // Append it to the queue
        self.queue.add_interaction(item.clone());

        match item.kind {
            InteractionQueueItemKind::Transaction(transaction) => {
                // If it is a Transaction, submit it to network.
                let request = TransactionSubmitRequest {
                    notarized_transaction_hex: transaction
                        .notarized_transaction_hex,
                };
                let _ = self
                    .gateway_client
                    .submit_transaction(request, transaction.transaction_id)
                    .await;
            }
            InteractionQueueItemKind::PreAuthorization(_) => {}
        }
    }
}
