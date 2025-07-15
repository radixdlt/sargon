use gateway_client_and_api::GatewayClient;

use crate::prelude::*;
use std::sync::RwLockWriteGuard;

/// Manages the home cards by handling storage, parsing, and updating operations.
/// Call `bootstrap` before invoking any other public functions.
pub struct HomeCardsManager {
    /// Parser for handling deferred deep links.
    parser: Arc<dyn DeferredDeepLinkParser>,
    /// Storage for saving and loading home cards.
    cards_storage: Arc<dyn HomeCardsStorage>,
    /// Observer to handle updates to the home cards.
    observer: Arc<dyn HomeCardsObserver>,
    /// In-memory storage of the current home cards.
    cards: RwLock<HomeCards>,
    /// In-memory storage of the dismissed cards.
    dismissed_cards: RwLock<HomeCards>,
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
            dismissed_cards: RwLock::new(HomeCards::new()),
        }
    }
}

impl HomeCardsManager {
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        network_id: NetworkID,
        cards_storage: Arc<dyn HomeCardsStorage>,
        observer: Arc<dyn HomeCardsObserver>,
    ) -> Self {
        Self::init(
            Arc::new(Parser::new(GatewayClient::new(
                networking_driver,
                network_id,
            ))),
            cards_storage,
            observer,
        )
    }
}

impl HomeCardsManager {
    /// Initializes `HomeCards` by loading from storage.
    /// This function should be called before invoking any other public functions.
    /// Notifies `HomeCardsObserver`.
    pub async fn bootstrap(&self) -> Result<()> {
        // The cards that can be added on app update.
        // Compared to other cards which are added explicitely only on wallet creation, these
        // cards will be shown also for existing users which do have the wallet configured.
        let cards_to_be_added_on_update =
            HomeCards::from_iter([HomeCard::JoinRadixRewards]);

        let stored_dimissed_cards = self.load_dismissed_cards().await?;

        let cards_to_add = cards_to_be_added_on_update
            .into_iter()
            .filter(|card| !stored_dimissed_cards.contains_by_id(&card))
            .collect_vec();

        let mut stored_cards = self.load_cards().await?;
        stored_cards.extend(cards_to_add);

        self.update_cards(|write_guard| {
            Self::insert_cards(write_guard, stored_cards)
        })
        .await?;
        self.update_dismissed_cards(|write_guard| {
            Self::insert_cards(write_guard, stored_dimissed_cards)
        })
        .await?;
        Ok(())
    }

    /// Initializes and saves to storage default `HomeCards`.
    /// Marks the wallet creation and populates the set of cards required for a new wallet.
    /// Notifies `HomeCardsObserver`.
    pub async fn wallet_created(&self) -> Result<()> {
        let default_cards = HomeCards::from_iter([
            HomeCard::JoinRadixRewards,
            HomeCard::Connector,
            HomeCard::StartRadQuest,
        ]);
        let updated_cards = self
            .update_cards(|write_guard| {
                Self::insert_cards(write_guard, default_cards.clone())
            })
            .await?;
        self.save_cards(updated_cards).await
    }

    /// Handles a deferred deep link by parsing it and saving the generated `HomeCards` to `HomeCardsStorage`.
    /// `HomeCard::ContinueRadQuest` if found in the link parsing result, replaces `HomeCard::StartRadQuest`.
    /// Notifies `HomeCardsObserver`.
    pub async fn deferred_deep_link_received(
        &self,
        encoded_value: String,
    ) -> Result<()> {
        let deep_link_cards = self.parser.parse(encoded_value).await?;
        let updated_cards = self
            .update_cards(|write_guard| {
                Self::insert_cards(write_guard, deep_link_cards)
            })
            .await?;
        self.save_cards(updated_cards).await
    }

