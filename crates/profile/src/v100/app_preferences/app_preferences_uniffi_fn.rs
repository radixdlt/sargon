use crate::prelude::*;

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
}
