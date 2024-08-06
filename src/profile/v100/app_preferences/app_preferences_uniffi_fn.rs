use crate::prelude::*;
use crate::types::*;

#[uniffi::export]
pub fn new_app_preferences_sample() -> AppPreferences {
    AppPreferences::sample()
}

#[uniffi::export]
pub fn new_app_preferences_sample_other() -> AppPreferences {
    AppPreferences::sample_other()
}

#[uniffi::export]
pub fn new_app_preferences_default() -> AppPreferences {
    AppPreferences::default()
}

#[uniffi::export]
pub fn app_preferences_has_gateway_with_url(
    app_preferences: AppPreferences,
    url: &FfiUrl,
) -> bool {
    app_preferences.has_gateway_with_url(url.url.clone())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AppPreferences;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_app_preferences_sample());
        assert_eq!(SUT::sample_other(), new_app_preferences_sample_other());
    }

    #[test]
    fn test_default() {
        assert_eq!(new_app_preferences_default(), AppPreferences::default());
    }

    #[test]
    fn test_app_preferences_has_gateway_with_url() {
        assert!(app_preferences_has_gateway_with_url(
            SUT::sample(),
            &FfiUrl::new(Url::parse("https://mainnet.radixdlt.com").unwrap())
                .unwrap()
        ));
    }
}
