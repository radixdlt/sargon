use crate::prelude::*;

#[uniffi::export]
pub fn resource_preferences_get_hidden_resources(
    resource_preferences: ResourcePreferences,
) -> HiddenResources {
    resource_preferences.get_hidden_resources()
}

#[uniffi::export]
pub fn resource_preferences_hide_resource(
    resource_preferences: ResourcePreferences,
    resource: ResourceIdentifier,
) -> ResourcePreferences {
    let mut resource_preferences = resource_preferences.clone();
    resource_preferences.hide_resource(resource);
    resource_preferences
}

#[uniffi::export]
pub fn resource_preferences_unhide_resource(
    resource_preferences: ResourcePreferences,
    resource: ResourceIdentifier,
) -> ResourcePreferences {
    let mut resource_preferences = resource_preferences.clone();
    resource_preferences.unhide_resource(resource);
    resource_preferences
}

#[cfg(test)]
mod tests {

    use std::clone;

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
        let mut sut = SUT::new();
        let mut result = resource_preferences_get_hidden_resources(sut.clone());
        assert!(result.is_empty());

        // Test after hiding one resource
        let resource_one = ResourceIdentifier::sample();
        sut = resource_preferences_hide_resource(sut, resource_one.clone());

        result = resource_preferences_get_hidden_resources(sut.clone());
        assert_eq!(HiddenResources::from_iter([resource_one.clone()]), result);

        // Test after unhiding the fungible resource
        sut = resource_preferences_unhide_resource(sut, resource_one.clone());
        result = resource_preferences_get_hidden_resources(sut);
        assert!(result.is_empty());
    }
}
