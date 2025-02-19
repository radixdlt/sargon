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

        // Set up timer to perform the corresponding checks every 3 seconds
        self.set_polling_timer().await;

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

    /// Sets up a timer to perform the corresponding checks every 3 seconds. These are:
    /// - Poll the status of every interaction that is `InProgress`.
    /// - Check if any batch has an interaction ready to be processed.
    async fn set_polling_timer(&mut self) {
        #[cfg(test)]
        let sleep_duration =
            Duration::from_millis(INTERACTION_QUEUE_POLLING_INTERVAL); // make it faster for tests
        #[cfg(not(test))]
        let sleep_duration =
            Duration::from_secs(INTERACTION_QUEUE_POLLING_INTERVAL);

        loop {
            task::spawn(self.check_in_progress_interactions_status());
            // task::spawn(self.check_batch_ready_interactions());
            task::sleep(sleep_duration).await;
        }
    }

    /// Checks the status of every interaction that is currently `InProgress`.
    /// Every interaction which has finished will be updated in the queue.
    async fn check_in_progress_interactions_status(&mut self) {
        let items =  self.queue.items
            .iter()
            .filter(|item| item.status == InteractionQueueItemStatus::InProgress)
            .cloned()
            .collect::<Vec<_>>();

        for item in items {
            let updated_item = self.check_in_progress_interaction_status(item).await;
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
                    None => {
                        None
                    }
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
    async fn check_batch_ready_interactions(&self) {}

    async fn poll_status(&self) {
        // Poll the status of every interaction that is `InProgress`.
        // For each of them that has finished and has a `batchId` set, handle the next interaction of its batch.
        // Call observer to notify of updated queue

        // Also, loop over the batches and call `batch.get_first_if_ready()` for each of them.
        // If they return an interaction, call `process_interaction()` with it.
    }

    fn process_interaction(&self, _item: InteractionQueueItem) {
        // Set interaction status to `InProgress`
        // If it is a Transaction, submit it to network.
    }
}
