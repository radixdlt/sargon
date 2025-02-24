use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// A struct representing all the interactions that were dispatched, or are waiting to be dispatched.
pub struct InteractionQueue {
    /// Interactions that were already dispatched.
    /// Their status will always be `InProgress`, `Success` or `Failure`.
    pub items: IndexSet<InteractionQueueItem>,

    /// Batches of interactions that are waiting to be dispatched.
    pub batches: Vec<InteractionQueueBatch>,
}

impl InteractionQueue {
    pub fn new() -> Self {
        Self {
            items: IndexSet::new(),
            batches: Vec::new(),
        }
    }
}

impl InteractionQueue {
    /// Returns every interaction in the queue, sorted by its status.
    pub fn sorted_items(&self) -> Vec<InteractionQueueItem> {
        let mut all_items: Vec<InteractionQueueItem> = self
            .items
            .iter()
            .cloned()
            .chain(
                self.batches
                    .iter()
                    .flat_map(|batch| batch.interactions.clone()),
            )
            .collect();

        all_items.sort_by_key(|item| item.status.clone());
        all_items
    }

    /// Removes every successful interaction from `items`, and clears `batches` that have no remaining interactions.
    pub fn removing_stale(&mut self) {
        self.items
            .retain(|item| item.status != InteractionQueueItemStatus::Success);
        self.batches.retain(|batch| !batch.interactions.is_empty());
    }

    /// Adds as interaction to the queue.
    pub fn add_interaction(&mut self, interaction: InteractionQueueItem) {
        assert_eq!(
            interaction.status,
            InteractionQueueItemStatus::InProgress,
            "Should never add an interaction whose status isn't InProgress"
        );
        self.items.insert(interaction);
    }

    /// Replaces an interaction in the queue with an updated version of it.
    pub fn replace_interaction(&mut self, interaction: InteractionQueueItem) {
        self.items.replace(interaction);
    }

    /// Removes an interaction from the queue.
    pub fn remove_interaction(&mut self, interaction: InteractionQueueItem) {
        assert!(matches!(
            interaction.status,
            InteractionQueueItemStatus::Success
                | InteractionQueueItemStatus::Failure(_)
        ), "Should never remove an interaction whose status isn't Success or Failure");
        self.items.shift_remove(&interaction);
    }

    /// Cancels a pending interaction.
    ///
    /// Attempts to find the given interaction in any batch, and if found, it is removed from such batch
    pub fn cancel_interaction(&mut self, interaction: InteractionQueueItem) {
        assert!(matches!(
            interaction.status,
            InteractionQueueItemStatus::Next(_)
                | InteractionQueueItemStatus::Queued
        ), "Should never cancel an interaction whose status isn't Next or Queued");
        for batch in self.batches.iter_mut() {
            if let Some(index) =
                batch.interactions.iter().position(|i| i == &interaction)
            {
                batch.interactions.remove(index);
                return;
            }
        }
    }

