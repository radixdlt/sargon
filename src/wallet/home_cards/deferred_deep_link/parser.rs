use crate::prelude::*;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

pub struct DeferredDeepLinkParser {
    gateway_client: GatewayClient,
}

impl DeferredDeepLinkParser {
    pub fn new(gateway_client: GatewayClient) -> Self {
        Self { gateway_client }
    }
}

impl DeferredDeepLinkParser {
    pub async fn parse(
        &self,
        encoded_value: impl AsRef<str>,
    ) -> Result<HomeCards> {
        let decoded = self.decode(encoded_value)?;
        self.transform_onboarding_deep_link_value(decoded).await
    }

    fn decode(
        &self,
        encoded_value: impl AsRef<str>,
    ) -> Result<OnboardingDeepLinkValue> {
        let decoded_value_json_bytes = URL_SAFE_NO_PAD
            .decode(encoded_value.as_ref())
            .map_err(|e| {
                println!("{}", e);
                CommonError::DeferredDeepLinkInvalidValueFormat
            })?;

        let deep_link_value =
            serde_json::from_slice::<OnboardingDeepLinkValue>(
                decoded_value_json_bytes.as_ref(),
            )
            .map_err(|_| CommonError::DeferredDeepLinkDecodingFailed)?;

        Ok(deep_link_value)
    }
}

#[cfg(test)]
mod tests_decode {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeferredDeepLinkParser;

    fn make_sut() -> SUT {
        SUT::new(GatewayClient::new(
            Arc::new(MockAntenna::new_always_failing()),
            NetworkID::Stokenet,
        ))
    }

    #[test]
    fn decode_correct() {
        let sut = make_sut();
        let encoded_value = "eyJtZXRob2QiOiJtb2JpbGUiLCJkYXBwX3JlZmVycmVyIjoiYWNjb3VudF9yZHgxMjh5Nmo3OG10MGFxdjYzNzJldnoyOGhyeHA4bW4wNmNjZGRrcjd4cHBjODhoeXZ5bnZqZHdyIiwic3BlY2lhbF9kYXBwIjoicmFkcXVlc3QifQ";
        let result = sut.decode(encoded_value).unwrap();
        assert_eq!(result, OnboardingDeepLinkValue::sample());
    }

    #[test]
    fn decode_invalid_value_format() {
        let sut = make_sut();
        let encoded_value = "invalid format";
        let result = sut.decode(encoded_value).unwrap_err();
        assert_eq!(result, CommonError::DeferredDeepLinkInvalidValueFormat);
    }

    #[test]
    fn decode_decoding_failed() {
        let sut = make_sut();
        let encoded_value = "e30";
        let result = sut.decode(encoded_value).unwrap_err();
        assert_eq!(result, CommonError::DeferredDeepLinkDecodingFailed);
    }
}

impl DeferredDeepLinkParser {
    async fn transform_onboarding_deep_link_value(
        &self,
        value: OnboardingDeepLinkValue,
    ) -> Result<HomeCards> {
        let mut result = Vec::new();

        if let Some(special_dapp) = value.special_dapp {
            if special_dapp == DeferredDeepLinkSpecialDapp::RadQuest {
                result.push(HomeCard::ContinueRadQuest);
            } else {
                result.push(HomeCard::StartRadQuest);
            }
        } else {
            result.push(HomeCard::StartRadQuest);
        }

        if let Some(dapp_referrer) = value.dapp_referrer {
            match self
                .gateway_client
                .icon_url_of_address(dapp_referrer.to_string())
                .await
            {
                Ok(icon_url) => result.push(HomeCard::Dapp {
                    icon_url: (icon_url),
                }),
                Err(_) => result.push(HomeCard::Dapp { icon_url: None }),
            }
        }

        result.push(HomeCard::Connector);

        Ok(HomeCards::from_iter(result))
    }
}

#[cfg(test)]
mod tests_transform {
    use super::*;
    use actix_rt::time::timeout;
    use std::time::Duration;

