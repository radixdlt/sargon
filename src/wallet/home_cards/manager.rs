use crate::prelude::*;
use std::sync::{RwLock, RwLockWriteGuard};

/// Manages the home cards by handling storage, parsing, and updating operations.
#[derive(uniffi::Object)]
pub struct HomeCardsManager {
    /// Parser for handling deferred deep links.
    parser: Arc<dyn DeferredDeepLinkParser>,
    /// Storage for saving and loading home cards.
    cards_storage: Arc<dyn HomeCardsStorage>,
    /// Observer to handle updates to the home cards.
    observer: Arc<dyn HomeCardsObserver>,
    /// In-memory storage of the current home cards.
    cards: RwLock<HomeCards>,
}

impl HomeCardsManager {
    pub fn init(
        parser: Arc<dyn DeferredDeepLinkParser>,
        cards_storage: Arc<dyn HomeCardsStorage>,
        observer: Arc<dyn HomeCardsObserver>,
    ) -> Self {
        Self {
            parser,
            cards_storage,
            observer,
            cards: RwLock::new(HomeCards::new()),
        }
    }
}

#[uniffi::export]
impl HomeCardsManager {
    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
        network_id: NetworkID,
        cards_storage: Arc<dyn HomeCardsStorage>,
        observer: Arc<dyn HomeCardsObserver>,
    ) -> Self {
        Self::init(
            Arc::new(Parser::new(GatewayClient::new(
                network_antenna,
                network_id,
            ))),
            cards_storage,
            observer,
        )
    }
}

#[uniffi::export]
impl HomeCardsManager {
    /// Initializes home cards by loading from storage.
    /// If there were no stored home cards, initializes and saves to storage default cards.
    /// Notifies `HomeCardsObserver`.
    #[uniffi::method]
    pub async fn init_cards(&self) -> Result<()> {
        match self.load_cards().await {
            Ok(home_cards) => {
                _ = self
                    .update_cards(|write_guard| {
                        self.insert_cards(write_guard, home_cards)
                    })
                    .await;
                Ok(())
            }
            Err(CommonError::FailedAccessingHomeCards) => {
                let default_cards = HomeCards::from_iter([HomeCard::Connector]);
                let updated_cards = self
                    .update_cards(|write_guard| {
                        self.insert_cards(write_guard, default_cards)
                    })
                    .await?;
                self.save_cards(updated_cards).await
            }
            Err(e) => Err(e),
        }
    }

    /// Handles a deferred deep link by parsing it, saving the generated home cards to storage and notifying the observer.
    /// Notifies `HomeCardsObserver`.
    #[uniffi::method]
    pub async fn handle_deferred_deep_link(
        &self,
        encoded_value: String,
    ) -> Result<()> {
        let deep_link_cards = self.parser.parse(encoded_value).await?;
        let updated_cards = self
            .update_cards(|write_guard| {
                self.insert_cards(write_guard, deep_link_cards)
            })
            .await?;
        self.save_cards(updated_cards).await
    }

    /// Removes a specified home card, save the updated home cards to storage and notifies the observer.
    /// Notifies `HomeCardsObserver`.
    #[uniffi::method]
    pub async fn remove_card(&self, card: HomeCard) -> Result<()> {
        let updated_cards = self
            .update_cards(|write_guard| {
                write_guard.remove_id(&card);
            })
            .await?;
        self.save_cards(updated_cards).await
    }
}

impl HomeCardsManager {
    /// Updates home cards both in-memory and in storage after applying `f` function, then notifies the observer.
    async fn update_cards<F>(&self, f: F) -> Result<HomeCards>
    where
        F: FnOnce(&mut RwLockWriteGuard<HomeCards>),
    {
        let mut write_guard = self
            .cards
            .write()
            .map_err(|_| CommonError::FailedUpdatingHomeCards)?;

        f(&mut write_guard);

        let updated_cards = write_guard.clone();
        updated_cards.sort();

        self.observer.handle_cards_update(updated_cards.clone());
        Ok(updated_cards)
    }

    fn insert_cards(
        &self,
        write_guard: &mut RwLockWriteGuard<HomeCards>,
        cards: HomeCards,
    ) {
        cards.into_iter().for_each(|card| {
            if write_guard.try_insert_unique(card).is_ok() {
                debug!("Home card inserted");
            } else {
                debug!("Home card insert failed");
            }
        })
    }
}

impl HomeCardsManager {
    /// Loads the home cards from storage.
    async fn load_cards(&self) -> Result<HomeCards> {
        let cards_bytes = self
            .cards_storage
            .load_cards()
            .await?
            .ok_or(CommonError::FailedAccessingHomeCards)?;
        deserialize_from_slice(cards_bytes.as_slice())
    }

