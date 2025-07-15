use crate::prelude::*;
use sargon::BagOfBytes as InternalBagOfBytes;
use sargon::HomeCardsStorage as InternalHomeCardsStorage;
use sargon::Result as InternalResult;

/// A trait for storing and loading home cards.
/// Defines asynchronous methods for saving and loading encoded home cards.
#[uniffi::export(with_foreign)]
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

pub struct HomeCardsStorageAdapter {
    pub wrapped: Arc<dyn HomeCardsStorage>,
}

#[async_trait::async_trait]
impl InternalHomeCardsStorage for HomeCardsStorageAdapter {
    async fn save_cards(
        &self,
        encoded_cards: InternalBagOfBytes,
    ) -> InternalResult<()> {
        self.wrapped
            .save_cards(encoded_cards.into())
            .await
            .into_internal_result()
    }

    async fn load_cards(&self) -> InternalResult<Option<InternalBagOfBytes>> {
        self.wrapped.load_cards().await.into_internal_result()
    }

    async fn save_dismissed_cards(
        &self,
        encoded_cards: InternalBagOfBytes,
    ) -> InternalResult<()> {
        self.wrapped
            .save_dismissed_cards(encoded_cards.into())
            .await
            .into_internal_result()
    }

    async fn load_dismissed_cards(
        &self,
    ) -> InternalResult<Option<InternalBagOfBytes>> {
        self.wrapped
            .load_dismissed_cards()
            .await
            .into_internal_result()
    }
}
