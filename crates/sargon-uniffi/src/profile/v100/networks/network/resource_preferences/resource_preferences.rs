use crate::prelude::*;
use sargon::ResourcePreferences as InternalResourcePreferences;

decl_identified_vec_of!(
    /// User off-ledger preferences regarding resources.
    ResourcePreferences,
    ResourceAppPreference
);

#[uniffi::export]
pub fn resource_preferences_get_hidden_resources(
    resource_preferences: ResourcePreferences,
) -> HiddenResources {
    resource_preferences.into_internal_vec().get_hidden_resources().into()
}

#[uniffi::export]
pub fn resource_preferences_hide_resource(
    resource_preferences: ResourcePreferences,
    resource: ResourceIdentifier,
) -> ResourcePreferences {
    let mut resource_preferences: InternalResourcePreferences = resource_preferences.into_internal_vec().clone();
    resource_preferences.hide_resource(resource.into());
    resource_preferences.into()
}

#[uniffi::export]
pub fn resource_preferences_unhide_resource(
    resource_preferences: ResourcePreferences,
    resource: ResourceIdentifier,
) -> ResourcePreferences {
    let mut resource_preferences:InternalResourcePreferences = resource_preferences.into_internal_vec().clone();
    resource_preferences.unhide_resource(resource.into());
    resource_preferences.into()
}