    /// Adds a new batch to the queue.
    pub fn add_batch(&mut self, batch: InteractionQueueBatch) {
        self.batches.push(batch);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = InteractionQueue;

    #[test]
    fn sorted_items() {
        // Set up the Queue with 3 interactions among items and 4 interactions inside batches
        let interaction_in_progress =
            InteractionQueueItem::sample_in_progress();
        let interaction_success = InteractionQueueItem::sample_success();
        let interaction_failed = InteractionQueueItem::sample_failed();

        let interaction_b1_next = InteractionQueueItem::sample_next();
        let interaction_b1_queued = InteractionQueueItem::sample_queued();
        let batch_1 = InteractionQueueBatch::with_items([
            interaction_b1_next.clone(),
            interaction_b1_queued.clone(),
        ]);

        let interaction_b2_next = InteractionQueueItem::sample_next();
        let interaction_b2_queued = InteractionQueueItem::sample_queued();
        let batch_2 = InteractionQueueBatch::with_items([
            interaction_b2_next.clone(),
            interaction_b2_queued.clone(),
        ]);

        let sut = SUT::with_items_and_batches(
            [
                interaction_in_progress.clone(),
                interaction_success.clone(),
                interaction_failed.clone(),
            ],
            [batch_1.clone(), batch_2.clone()],
        );

        assert_eq!(
            sut.sorted_items(),
            vec![
                // First failed
                interaction_failed,
                // Then success
                interaction_success,
                // Then in progress
                interaction_in_progress,
                // Then next
                interaction_b1_next,
                interaction_b2_next,
                // Then queued
                interaction_b1_queued,
                interaction_b2_queued,
            ]
        );
    }

    #[test]
    fn removing_stale() {
        // Set up the Queue with 3 interactions among items and 1 empty batch and 1 batch with one interaction
        let interaction_in_progress =
            InteractionQueueItem::sample_in_progress();
        let interaction_success = InteractionQueueItem::sample_success();
        let interaction_failed = InteractionQueueItem::sample_failed();

        let interaction_b1_next = InteractionQueueItem::sample_next();
        let interaction_b1_queued = InteractionQueueItem::sample_queued();
        let batch_1 = InteractionQueueBatch::with_items([
            interaction_b1_next.clone(),
            interaction_b1_queued.clone(),
        ]);

        let batch_2 = InteractionQueueBatch::with_items([]);

        let mut sut = SUT::with_items_and_batches(
            [
                interaction_in_progress.clone(),
                interaction_success.clone(),
                interaction_failed.clone(),
            ],
            [batch_1.clone(), batch_2.clone()],
        );

        sut.removing_stale();

        assert_eq!(
            sut.items,
            IndexSet::from([interaction_in_progress, interaction_failed])
        );
        assert_eq!(sut.batches, [batch_1]);
    }

    #[test]
    fn add_interaction_valid() {
        let mut sut = SUT::new();

        let interaction = InteractionQueueItem::sample_in_progress();
        sut.add_interaction(interaction.clone());

        assert_eq!(sut.items, IndexSet::from([interaction]));
    }

    #[test]
    #[should_panic(
        expected = "Should never add an interaction whose status isn't InProgress"
    )]
    fn add_interaction_invalid() {
        let mut sut = SUT::new();

        let interaction = InteractionQueueItem::sample_queued();
        sut.add_interaction(interaction);
    }

    #[test]
    fn replace_interaction() {
        let mut sut = SUT::new();

        let interaction = InteractionQueueItem::sample_in_progress();
        sut.add_interaction(interaction.clone());
        assert_eq!(sut.items[0].status, InteractionQueueItemStatus::InProgress);

        let updated_interaction = InteractionQueueItem::sample_success();
        sut.replace_interaction(updated_interaction.clone());

        assert_eq!(sut.items[0].status, InteractionQueueItemStatus::Success);
    }

    #[test]
    fn remove_interaction_valid() {
        let interaction_in_progress =
            InteractionQueueItem::sample_in_progress();
        let interaction_success = InteractionQueueItem::sample_success();
        let interaction_failed = InteractionQueueItem::sample_failed();
        let mut sut = SUT::with_items(vec![
            interaction_in_progress.clone(),
            interaction_success.clone(),
            interaction_failed.clone(),
        ]);

        sut.remove_interaction(interaction_success.clone());
        assert_eq!(
            sut.items,
            IndexSet::from([
                interaction_in_progress.clone(),
                interaction_failed.clone()
            ])
        );

        sut.remove_interaction(interaction_failed);
        assert_eq!(sut.items, IndexSet::from([interaction_in_progress]));
    }

    #[test]
    #[should_panic(
        expected = "Should never remove an interaction whose status isn't Success or Failure"
    )]
    fn remove_interaction_invalid() {
        let interaction_in_progress =
            InteractionQueueItem::sample_in_progress();
        let mut sut = SUT::with_items(vec![interaction_in_progress.clone()]);

        sut.remove_interaction(interaction_in_progress);
    }

    #[test]
    fn cancel_interaction_valid() {
        let interaction_b1_next = InteractionQueueItem::sample_next();
        let interaction_b1_queued = InteractionQueueItem::sample_queued();
        let batch_1 = InteractionQueueBatch::with_items([
            interaction_b1_next.clone(),
            interaction_b1_queued.clone(),
        ]);

        let interaction_b2_next = InteractionQueueItem::sample_next();
        let interaction_b2_queued = InteractionQueueItem::sample_queued();
        let batch_2 = InteractionQueueBatch::with_items([
            interaction_b2_next.clone(),
            interaction_b2_queued.clone(),
        ]);

        let mut sut = SUT::with_batches(vec![batch_1.clone(), batch_2.clone()]);
        sut.cancel_interaction(interaction_b1_next.clone());

        assert_eq!(sut.batches.len(), 2);
        assert_eq!(
            sut.batches[0].interactions,
            vec![interaction_b1_queued.clone()]
        );
        assert_eq!(
            sut.batches[1].interactions,
            vec![interaction_b2_next.clone(), interaction_b2_queued.clone()]
        );

        sut.cancel_interaction(interaction_b2_queued.clone());
        assert_eq!(sut.batches.len(), 2);
        assert_eq!(sut.batches[0].interactions, vec![interaction_b1_queued]);
        assert_eq!(sut.batches[1].interactions, vec![interaction_b2_next]);
    }

    #[test]
    #[should_panic(
        expected = "Should never cancel an interaction whose status isn't Next or Queued"
    )]
    fn cancel_interaction_invalid() {
        let interaction_in_progress =
            InteractionQueueItem::sample_in_progress();
        let mut sut = SUT::with_items(vec![interaction_in_progress.clone()]);

        sut.cancel_interaction(interaction_in_progress);
    }

    #[test]
    fn add_batch() {
        let mut sut = SUT::new();

        let batch = InteractionQueueBatch::with_items([
            InteractionQueueItem::sample_next(),
            InteractionQueueItem::sample_queued(),
        ]);
        sut.add_batch(batch.clone());

        assert_eq!(sut.batches, vec![batch]);
    }
}
