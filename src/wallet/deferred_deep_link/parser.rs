use crate::prelude::*;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

pub fn parse_onboarding_deep_link_value(
    encoded_value: impl AsRef<str>,
) -> Result<DeferredDeepLink> {
    let decoded_value_json_bytes = URL_SAFE_NO_PAD
        .decode(encoded_value.as_ref())
        .map_err(|_| CommonError::DeferredDeepLinkInvalidValueFormat)?;
    let deep_link_value = serde_json::from_slice::<OnboardingDeepLinkValue>(
        decoded_value_json_bytes.as_ref(),
    )
    .map_err(|_| CommonError::DeferredDeepLinkDecodingFailed)?;

    Ok(deep_link_value.into())
}

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn parse_onboarding_deep_link_value_into_deferred_deep_link() {
//         let value =
//         let result = parse_onboarding_deep_link_value()
//     }
// }
