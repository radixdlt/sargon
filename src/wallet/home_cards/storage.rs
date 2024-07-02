use crate::prelude::*;

/// A trait for storing and loading home cards.
/// Defines asynchronous methods for saving and loading encoded home cards.
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HomeCardsStorage: Send + Sync {
    /// Saves the encoded home cards to the storage.
    async fn save_cards(&self, encoded_cards: BagOfBytes) -> Result<()>;

    /// Loads the encoded home cards from the storage.
    async fn load_cards(&self) -> Result<Option<BagOfBytes>>;
}
