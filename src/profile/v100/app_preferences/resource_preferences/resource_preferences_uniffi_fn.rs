use crate::prelude::*;

#[uniffi::export]
pub fn new_resource_preferences_sample() -> ResourcePreferences {
    ResourcePreferences::sample()
}

#[uniffi::export]
pub fn new_resource_preferences_sample_other() -> ResourcePreferences {
    ResourcePreferences::sample_other()
}

#[uniffi::export]
pub fn resource_preferences_get_hidden_resources(
    resource_preferences: ResourcePreferences,
) -> HiddenResources {
    resource_preferences.get_hidden_resources()
}

#[uniffi::export]
pub fn resource_preferences_hide_resource(
    resource_preferences: &ResourcePreferences,
    kind: ResourcePreferenceKind,
) -> ResourcePreferences {
    let mut resource_preferences = resource_preferences.clone();
    resource_preferences.hide_resource(kind);
    resource_preferences
}

#[uniffi::export]
pub fn resource_preferences_unhide_resource(
    resource_preferences: &ResourcePreferences,
    kind: ResourcePreferenceKind,
) -> ResourcePreferences {
    let mut resource_preferences = resource_preferences.clone();
    resource_preferences.unhide_resource(kind);
    resource_preferences
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourcePreferences;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_resource_preferences_sample());
        assert_eq!(
            SUT::sample_other(),
            new_resource_preferences_sample_other()
        );
    }

    #[test]
    fn hidden_resources() {
        // Test with empty ResourcePreferences
        let sut = SUT::new();
        let mut result = resource_preferences_get_hidden_resources(sut.clone());
        assert!(result.fungible.is_empty());

        // Test after hiding a fungible resource
        let fungible = ResourceAddress::sample();
        let after_hiding = resource_preferences_hide_resource(
            &sut,
            ResourcePreferenceKind::Fungible(fungible),
        );

        result =
            resource_preferences_get_hidden_resources(after_hiding.clone());
        assert_eq!(result.fungible, vec![fungible]);

        // Test after unhiding the fungible resource
        let after_unhiding = resource_preferences_unhide_resource(
            &after_hiding,
            ResourcePreferenceKind::Fungible(fungible),
        );
        result = resource_preferences_get_hidden_resources(after_unhiding);
        assert!(result.fungible.is_empty());
    }
}