    /// Marks the wallet restoration.
    /// Ensures only the expected `HomeCards` remain in `HomeCardsStorage` - currently none.
    /// Notifies `HomeCardsObserver`.
    pub async fn wallet_restored(&self) -> Result<()> {
        let updated_cards = self
            .update_cards(|write_guard| {
                **write_guard = HomeCards::new();
            })
            .await?;
        self.save_cards(updated_cards).await
    }

    /// Dismisses a specified `HomeCard` by removing it from `HomeCardsStorage`.
    /// Notifies `HomeCardsObserver`.
    pub async fn card_dismissed(&self, card: HomeCard) -> Result<()> {
        let updated_cards = self
            .update_cards(|write_guard| {
                write_guard.remove_id(&card);
            })
            .await?;
        self.save_cards(updated_cards).await?;

        let updated_dismissed_cards = self
            .update_dismissed_cards(|write_guard| {
                write_guard.insert(card);
            })
            .await?;
        self.save_dismissed_cards(updated_dismissed_cards).await
    }

    /// Clears the home cards from the `HomeCardsStorage`.
    /// Notifies `HomeCardsObserver`.
    pub async fn wallet_reset(&self) -> Result<()> {
        let updated_cards = self
            .update_cards(|write_guard| {
                **write_guard = HomeCards::new();
            })
            .await?;
        self.save_cards(updated_cards).await
    }
}

impl HomeCardsManager {
    /// Updates in-memory home cards after applying `f` function, then notifies the observer.
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
        let sorted_cards = updated_cards.sort();

        self.observer.handle_cards_update(sorted_cards.clone());
        Ok(sorted_cards)
    }

    fn insert_cards(
        write_guard: &mut RwLockWriteGuard<HomeCards>,
        cards: HomeCards,
    ) {
        // Insert all cards into write_guard
        cards.into_iter().for_each(|card| {
            if write_guard.try_insert_unique(card).is_ok() {
                debug!("Home card inserted");
            } else {
                debug!("Home card insert failed");
            }
        });

        // Check if ContinueRadQuest is present and remove StartRadQuest if it is
        if write_guard.contains_id(&HomeCard::ContinueRadQuest) {
            write_guard.remove_id(&HomeCard::StartRadQuest);
        }
    }

    async fn update_dismissed_cards<F>(&self, f: F) -> Result<HomeCards>
    where
        F: FnOnce(&mut RwLockWriteGuard<HomeCards>),
    {
        let mut write_guard = self
            .dismissed_cards
            .write()
            .map_err(|_| CommonError::FailedUpdatingHomeCards)?;

        f(&mut write_guard);
        let updated_cards = write_guard.clone();

        Ok(updated_cards)
    }
}

impl HomeCardsManager {
    /// Loads the home cards from storage.
    async fn load_cards(&self) -> Result<HomeCards> {
        let cards_bytes = self
            .cards_storage
            .load_cards()
            .await?
            .ok_or(CommonError::HomeCardsNotFound)?;

        Self::decode_cards(cards_bytes)
    }

    /// Loads the home cards from storage.
    async fn load_dismissed_cards(&self) -> Result<HomeCards> {
        let cards_bytes = self
            .cards_storage
            .load_dismissed_cards()
            .await?
            .ok_or(CommonError::HomeCardsNotFound)?;

        Self::decode_cards(cards_bytes)
    }

    fn decode_cards(cards_bytes: BagOfBytes) -> Result<HomeCards> {
        // Needs special handling. Some HomeCard variants have been removed.
        // To ensure compatibility with old users, such cards need to be ignored.
        let slice = cards_bytes.bytes();
        serde_json::from_slice::<Vec<String>>(slice)
            .map(|serialized_cards| {
                serialized_cards
                    .iter()
                    .filter_map(|card_str| {
                        HomeCard::deserialize_from_string(card_str).ok() // Ignore unknown cards
                    })
                    .collect::<HomeCards>()
            })
            .map_failed_to_deserialize_bytes::<Self>(slice)
    }

