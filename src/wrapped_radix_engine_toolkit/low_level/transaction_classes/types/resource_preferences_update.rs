use radix_engine_interface::blueprints::account::ResourcePreference as RetResourcePreference;
use radix_engine_toolkit::transaction_types::Update as RetUpdate;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum ResourcePreference {
    Allowed,
    Disallowed,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum ResourcePreferenceUpdate {
    Set { value: ResourcePreference },
    Remove,
}

impl From<RetResourcePreference> for ResourcePreference {
    fn from(value: RetResourcePreference) -> Self {
        match value {
            RetResourcePreference::Allowed => Self::Allowed,
            RetResourcePreference::Disallowed => Self::Disallowed,
        }
    }
}

impl From<RetUpdate<RetResourcePreference>> for ResourcePreferenceUpdate {
    fn from(value: RetUpdate<RetResourcePreference>) -> Self {
        match value {
            RetUpdate::Set(preference) => Self::Set {
                value: preference.into(),
            },
            RetUpdate::Remove => Self::Remove,
        }
    }
}