    const MAX: Duration = Duration::from_millis(10);

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeferredDeepLinkParser;

    fn make_sut(icon_url: Url) -> SUT {
        let mock_antenna =
            MockAntenna::with_response(StateEntityDetailsResponse {
                items: vec![StateEntityDetailsResponseItem {
                    address: Address::sample_account_stokenet(),
                    fungible_resources: None,
                    metadata: EntityMetadataCollection {
                        items: vec![EntityMetadataItem {
                            key: MetadataKey::IconUrl.to_string(),
                            value: EntityMetadataItemValue {
                                typed: MetadataTypedValue::MetadataUrlValue {
                                    value: (icon_url),
                                },
                            },
                        }],
                    },
                }],
            });
        SUT::new(GatewayClient::new(
            Arc::new(mock_antenna),
            NetworkID::Stokenet,
        ))
    }

    fn make_failing_sut() -> SUT {
        SUT::new(GatewayClient::new(
            Arc::new(MockAntenna::new_always_failing()),
            NetworkID::Stokenet,
        ))
    }

    #[actix_rt::test]
    async fn transform_radquest_without_referrer() {
        let sut = make_failing_sut();
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            None,
            Some(DeferredDeepLinkSpecialDapp::RadQuest),
        );
        let req = sut.transform_onboarding_deep_link_value(value);
        let result = timeout(MAX, req).await.unwrap().unwrap();
        let expected_result = HomeCards::from_iter([
            HomeCard::ContinueRadQuest,
            HomeCard::Connector,
        ]);
        assert_eq!(result, expected_result);
    }

    #[actix_rt::test]
    async fn transform_radquest_with_referrer() {
        let icon_url = Url::parse("https://www.example.com").unwrap();
        let sut = make_sut(icon_url.clone());
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            Some(AccountAddress::sample_stokenet()),
            Some(DeferredDeepLinkSpecialDapp::RadQuest),
        );
        let req = sut.transform_onboarding_deep_link_value(value);
        let result = timeout(MAX, req).await.unwrap().unwrap();
        let expected_result = HomeCards::from_iter([
            HomeCard::ContinueRadQuest,
            HomeCard::Dapp {
                icon_url: (Some(icon_url)),
            },
            HomeCard::Connector,
        ]);
        assert_eq!(result, expected_result);
    }

    #[actix_rt::test]
    async fn transform_no_special_dapp_without_referrer() {
        let sut = make_failing_sut();
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Desktop,
            None,
            None,
        );
        let req = sut.transform_onboarding_deep_link_value(value);
        let result = timeout(MAX, req).await.unwrap().unwrap();
        let expected_result = HomeCards::from_iter([
            HomeCard::StartRadQuest,
            HomeCard::Connector,
        ]);
        assert_eq!(result, expected_result);
    }

    #[actix_rt::test]
    async fn transform_no_special_dapp_with_referrer() {
        let icon_url = Url::parse("https://www.example.com").unwrap();
        let sut = make_sut(icon_url.clone());
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            Some(AccountAddress::sample_stokenet()),
            None,
        );
        let req = sut.transform_onboarding_deep_link_value(value);
        let result = timeout(MAX, req).await.unwrap().unwrap();
        let expected_result = HomeCards::from_iter([
            HomeCard::StartRadQuest,
            HomeCard::Dapp {
                icon_url: (Some(icon_url)),
            },
            HomeCard::Connector,
        ]);
        assert_eq!(result, expected_result);
    }

    #[actix_rt::test]
    async fn transform_no_special_dapp_with_referrer_that_cannot_be_retrieved()
    {
        let sut = make_failing_sut();
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            Some(AccountAddress::sample()),
            None,
        );
        let req = sut.transform_onboarding_deep_link_value(value);
        let result = timeout(MAX, req).await.unwrap().unwrap();
        let expected_result = HomeCards::from_iter([
            HomeCard::StartRadQuest,
            HomeCard::Dapp { icon_url: (None) },
            HomeCard::Connector,
        ]);
        assert_eq!(result, expected_result);
    }
}
