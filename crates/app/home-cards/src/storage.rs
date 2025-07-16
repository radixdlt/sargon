use crate::prelude::*;

/// A trait for storing and loading home cards.
/// Defines asynchronous methods for saving and loading encoded home cards.
#[async_trait::async_trait]
pub trait HomeCardsStorage: Send + Sync {
    /// Saves the encoded home cards to the storage.
    async fn save_cards(&self, encoded_cards: BagOfBytes) -> Result<()>;

    /// Loads the encoded home cards from the storage.
    async fn load_cards(&self) -> Result<Option<BagOfBytes>>;

    /// Saves the encoded dismissed home cards to the storage.
    async fn save_dismissed_cards(
        &self,
        encoded_cards: BagOfBytes,
    ) -> Result<()>;

    /// Loads the encoded dismissed home cards from the storage.
    async fn load_dismissed_cards(&self) -> Result<Option<BagOfBytes>>;
}
