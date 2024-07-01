use crate::prelude::*;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

pub fn parse_deferred_deep_link(
    encoded_value: impl AsRef<str>,
) -> Result<HomeCards> {
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
        let encoded_value = "eyJtZXRob2QiOiJtb2JpbGUiLCJkYXBwX3JlZmVycmVyIjoiYWNjb3VudF9yZHgxMjh5Nmo3OG10MGFxdjYzNzJldnoyOGhyeHA4bW4wNmNjZGRrcjd4cHBjODhoeXZ5bnZqZHdyIiwic3BlY2lhbF9kYXBwIjoicmFkcXVlc3QifQ";
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
        let encoded_value = "e30";
        let result = decode_deferred_deep_link(encoded_value).unwrap_err();
        assert_eq!(result, CommonError::DeferredDeepLinkDecodingFailed);
    }
}

fn transform_onboarding_deep_link_value(
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

    if value.dapp_referrer.is_some() {
        // TODO: Download the dApp metadata and set its icon_url
        result.push(HomeCard::Dapp { icon_url: (None) });
    }

    result.push(HomeCard::Connector);

    Ok(HomeCards::from_iter(result))
}

#[cfg(test)]
mod tests_transform {
    use super::*;

    #[test]
    fn transform_radquest_without_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            None,
            Some(DeferredDeepLinkSpecialDapp::RadQuest),
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let expected_result = HomeCards::from_iter([
            HomeCard::ContinueRadQuest,
            HomeCard::Connector,
        ]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_radquest_with_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            Some(AccountAddress::sample()),
            Some(DeferredDeepLinkSpecialDapp::RadQuest),
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let expected_result = HomeCards::from_iter([
            HomeCard::ContinueRadQuest,
            HomeCard::Dapp { icon_url: (None) },
            HomeCard::Connector,
        ]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_no_special_dapp_without_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Desktop,
            None,
            None,
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let expected_result = HomeCards::from_iter([
            HomeCard::StartRadQuest,
            HomeCard::Connector,
        ]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn transform_no_special_dapp_with_referrer() {
        let value = OnboardingDeepLinkValue::new(
            DeferredDeepLinkMethod::Mobile,
            Some(AccountAddress::sample()),
            None,
        );
        let result = transform_onboarding_deep_link_value(value).unwrap();
        let expected_result = HomeCards::from_iter([
            HomeCard::StartRadQuest,
            HomeCard::Dapp { icon_url: (None) },
            HomeCard::Connector,
        ]);
        assert_eq!(result, expected_result);
    }
}
