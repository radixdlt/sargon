use crate::prelude::*;

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
