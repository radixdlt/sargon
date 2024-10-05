use crate::prelude::*;
use sargon::HomeCardsObserver as InternalHomeCardsObserver;
use sargon::HomeCards as InternalHomeCards;

/// Trait for observing home cards updates.
/// Defines a method for handling updates to home cards.
#[uniffi::export(with_foreign)]
pub trait HomeCardsObserver: Send + Sync {
    /// Handles updates to the home cards.
    fn handle_cards_update(&self, cards: HomeCards);
}

#[derive(Debug)]
pub struct HomeCardsObserverAdapter {
    pub wrapped: Arc<dyn HomeCardsObserver>,
}

impl InternalHomeCardsObserver for HomeCardsObserverAdapter {
    fn handle_cards_update(&self, cards: InternalHomeCards) {
        self.wrapped.handle_cards_update(cards.into())
    }
}
