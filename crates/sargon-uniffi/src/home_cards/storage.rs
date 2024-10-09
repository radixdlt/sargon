use crate::prelude::*;
use sargon::HomeCardsStorage as InternalHomeCardsStorage;
use sargon::BagOfBytes as InternalBagOfBytes;

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

#[derive(Debug)]
pub struct HomeCardsStorageAdapter {
    pub wrapped: Arc<dyn HomeCardsStorage>,
}

#[async_trait::async_trait]
impl InternalHomeCardsStorage for HomeCardsStorageAdapter {
    async fn save_cards(&self, encoded_cards: InternalBagOfBytes) -> Result<()> {
        map_result_to_internal(self.wrapped.save_cards(encoded_cards.into()).await)
    }

    async fn load_cards(&self) -> Result<Option<InternalBagOfBytes>> {
        map_result_to_internal_optional(self.wrapped.load_cards().await)
    }
}
