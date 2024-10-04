use crate::prelude::*;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

#[async_trait::async_trait]
pub trait DeferredDeepLinkParser: Send + Sync {
    async fn parse(&self, encoded_value: String) -> Result<HomeCards>;
}

pub struct Parser {
}

#[async_trait::async_trait]
impl DeferredDeepLinkParser for Parser {
    async fn parse(&self, encoded_value: String) -> Result<HomeCards> {
        let decoded = self.decode(encoded_value)?;
        let result = self.transform_onboarding_deep_link_value(decoded).await;
        Ok(result)
    }
}

impl Parser {
    fn decode(
        &self,
        encoded_value: impl AsRef<str>,
    ) -> Result<OnboardingDeepLinkValue> {
        let decoded_value_json_bytes = URL_SAFE_NO_PAD
            .decode(encoded_value.as_ref())
            .map_err(|e| CommonError::DeferredDeepLinkInvalidValueFormat {
                bad_value: e.to_string(),
            })?;

        deserialize_from_slice(decoded_value_json_bytes.as_slice())
    }
}

impl Parser {
    async fn transform_onboarding_deep_link_value(
        &self,
        value: OnboardingDeepLinkValue,
    ) -> HomeCards {
        let mut result = Vec::new();

        if let Some(DeferredDeepLinkSpecialDapp::RadQuest) = value.special_dapp
        {
            result.push(HomeCard::ContinueRadQuest)
        }

        HomeCards::from_iter(result)
    }
}