    /// Saves the home cards to storage.
    async fn save_cards(&self, cards: HomeCards) -> Result<()> {
        let bytes = cards.serialize_to_bytes()?;
        self.cards_storage
            .save_cards(bytes.into())
            .await
            .map_err(|_| CommonError::FailedSavingHomeCards)
    }

    async fn save_dismissed_cards(&self, cards: HomeCards) -> Result<()> {
        let bytes = cards.serialize_to_bytes()?;
        self.cards_storage
            .save_dismissed_cards(bytes.into())
            .await
            .map_err(|_| CommonError::FailedSavingHomeCards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct MockHomeCardsStorage {
        stubbed_save_cards_result: Result<()>,
        stubbed_load_cards_result: Result<Option<BagOfBytes>>,
        stubbed_save_dismissed_cards_result: Result<()>,
        stubbed_load_dismissed_cards_result: Result<Option<BagOfBytes>>,
    }

    impl MockHomeCardsStorage {
        fn new_empty() -> Self {
            Self {
                stubbed_save_cards_result: Ok(()),
                stubbed_load_cards_result: Self::encode_cards(HomeCards::new()),
                stubbed_save_dismissed_cards_result: Ok(()),
                stubbed_load_dismissed_cards_result: Self::encode_cards(
                    HomeCards::new(),
                ),
            }
        }

        fn new_with_stored_cards(cards: HomeCards) -> Self {
            Self {
                stubbed_save_cards_result: Ok(()),
                stubbed_load_cards_result: Self::encode_cards(cards),
                stubbed_save_dismissed_cards_result: Ok(()),
                stubbed_load_dismissed_cards_result: Self::encode_cards(
                    HomeCards::new(),
                ),
            }
        }

        fn new_with_cards_and_old_now_removed_encoded_cards() -> Self {
            let stored_json = r#"["DiscoverRadixDapps","Connector"]"#;
            let encoded = BagOfBytes::from(stored_json.as_bytes());

            Self {
                stubbed_save_cards_result: Ok(()),
                stubbed_load_cards_result: Ok(Some(encoded)),
                stubbed_save_dismissed_cards_result: Ok(()),
                stubbed_load_dismissed_cards_result: Self::encode_cards(
                    HomeCards::new(),
                ),
            }
        }

        fn new_with_load_error() -> Self {
            Self {
                stubbed_save_cards_result: Ok(()),
                stubbed_load_cards_result: Err(CommonError::HomeCardsNotFound),
                stubbed_save_dismissed_cards_result: Ok(()),
                stubbed_load_dismissed_cards_result: Self::encode_cards(
                    HomeCards::new(),
                ),
            }
        }

        fn new_with_save_error() -> Self {
            Self {
                stubbed_save_cards_result: Err(
                    CommonError::FailedSavingHomeCards,
                ),
                stubbed_load_cards_result: Self::encode_cards(HomeCards::new()),
                stubbed_save_dismissed_cards_result: Ok(()),
                stubbed_load_dismissed_cards_result: Self::encode_cards(
                    HomeCards::new(),
                ),
            }
        }

        fn encode_cards(cards: HomeCards) -> Result<Option<BagOfBytes>> {
            cards.serialize_to_bytes().map(|cards| Some(cards.into()))
        }
    }

    #[async_trait::async_trait]
    impl HomeCardsStorage for MockHomeCardsStorage {
        async fn save_cards(&self, encoded_cards: BagOfBytes) -> Result<()> {
            let _: HomeCards = encoded_cards.deserialize()?;
            self.stubbed_save_cards_result.clone()
        }

        async fn load_cards(&self) -> Result<Option<BagOfBytes>> {
            self.stubbed_load_cards_result.clone()
        }

        async fn save_dismissed_cards(
            &self,
            encoded_cards: BagOfBytes,
        ) -> Result<()> {
            let _: HomeCards = encoded_cards.deserialize()?;
            self.stubbed_save_dismissed_cards_result.clone()
        }

        async fn load_dismissed_cards(&self) -> Result<Option<BagOfBytes>> {
            self.stubbed_load_dismissed_cards_result.clone()
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
    async fn test_bootstrap_with_stored_cards_containing_old_cards() {
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_cards_and_old_now_removed_encoded_cards()),
            observer.clone(),
        );

        manager.bootstrap().await.unwrap();
        let handled_cards = observer.handled_cards.lock().unwrap().clone();

        pretty_assertions::assert_eq!(
            handled_cards,
            Some(HomeCards::from_iter([
                HomeCard::JoinRadixRewards,
                HomeCard::Connector
            ]))
        );
    }

    #[actix_rt::test]
    async fn test_bootstrap_with_stored_cards() {
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_stored_cards(
                HomeCards::just(HomeCard::Connector),
            )),
            observer.clone(),
        );

        manager.bootstrap().await.unwrap();
        let handled_cards = observer.handled_cards.lock().unwrap().clone();

        pretty_assertions::assert_eq!(
            handled_cards,
            Some(HomeCards::from_iter([
                HomeCard::JoinRadixRewards,
                HomeCard::Connector
            ]))
        );
    }

