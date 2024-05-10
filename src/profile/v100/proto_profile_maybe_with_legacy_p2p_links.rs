use crate::prelude::*;

/// This proto-Profile model has one single purpose - facilitate the query:
/// "does this profile contain legacy links", in a type-safe manner, using
/// the fact that the `p2pLinks` were only "3 levels" deep into Profile,
/// we can thus try deserialize JSON into this proto-Profile and check if
/// the proto-p2plinks collection is empty or not.
#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct ProtoProfileMaybeWithLegacyP2PLinks {
    /// Proto-AppPreferences model
    #[serde(rename = "appPreferences")]
    pub app_preferences: ProtoAppPreferencesMaybeWithLegacyP2PLinks,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct ProtoAppPreferencesMaybeWithLegacyP2PLinks {
    /// Proto-P2PLinks model
    #[serde(rename = "p2pLinks")]
    pub p2p_links: Vec<ProtoDummyLinkMaybeWithLegacyP2PLinks>,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct ProtoDummyLinkMaybeWithLegacyP2PLinks {
    /// A legacy property used here to facilitate unit tests.
    #[serde(rename = "displayName")]
    pub display_name: DisplayName,
}

impl HasSampleValues for ProtoDummyLinkMaybeWithLegacyP2PLinks {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self {
            display_name: DisplayName::sample(),
        }
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self {
            display_name: DisplayName::sample_other(),
        }
    }
}

impl HasSampleValues for ProtoAppPreferencesMaybeWithLegacyP2PLinks {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self {
            p2p_links: vec![ProtoDummyLinkMaybeWithLegacyP2PLinks::sample()],
        }
    }

    /// A sample used to facilitate unit tests.
    #[rustfmt::skip]
    fn sample_other() -> Self {
        Self {
            p2p_links: vec![ProtoDummyLinkMaybeWithLegacyP2PLinks::sample_other()],
        }
    }
}

#[rustfmt::skip]
impl HasSampleValues for ProtoProfileMaybeWithLegacyP2PLinks {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self {
            app_preferences: ProtoAppPreferencesMaybeWithLegacyP2PLinks::sample(),
        }
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self {
            app_preferences: ProtoAppPreferencesMaybeWithLegacyP2PLinks::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests_profile {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProtoProfileMaybeWithLegacyP2PLinks;

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }
}

#[cfg(test)]
mod tests_dummy_link {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProtoDummyLinkMaybeWithLegacyP2PLinks;

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }
}

#[cfg(test)]
mod tests_app_preferences {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProtoAppPreferencesMaybeWithLegacyP2PLinks;

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }
}
