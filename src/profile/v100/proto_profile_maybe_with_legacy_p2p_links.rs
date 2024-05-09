use crate::prelude::*;

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct ProtoProfileMaybeWithLegacyP2PLinks {
    #[serde(rename = "appPreferences")]
    pub app_preferences: ProtoAppPreferencesMaybeWithLegacyP2PLinks,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct ProtoAppPreferencesMaybeWithLegacyP2PLinks {
    #[serde(rename = "p2pLinks")]
    pub p2p_links: Vec<ProtoDummyLinkMaybeWithLegacyP2PLinks>,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct ProtoDummyLinkMaybeWithLegacyP2PLinks {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl HasSampleValues for ProtoDummyLinkMaybeWithLegacyP2PLinks {
    fn sample() -> Self {
        Self {
            display_name: String::from("Sample Display Name"),
        }
    }

    fn sample_other() -> Self {
        Self {
            display_name: String::from("Sample Display Name Other"),
        }
    }
}

impl HasSampleValues for ProtoAppPreferencesMaybeWithLegacyP2PLinks {
    fn sample() -> Self {
        Self {
            p2p_links: Vec::from([
                ProtoDummyLinkMaybeWithLegacyP2PLinks::sample(),
            ]),
        }
    }

    fn sample_other() -> Self {
        Self {
            p2p_links: Vec::from([
                ProtoDummyLinkMaybeWithLegacyP2PLinks::sample_other(),
            ]),
        }
    }
}

impl HasSampleValues for ProtoProfileMaybeWithLegacyP2PLinks {
    fn sample() -> Self {
        Self {
            app_preferences: ProtoAppPreferencesMaybeWithLegacyP2PLinks::sample(
            ),
        }
    }

    fn sample_other() -> Self {
        Self {
            app_preferences:
                ProtoAppPreferencesMaybeWithLegacyP2PLinks::sample_other(),
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
