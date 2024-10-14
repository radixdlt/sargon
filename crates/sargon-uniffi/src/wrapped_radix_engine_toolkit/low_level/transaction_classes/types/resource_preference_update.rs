use crate::prelude::*;
use sargon::ResourcePreference as InternalResourcePreference;
use sargon::ResourcePreferenceUpdate as InternalResourcePreferenceUpdate;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum ResourcePreference {
    Allowed,
    Disallowed,
}

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum ResourcePreferenceUpdate {
    Set { value: ResourcePreference },
    Remove,
}