use crate::prelude::*;
use std::sync::RwLock;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait PostOnboardingCardsStorage: Send + Sync {
    async fn save_cards(&self, encoded_cards: BagOfBytes) -> Result<()>;

    async fn load_cards(&self) -> Result<Option<BagOfBytes>>;
}

#[derive(uniffi::Object)]
pub struct PostOnboardingCardsManager {
    #[allow(dead_code)] // Remove this line after gateway_client is being used
    gateway_client: GatewayClient,
    cards: RwLock<PostOnboardingCards>,
}

impl PostOnboardingCardsManager {
    pub fn init(gateway_client: GatewayClient) -> Self {
        Self {
            gateway_client,
            cards: RwLock::new(PostOnboardingCards::new()),
        }
    }
}

#[uniffi::export]
impl PostOnboardingCardsManager {
    #[uniffi::constructor]
    pub fn new(
        network_antenna: Arc<dyn NetworkAntenna>,
        network_id: NetworkID,
    ) -> Self {
        Self::init(GatewayClient::new(network_antenna, network_id))
    }
}

#[uniffi::export]
impl PostOnboardingCardsManager {
    #[uniffi::method]
    pub async fn handle_deferred_deep_link(
        &self,
        encoded_value: String,
    ) -> Result<()> {
        let deep_link_cards = parse_deferred_deep_link(encoded_value)?;
        self.cards
            .try_write()
            .map(|mut cards| {
                deep_link_cards.into_iter().for_each(|card| {
                    match cards.try_insert_unique(card) {
                        Ok(_) => debug!("Post-onboarding card inserted"),
                        Err(_) => debug!("Post-onboarding card insert failed"),
                    }
                })
            })
            .map_err(|_| {
                CommonError::FailedUpdatingPostOnboardingCardsFromDeferredDeepLink
            })
    }
}
