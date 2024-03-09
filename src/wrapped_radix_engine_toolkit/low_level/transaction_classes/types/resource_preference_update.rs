use crate::prelude::*;

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

impl HasSampleValues for ResourcePreferenceUpdate {
    fn sample() -> Self {
        Self::Set {
            value: ResourcePreference::Allowed,
        }
    }

    fn sample_other() -> Self {
        Self::Remove
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourcePreferenceUpdate;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_ret() {
        assert_eq!(
            SUT::from(RetUpdate::<RetResourcePreference>::Set(
                RetResourcePreference::Allowed
            )),
            SUT::sample()
        );

        assert_eq!(
            SUT::from(RetUpdate::<RetResourcePreference>::Remove),
            SUT::sample_other()
        );
    }
}
