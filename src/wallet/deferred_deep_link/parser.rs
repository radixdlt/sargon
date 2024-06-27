use crate::prelude::*;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

pub fn parse_deferred_deep_link(
    encoded_value: impl AsRef<str>,
) -> Result<DeferredDeepLink> {
    let decoded = decode_deferred_deep_link(encoded_value)?;
    transform_onboarding_deep_link_value(decoded)
}

fn decode_deferred_deep_link(
    encoded_value: impl AsRef<str>,
) -> Result<OnboardingDeepLinkValue> {
    let decoded_value_json_bytes = URL_SAFE_NO_PAD
        .decode(encoded_value.as_ref())
        .map_err(|e| {
            println!("{}", e);
            CommonError::DeferredDeepLinkInvalidValueFormat
        })?;

    let deep_link_value = serde_json::from_slice::<OnboardingDeepLinkValue>(
        decoded_value_json_bytes.as_ref(),
    )
    .map_err(|_| CommonError::DeferredDeepLinkDecodingFailed)?;

    Ok(deep_link_value)
}

#[cfg(test)]
mod tests_decode {
    use super::*;

    #[test]
    fn decode_deferred_deep_link_correct() {
        let encoded_value = "eyJtZXRob2QiOiJtb2JpbGUiLCJyYWRxdWVzdCI6dHJ1ZSwiZGFwcF9yZWZlcnJlciI6ImFjY291bnRfcmR4MTI4eTZqNzhtdDBhcXY2MzcyZXZ6MjhocnhwOG1uMDZjY2Rka3I3eHBwYzg4aHl2eW52amR3ciIsImRhcHBfY2FsbGJhY2siOiJodHRwczovL2V4YW1wbGUuY29tIiwicmFkcXVlc3RfZGF0YSI6ImV4YW1wbGVfdHJhY2tpbmdfZGF0YSJ9";
        let result = decode_deferred_deep_link(encoded_value).unwrap();
        assert_eq!(result, OnboardingDeepLinkValue::sample());
    }

    #[test]
    fn decode_deferred_deep_link_invalid_value_format() {
        let encoded_value = "invalid format";
        let result = decode_deferred_deep_link(encoded_value).unwrap_err();
        assert_eq!(result, CommonError::DeferredDeepLinkInvalidValueFormat);
    }

    #[test]
    fn decode_deferred_deep_link_decoding_failed() {
        let encoded_value = "bm90IGEgdmFsaWQgSlNPTg==";
        let result = decode_deferred_deep_link(encoded_value).unwrap_err();
        assert_eq!(result, CommonError::DeferredDeepLinkInvalidValueFormat);
    }
}

fn transform_onboarding_deep_link_value(
    value: OnboardingDeepLinkValue,
) -> Result<DeferredDeepLink> {
    // NOTE: This won't be inside a From/Into implementation anymore and instead will have async logic
    // performed here, where we will download the dApp metadata and set its name.
    Ok(value.into())
}

#[cfg(test)]
mod tests_transform {
    use super::*;

    #[test]
    fn transform_radquest_mobile_without_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            true,
            None,
            None,
            Some("this is the tracking info".to_owned()),
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let cards = PostOnboardingCards::from_iter([
            PostOnboardingCard::ContinueRadQuest {
                should_redirect: (true),
                tracking_data: Some("this is the tracking info".to_owned()),
            },
            PostOnboardingCard::Connector,
        ]);
        let expected_result = DeferredDeepLink::Onboarding { cards };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_radquest_desktop_without_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Desktop,
            true,
            None,
            None,
            Some("this is the tracking info".to_owned()),
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let cards = PostOnboardingCards::from_iter([
            PostOnboardingCard::ContinueRadQuest {
                should_redirect: (false),
                tracking_data: Some("this is the tracking info".to_owned()),
            },
            PostOnboardingCard::Connector,
        ]);
        let expected_result = DeferredDeepLink::Onboarding { cards };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_radquest_mobile_with_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            true,
            Some(AccountAddress::sample()),
            Some("https://example.com".to_owned()),
            None,
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let cards = PostOnboardingCards::from_iter([
            PostOnboardingCard::ContinueRadQuest {
                should_redirect: (true),
                tracking_data: None,
            },
            PostOnboardingCard::Dapp {
                name: ("TODO".to_owned()),
                callback_url: Some(Url::parse("https://example.com").unwrap()),
            },
            PostOnboardingCard::Connector,
        ]);
        let expected_result = DeferredDeepLink::Onboarding { cards };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_radquest_desktop_with_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Desktop,
            true,
            Some(AccountAddress::sample()),
            Some("https://example.com".to_owned()),
            None,
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let cards = PostOnboardingCards::from_iter([
            PostOnboardingCard::ContinueRadQuest {
                should_redirect: (false),
                tracking_data: None,
            },
            PostOnboardingCard::Connector,
            PostOnboardingCard::Dapp {
                name: ("TODO".to_owned()),
                callback_url: None,
            },
        ]);
        let expected_result = DeferredDeepLink::Onboarding { cards };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_not_radquest_mobile_without_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            false,
            None,
            None,
            None,
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let cards = PostOnboardingCards::from_iter([
            PostOnboardingCard::StartRadQuest,
            PostOnboardingCard::Connector,
        ]);
        let expected_result = DeferredDeepLink::Onboarding { cards };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_not_radquest_desktop_without_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Desktop,
            false,
            None,
            None,
            None,
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let cards = PostOnboardingCards::from_iter([
            PostOnboardingCard::StartRadQuest,
            PostOnboardingCard::Connector,
        ]);
        let expected_result = DeferredDeepLink::Onboarding { cards };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_not_radquest_mobile_with_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            false,
            Some(AccountAddress::sample()),
            Some("https://example.com".to_owned()),
            None,
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let cards = PostOnboardingCards::from_iter([
            PostOnboardingCard::StartRadQuest,
            PostOnboardingCard::Dapp {
                name: ("TODO".to_owned()),
                callback_url: Some(Url::parse("https://example.com").unwrap()),
            },
            PostOnboardingCard::Connector,
        ]);
        let expected_result = DeferredDeepLink::Onboarding { cards };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_not_radquest_desktop_with_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Desktop,
            false,
            Some(AccountAddress::sample()),
            Some("https://example.com".to_owned()),
            None,
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let cards = PostOnboardingCards::from_iter([
            PostOnboardingCard::StartRadQuest,
            PostOnboardingCard::Connector,
            PostOnboardingCard::Dapp {
                name: ("TODO".to_owned()),
                callback_url: None,
            },
        ]);
        let expected_result = DeferredDeepLink::Onboarding { cards };
        assert_eq!(result, expected_result);
    }
}
