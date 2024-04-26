use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize
)]
pub struct ProtoProfileMaybeWithLegacyP2PLinks {
    #[serde(rename = "appPreferences")]
    pub app_preferences: ProtoAppPreferencesMaybeWithLegacyP2PLinks,
}

impl ProtoProfileMaybeWithLegacyP2PLinks {
    pub fn with(
        app_preferences: ProtoAppPreferencesMaybeWithLegacyP2PLinks,
    ) -> Self {
        Self {
            app_preferences,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ProtoAppPreferencesMaybeWithLegacyP2PLinks {
    #[serde(rename = "p2pLinks")]
    pub p2p_links: Vec<ProtoDummyLinkMaybeWithLegacyP2PLinks>,
}

impl ProtoAppPreferencesMaybeWithLegacyP2PLinks {
    pub fn with(
        p2p_links: Vec<ProtoDummyLinkMaybeWithLegacyP2PLinks>,
    ) -> Self {
        Self {
            p2p_links,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ProtoDummyLinkMaybeWithLegacyP2PLinks {
    // I think it is fine with an empty object
}

impl HasSampleValues for ProtoDummyLinkMaybeWithLegacyP2PLinks {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self { }
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self { }
    }
}

impl HasSampleValues for ProtoAppPreferencesMaybeWithLegacyP2PLinks {
    fn sample() -> Self {
        Self::with(Vec::from([ProtoDummyLinkMaybeWithLegacyP2PLinks::sample()]))
    }

    fn sample_other() -> Self {
        Self::with(Vec::from([ProtoDummyLinkMaybeWithLegacyP2PLinks::sample_other()]))
    }
}

impl HasSampleValues for ProtoProfileMaybeWithLegacyP2PLinks {
    fn sample() -> Self {
        Self::with(ProtoAppPreferencesMaybeWithLegacyP2PLinks::sample(),)
    }

    fn sample_other() -> Self {
        Self::with(ProtoAppPreferencesMaybeWithLegacyP2PLinks::sample_other(),)
    }
}