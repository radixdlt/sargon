use crate::prelude::*;
use sargon::ResourcePreferences as InternalResourcePreferences;

#[uniffi::export]
pub fn resource_preferences_get_hidden_resources(
    resource_preferences: Vec<ResourceAppPreference>,
) -> Vec<ResourceIdentifier> {
    let resource_preferences: InternalResourcePreferences =
        resource_preferences.into_internal();
    resource_preferences.get_hidden_resources().into_type()
}

#[uniffi::export]
pub fn resource_preferences_hide_resource(
    resource_preferences: Vec<ResourceAppPreference>,
    resource: ResourceIdentifier,
) -> Vec<ResourceAppPreference> {
    let mut resource_preferences: InternalResourcePreferences =
        resource_preferences.into_internal();
    resource_preferences.hide_resource(resource.into());
    resource_preferences.into_type()
}

#[uniffi::export]
pub fn resource_preferences_unhide_resource(
    resource_preferences: Vec<ResourceAppPreference>,
    resource: ResourceIdentifier,
) -> Vec<ResourceAppPreference> {
    let mut resource_preferences: InternalResourcePreferences =
        resource_preferences.into_internal();
    resource_preferences.unhide_resource(resource.into());
    resource_preferences.into_type()
}
