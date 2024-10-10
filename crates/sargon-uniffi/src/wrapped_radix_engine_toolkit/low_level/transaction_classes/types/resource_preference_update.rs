use crate::prelude::*;
use sargon::ResourcePreference as InternalResourcePreference;
use sargon::ResourcePreferenceUpdate as InternalResourcePreferenceUpdate;

#[derive(Clone,  PartialEq, Eq, Hash, uniffi::Enum)]
pub enum ResourcePreference {
    Allowed,
    Disallowed,
}

#[derive(Clone,  PartialEq, Eq, Hash, uniffi::Enum)]
pub enum ResourcePreferenceUpdate {
    Set { value: ResourcePreference },
    Remove,
}

impl From<InternalResourcePreference> for ResourcePreference {
    fn from(value: InternalResourcePreference) -> Self {
        match value {
            InternalResourcePreference::Allowed => ResourcePreference::Allowed,
            InternalResourcePreference::Disallowed => ResourcePreference::Disallowed,
        }
    }
}

impl Into<InternalResourcePreference> for ResourcePreference {
    fn into(self) -> InternalResourcePreference {
        match self {
            ResourcePreference::Allowed => InternalResourcePreference::Allowed,
            ResourcePreference::Disallowed => InternalResourcePreference::Disallowed,
        }
    }
}

impl From<InternalResourcePreferenceUpdate> for ResourcePreferenceUpdate {
    fn from(value: InternalResourcePreferenceUpdate) -> Self {
        match value {
            InternalResourcePreferenceUpdate::Set { value } => ResourcePreferenceUpdate::Set {
                value: value.into(),
            },
            InternalResourcePreferenceUpdate::Remove => ResourcePreferenceUpdate::Remove,
        }
    }
}

impl Into<InternalResourcePreferenceUpdate> for ResourcePreferenceUpdate {
    fn into(self) -> InternalResourcePreferenceUpdate {
        match self {
            ResourcePreferenceUpdate::Set { value } => InternalResourcePreferenceUpdate::Set {
                value: value.into(),
            },
            ResourcePreferenceUpdate::Remove => InternalResourcePreferenceUpdate::Remove,
        }
    }
}