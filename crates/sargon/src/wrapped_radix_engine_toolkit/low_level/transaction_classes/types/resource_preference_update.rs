use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResourcePreference {
    Allowed,
    Disallowed,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResourcePreferenceUpdate {
    Set { value: ResourcePreference },
    Remove,
}

impl From<ScryptoResourcePreference> for ResourcePreference {
    fn from(value: ScryptoResourcePreference) -> Self {
        match value {
            ScryptoResourcePreference::Allowed => Self::Allowed,
            ScryptoResourcePreference::Disallowed => Self::Disallowed,
        }
    }
}

impl From<RetUpdate<ScryptoResourcePreference>> for ResourcePreferenceUpdate {
    fn from(value: RetUpdate<ScryptoResourcePreference>) -> Self {
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
            SUT::from(RetUpdate::<ScryptoResourcePreference>::Set(
                ScryptoResourcePreference::Allowed
            )),
            SUT::sample()
        );

        assert_eq!(
            SUT::from(RetUpdate::<ScryptoResourcePreference>::Set(
                ScryptoResourcePreference::Disallowed
            )),
            SUT::Set {
                value: ResourcePreference::Disallowed
            }
        );

        assert_eq!(
            SUT::from(RetUpdate::<ScryptoResourcePreference>::Remove),
            SUT::sample_other()
        );
    }
}
