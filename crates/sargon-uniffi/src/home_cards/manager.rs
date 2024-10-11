use crate::prelude::*;

use sargon::HomeCardsManager as InternalHomeCardsManager;

/// Manages the home cards by handling storage, parsing, and updating operations.
/// Call `bootstrap` before invoking any other public functions.
#[derive(uniffi::Object)]
pub struct HomeCardsManager {
    pub wrapped: Arc<InternalHomeCardsManager>,
}

#[uniffi::export]
impl HomeCardsManager {
    #[uniffi::constructor]
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        network_id: NetworkID,
        cards_storage: Arc<dyn HomeCardsStorage>,
        observer: Arc<dyn HomeCardsObserver>,
    ) -> Self {
        let wrapped = InternalHomeCardsManager::new(
            Arc::new(NetworkingDriverAdapter {
                wrapped: networking_driver,
            }),
            network_id.into(),
            Arc::new(HomeCardsStorageAdapter {
                wrapped: cards_storage,
            }),
            Arc::new(HomeCardsObserverAdapter { wrapped: observer }),
        );
        Self {
            wrapped: Arc::new(wrapped),
        }
    }
}

#[uniffi::export]
impl HomeCardsManager {
    /// Initializes `HomeCards` by loading from storage.
    /// This function should be called before invoking any other public functions.
    /// Notifies `HomeCardsObserver`.
    #[uniffi::method]
    pub async fn bootstrap(&self) -> Result<()> {
        map_result_from_internal(self.wrapped.bootstrap().await)
    }

    /// Initializes and saves to storage default `HomeCards`.
    /// Marks the wallet creation and populates the set of cards required for a new wallet.
    /// Notifies `HomeCardsObserver`.
    pub async fn wallet_created(&self) -> Result<()> {
        map_result_from_internal(self.wrapped.wallet_created().await)
    }

    /// Handles a deferred deep link by parsing it and saving the generated `HomeCards` to `HomeCardsStorage`.
    /// `HomeCard::ContinueRadQuest` if found in the link parsing result, replaces `HomeCard::StartRadQuest`.
    /// Notifies `HomeCardsObserver`.
    #[uniffi::method]
    pub async fn deferred_deep_link_received(
        &self,
        encoded_value: String,
    ) -> Result<()> {
        map_result_from_internal(
            self.wrapped
                .deferred_deep_link_received(encoded_value)
                .await,
        )
    }

    /// Marks the wallet restoration.
    /// Ensures only the expected `HomeCards` remain in `HomeCardsStorage` - currently none.
    /// Notifies `HomeCardsObserver`.
    #[uniffi::method]
    pub async fn wallet_restored(&self) -> Result<()> {
        map_result_from_internal(self.wrapped.wallet_restored().await)
    }

    /// Dismisses a specified `HomeCard` by removing it from `HomeCardsStorage`.
    /// Notifies `HomeCardsObserver`.
    #[uniffi::method]
    pub async fn card_dismissed(&self, card: HomeCard) -> Result<()> {
        map_result_from_internal(self.wrapped.card_dismissed(card.into()).await)
    }

    /// Clears the home cards from the `HomeCardsStorage`.
    /// Notifies `HomeCardsObserver`.
    #[uniffi::method]
    pub async fn wallet_reset(&self) -> Result<()> {
        map_result_from_internal(self.wrapped.wallet_reset().await)
    }
}