    /// Saves the home cards to storage.
    async fn save_cards(&self, cards: HomeCards) -> Result<()> {
        let bytes = serialize(&cards)?;
        self.cards_storage.save_cards(bytes.into()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct MockHomeCardsStorage {
        stubbed_load_cards_result: Result<Option<BagOfBytes>>,
    }

    impl MockHomeCardsStorage {
        fn new_empty() -> Self {
            Self {
                stubbed_load_cards_result: Self::encode_cards(HomeCards::new()),
            }
        }

        fn new_with_stored_cards(cards: HomeCards) -> Self {
            Self {
                stubbed_load_cards_result: Self::encode_cards(cards),
            }
        }

        fn new_with_error() -> Self {
            Self {
                stubbed_load_cards_result: Err(
                    CommonError::FailedAccessingHomeCards,
                ),
            }
        }

        fn encode_cards(cards: HomeCards) -> Result<Option<BagOfBytes>> {
            serialize(&cards).map(|cards| Some(cards.into()))
        }
    }

    #[async_trait::async_trait]
    impl HomeCardsStorage for MockHomeCardsStorage {
        async fn save_cards(&self, encoded_cards: BagOfBytes) -> Result<()> {
            let _: HomeCards =
                deserialize_from_slice(encoded_cards.as_slice())?;
            Ok(())
        }

        async fn load_cards(&self) -> Result<Option<BagOfBytes>> {
            self.stubbed_load_cards_result.clone()
        }
    }

    struct MockHomeCardsObserver {
        handled_cards: Arc<Mutex<Option<HomeCards>>>,
    }

    impl MockHomeCardsObserver {
        fn new() -> Self {
            Self {
                handled_cards: Arc::new(Mutex::new(None)),
            }
        }
    }

    impl HomeCardsObserver for MockHomeCardsObserver {
        fn handle_cards_update(&self, cards: HomeCards) {
            *self.handled_cards.lock().unwrap() = Some(cards);
        }
    }

    struct MockDeferredDeepLinkParser {
        stubbed_result: Result<HomeCards>,
    }

    impl MockDeferredDeepLinkParser {
        fn new(stubbed_result: Result<HomeCards>) -> Self {
            Self { stubbed_result }
        }

        fn succeeding(cards: HomeCards) -> Self {
            Self::new(Ok(cards))
        }
    }

    #[async_trait::async_trait]
    impl DeferredDeepLinkParser for MockDeferredDeepLinkParser {
        async fn parse(&self, _: String) -> Result<HomeCards> {
            self.stubbed_result.clone()
        }
    }

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HomeCardsManager;

    #[actix_rt::test]
    async fn test_init_cards_with_stored_cards() {
        let expected_cards = HomeCards::from_iter(vec![HomeCard::Connector]);
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockAntenna::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_stored_cards(
                expected_cards.clone(),
            )),
            observer.clone(),
        );

        manager.init_cards().await.unwrap();
        let handled_cards = observer.handled_cards.lock().unwrap().clone();

        pretty_assertions::assert_eq!(handled_cards, Some(expected_cards));
    }

    #[actix_rt::test]
    async fn test_init_cards_without_stored_cards() {
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockAntenna::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_empty()),
            observer.clone(),
        );
        let expected_cards = HomeCards::new();

        manager.init_cards().await.unwrap();

        let handled_cards = observer.handled_cards.lock().unwrap().clone();
        pretty_assertions::assert_eq!(handled_cards, Some(expected_cards));
    }

    #[actix_rt::test]
    async fn test_init_cards_when_failed_accessing_storage() {
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockAntenna::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_error()),
            observer.clone(),
        );
        let expected_cards = HomeCards::from_iter(vec![HomeCard::Connector]);

        manager.init_cards().await.unwrap();

        let handled_cards = observer.handled_cards.lock().unwrap().clone();
        pretty_assertions::assert_eq!(handled_cards, Some(expected_cards));
    }

    #[actix_rt::test]
    async fn test_handle_deferred_deep_link() {
        let observer = Arc::new(MockHomeCardsObserver::new());
        let stored_cards = HomeCards::from_iter(vec![HomeCard::Connector]);
        let deep_link_cards = HomeCards::from_iter(vec![
            HomeCard::Dapp { icon_url: None },
            HomeCard::ContinueRadQuest,
        ]);
        let manager = SUT::init(
            Arc::new(MockDeferredDeepLinkParser::succeeding(
                deep_link_cards.clone(),
            )),
            Arc::new(MockHomeCardsStorage::new_with_stored_cards(
                stored_cards.clone(),
            )),
            observer.clone(),
        );

        manager.init_cards().await.unwrap();
        manager
            .handle_deferred_deep_link("encoded_value".to_string())
            .await
            .unwrap();

        let expected_cards = HomeCards::from_iter(vec![
            HomeCard::ContinueRadQuest,
            HomeCard::Dapp { icon_url: None },
            HomeCard::Connector,
        ]);
        let handled_cards = observer.handled_cards.lock().unwrap().clone();
        pretty_assertions::assert_eq!(handled_cards, Some(expected_cards));
    }

    #[actix_rt::test]
    async fn test_remove_card() {
        let initial_cards = HomeCards::from_iter(vec![HomeCard::Connector]);
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockAntenna::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_stored_cards(
                initial_cards,
            )),
            observer.clone(),
        );

        manager.init_cards().await.unwrap();
        manager.remove_card(HomeCard::Connector).await.unwrap();

        let handled_cards = observer.handled_cards.lock().unwrap().clone();
        assert!(handled_cards.unwrap().is_empty());
    }

    #[actix_rt::test]
    async fn test_remove_card_does_nothing_if_card_does_not_exist() {
        let initial_cards = HomeCards::from_iter(vec![HomeCard::Connector]);
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockAntenna::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_stored_cards(
                initial_cards.clone(),
            )),
            observer.clone(),
        );

        manager.init_cards().await.unwrap();
        manager.remove_card(HomeCard::StartRadQuest).await.unwrap();

        let handled_cards = observer.handled_cards.lock().unwrap().clone();
        pretty_assertions::assert_eq!(handled_cards.unwrap(), initial_cards);
    }
}
