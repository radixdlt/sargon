use crate::prelude::*;

/// Trait for observing home cards updates.
/// Defines a method for handling updates to home cards.
#[uniffi::export(with_foreign)]
pub trait HomeCardsObserver: Send + Sync {
    /// Handles updates to the home cards.
    fn handle_cards_update(&self, cards: HomeCards);
}
