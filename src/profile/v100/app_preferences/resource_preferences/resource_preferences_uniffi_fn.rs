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
) -> Vec<ResourceAddress> {
    resource_preferences.get_hidden_resources()
}

#[uniffi::export]
pub fn resource_preferences_has_resource_hidden(
    resource_preferences: ResourcePreferences,
    resource: ResourceAddress,
) -> bool {
    resource_preferences.is_resource_hidden(resource)
}

#[uniffi::export]
pub fn resource_preferences_hide_resource(
    resource_preferences: &ResourcePreferences,
    resource: ResourceAddress,
) -> ResourcePreferences {
    let mut resource_preferences = resource_preferences.clone();
    resource_preferences.hide_resource(resource);
    resource_preferences
}

#[uniffi::export]
pub fn resource_preferences_unhide_resource(
    resource_preferences: &ResourcePreferences,
    resource: ResourceAddress,
) -> ResourcePreferences {
    let mut resource_preferences = resource_preferences.clone();
    resource_preferences.unhide_resource(resource);
    resource_preferences
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourcePreferences;

    #[test]
    fn test_hidden_resources() {
        let mut sut = SUT::sample();
        sut.hide_resource(ResourceAddress::sample_other());
        sut.hide_resource(ResourceAddress::sample());

        assert_eq!(
            vec![ResourceAddress::sample_other(), ResourceAddress::sample()],
            sut.get_hidden_resources()
        );
    }

    #[test]
    fn test_hide_unhide_resource() {
        let mut sut = SUT::sample();
        let resource = ResourceAddress::sample_stokenet_candy();

        // Test the resource isn't hidden by default
        assert!(!resource_preferences_has_resource_hidden(
            sut.clone(),
            resource
        ));

        // Hide the resource
        sut = resource_preferences_hide_resource(&sut, resource);
        assert!(resource_preferences_has_resource_hidden(
            sut.clone(),
            resource
        ));

        // Unhide the resource
        sut = resource_preferences_unhide_resource(&sut, resource);
        assert!(!resource_preferences_has_resource_hidden(
            sut.clone(),
            resource
        ));
    }
}