    #[actix_rt::test]
    async fn test_bootstrap_failing() {
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_load_error()),
            observer.clone(),
        );

        let result = manager.bootstrap().await.unwrap_err();
        assert_eq!(result, CommonError::HomeCardsNotFound);
    }

    #[actix_rt::test]
    async fn test_wallet_created() {
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_empty()),
            observer.clone(),
        );
        let expected_cards = HomeCards::from_iter(vec![
            HomeCard::JoinRadixRewards,
            HomeCard::StartRadQuest,
            HomeCard::Connector,
        ]);

        manager.wallet_created().await.unwrap();

        let handled_cards =
            observer.handled_cards.lock().unwrap().clone().unwrap();
        pretty_assertions::assert_eq!(
            handled_cards.items(),
            expected_cards.items()
        );
    }

    #[actix_rt::test]
    async fn test_wallet_created_failing() {
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_save_error()),
            observer.clone(),
        );

        let result = manager.wallet_created().await.unwrap_err();
        assert_eq!(result, CommonError::FailedSavingHomeCards);
    }

    #[actix_rt::test]
    async fn test_wallet_created_with_stored_cards() {
        let expected_cards = HomeCards::from_iter(vec![
            HomeCard::JoinRadixRewards,
            HomeCard::StartRadQuest,
            HomeCard::Connector,
        ]);
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_stored_cards(
                expected_cards.clone(),
            )),
            observer.clone(),
        );

        manager.wallet_created().await.unwrap();
        let handled_cards =
            observer.handled_cards.lock().unwrap().clone().unwrap();
        pretty_assertions::assert_eq!(
            handled_cards.items(),
            expected_cards.items()
        );
    }

    #[actix_rt::test]
    async fn test_wallet_created_after_deferred_deep_link_received() {
        let deep_link_cards = HomeCards::from_iter(vec![
            HomeCard::Dapp { icon_url: None },
            HomeCard::ContinueRadQuest,
        ]);
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::init(
            Arc::new(MockDeferredDeepLinkParser::succeeding(
                deep_link_cards.clone(),
            )),
            Arc::new(MockHomeCardsStorage::new_empty()),
            observer.clone(),
        );

        manager
            .deferred_deep_link_received("encoded_value".to_string())
            .await
            .unwrap();
        manager.wallet_created().await.unwrap();

        let expected_cards = HomeCards::from_iter(vec![
            HomeCard::JoinRadixRewards,
            HomeCard::ContinueRadQuest,
            HomeCard::Dapp { icon_url: None },
            HomeCard::Connector,
        ]);
        let handled_cards =
            observer.handled_cards.lock().unwrap().clone().unwrap();
        pretty_assertions::assert_eq!(
            handled_cards.items(),
            expected_cards.items()
        );
    }

    #[actix_rt::test]
    async fn test_deferred_deep_link_received() {
        let observer = Arc::new(MockHomeCardsObserver::new());
        let stored_cards = HomeCards::from_iter(vec![
            HomeCard::Connector,
            HomeCard::StartRadQuest,
        ]);
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

        manager.bootstrap().await.unwrap();
        manager
            .deferred_deep_link_received("encoded_value".to_string())
            .await
            .unwrap();
        // Covers the case where `HomeCard::StartRadQuest` shouldn't replace the already existing `HomeCard::ContinueRadQuest`
        manager.wallet_created().await.unwrap();

        let expected_cards = HomeCards::from_iter(vec![
            HomeCard::JoinRadixRewards,
            HomeCard::ContinueRadQuest,
            HomeCard::Dapp { icon_url: None },
            HomeCard::Connector,
        ]);
        let handled_cards =
            observer.handled_cards.lock().unwrap().clone().unwrap();
        pretty_assertions::assert_eq!(
            handled_cards.items(),
            expected_cards.items()
        );
    }

    #[actix_rt::test]
    async fn test_deferred_deep_link_received_then_wallet_restored() {
        let deep_link_cards = HomeCards::from_iter(vec![
            HomeCard::Dapp { icon_url: None },
            HomeCard::ContinueRadQuest,
        ]);
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::init(
            Arc::new(MockDeferredDeepLinkParser::succeeding(
                deep_link_cards.clone(),
            )),
            Arc::new(MockHomeCardsStorage::new_empty()),
            observer.clone(),
        );

        manager
            .deferred_deep_link_received("encoded_value".to_string())
            .await
            .unwrap();
        manager.wallet_restored().await.unwrap();

        let handled_cards = observer.handled_cards.lock().unwrap().clone();
        pretty_assertions::assert_eq!(handled_cards.unwrap(), HomeCards::new());
    }

    #[actix_rt::test]
    async fn test_card_dismissed() {
        let initial_cards = HomeCards::from_iter(vec![
            HomeCard::JoinRadixRewards,
            HomeCard::Connector,
        ]);
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_stored_cards(
                initial_cards,
            )),
            observer.clone(),
        );

        manager.bootstrap().await.unwrap();
        manager.card_dismissed(HomeCard::Connector).await.unwrap();

        let handled_cards =
            observer.handled_cards.lock().unwrap().clone().unwrap();

        assert_eq!(handled_cards.len(), 1);
    }

    #[actix_rt::test]
    async fn test_card_dismissed_does_nothing_if_card_does_not_exist() {
        let initial_cards = HomeCards::from_iter([
            HomeCard::JoinRadixRewards,
            HomeCard::Connector,
        ]);
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_stored_cards(
                initial_cards.clone(),
            )),
            observer.clone(),
        );

        manager.bootstrap().await.unwrap();
        manager
            .card_dismissed(HomeCard::StartRadQuest)
            .await
            .unwrap();

        let handled_cards = observer.handled_cards.lock().unwrap().clone();
        pretty_assertions::assert_eq!(handled_cards.unwrap(), initial_cards);
    }

    #[actix_rt::test]
    async fn test_wallet_reset() {
        let initial_cards = HomeCards::from_iter(vec![
            HomeCard::ContinueRadQuest,
            HomeCard::Connector,
        ]);
        let observer = Arc::new(MockHomeCardsObserver::new());
        let manager = SUT::new(
            Arc::new(MockNetworkingDriver::new_always_failing()),
            NetworkID::Stokenet,
            Arc::new(MockHomeCardsStorage::new_with_stored_cards(
                initial_cards.clone(),
            )),
            observer.clone(),
        );

        manager.bootstrap().await.unwrap();
        manager.wallet_reset().await.unwrap();

        let handled_cards = observer.handled_cards.lock().unwrap().clone();
        pretty_assertions::assert_eq!(handled_cards.unwrap(), HomeCards::new());
    }
}
