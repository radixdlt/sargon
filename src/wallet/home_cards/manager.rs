use crate::prelude::*;
use std::sync::RwLock;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HomeCardsStorage: Send + Sync {
    async fn save_cards(&self, encoded_cards: BagOfBytes) -> Result<()>;

    async fn load_cards(&self) -> Result<Option<BagOfBytes>>;
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HomeCardsObserver: Send + Sync + std::fmt::Debug {
    fn handle_cards_update(&self, cards: HomeCards);
}

#[derive(uniffi::Object)]
pub struct HomeCardsManager {
    #[allow(dead_code)] // Remove this line after gateway_client is being used
    parser: DeferredDeepLinkParser,
    #[allow(dead_code)] // Remove this line after cards_storage is being used
    cards_storage: Arc<dyn HomeCardsStorage>,
    observer: Arc<dyn HomeCardsObserver>,
    cards: RwLock<HomeCards>,
}

impl HomeCardsManager {
    pub fn init(
        gateway_client: GatewayClient,
        cards_storage: Arc<dyn HomeCardsStorage>,
        observer: Arc<dyn HomeCardsObserver>,
    ) -> Self {
        Self {
            parser: DeferredDeepLinkParser::new(gateway_client),
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
            GatewayClient::new(network_antenna, network_id),
            cards_storage,
            observer,
        )
    }
}

#[uniffi::export]
impl HomeCardsManager {
    #[uniffi::method]
    pub async fn handle_deferred_deep_link(
        &self,
        encoded_value: String,
    ) -> Result<()> {
        let deep_link_cards = self.parser.parse(encoded_value)?;
        _ = self
            .cards
            .try_write()
            .map(|mut cards| {
                deep_link_cards.into_iter().for_each(|card| {
                    match cards.try_insert_unique(card) {
                        Ok(_) => debug!("Home card inserted"),
                        Err(_) => debug!("Home card insert failed"),
                    }
                })
            })
            .map_err(|_| {
                CommonError::FailedUpdatingHomeCardsFromDeferredDeepLink
            });
        let updated_cards = self
            .cards
            .read()
            .ok()
            .ok_or(CommonError::HomeCardsNotFound)?;

        self.observer
            .handle_cards_update(updated_cards.deref().to_owned());
        Ok(())
    }
}
