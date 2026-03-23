use identified_vec_of::decl_identified_vec_of;
use std::fmt;

use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered collection of unique [`P2PTransportProfile`]s.
    /// The identifier of a profile is its signaling server URL.
    P2PTransportProfile
);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P2PStunServer {
    /// STUN URL list. Supported URL scheme is `stun:`.
    pub urls: Vec<String>,
}

impl P2PStunServer {
    pub fn new(urls: Vec<String>) -> Self {
        Self { urls }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P2PTurnServer {
    /// TURN URL list. Supported URL scheme is `turn:`.
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub credential: Option<String>,
}

impl P2PTurnServer {
    pub fn new(
        urls: Vec<String>,
        username: Option<String>,
        credential: Option<String>,
    ) -> Self {
        Self {
            urls,
            username,
            credential,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P2PTransportProfile {
    /// User-facing label of the profile.
    pub name: String,
    /// Base URL of signaling server, e.g. wss://signaling-server.radixdlt.com/
    pub signaling_server: String,
    /// STUN server configuration used by WebRTC peer connections.
    pub stun: P2PStunServer,
    /// TURN server configuration used by WebRTC peer connections.
    pub turn: P2PTurnServer,
}

impl fmt::Display for P2PTransportProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.signaling_server)
    }
}

impl P2PTransportProfile {
    pub fn new(
        name: impl Into<String>,
        signaling_server: impl Into<String>,
        stun: P2PStunServer,
        turn: P2PTurnServer,
    ) -> Self {
        Self {
            name: name.into(),
            signaling_server: signaling_server.into(),
            stun,
            turn,
        }
    }
}

impl Identifiable for P2PTransportProfile {
    type ID = String;

    fn id(&self) -> String {
        self.signaling_server.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedP2PTransportProfiles {
    pub current: P2PTransportProfile,
    #[serde(default)]
    pub other: P2PTransportProfiles,
}

impl SavedP2PTransportProfiles {
    pub fn new(current: P2PTransportProfile) -> Self {
        Self {
            current,
            other: P2PTransportProfiles::default(),
        }
    }

    pub fn all(&self) -> Vec<P2PTransportProfile> {
        let mut all = Vec::new();
        all.push(self.current.clone());
        all.append(&mut self.other.items());
        all
    }

    pub fn all_available_for_selection(&self) -> Vec<P2PTransportProfile> {
        self.all()
            .into_iter()
            .filter(|profile| {
                !P2PTransportProfile::is_radix_development_signaling_server(
                    &profile.signaling_server,
                )
            })
            .collect()
    }

    pub fn has_signaling_server(&self, url: impl AsRef<str>) -> bool {
        let signaling_server = url.as_ref();
        self.all()
            .into_iter()
            .any(|p| p.id().as_str() == signaling_server)
    }

    pub fn append(&mut self, profile: P2PTransportProfile) -> bool {
        if self.current.id() == profile.id()
            || self.other.contains_by_id(&profile)
        {
            return false;
        }
        self.other.try_insert_unique(profile).is_ok()
    }

    pub fn remove(&mut self, profile: &P2PTransportProfile) -> bool {
        self.other.remove_id(&profile.id()).is_some()
    }

    pub fn change_current(&mut self, to: P2PTransportProfile) -> bool {
        if self.current.id() == to.id() {
            return false;
        }

        let old_current = self.current.clone();
        self.other.remove_id(&to.id());
        self.current = to;

        if !self.other.contains_by_id(&old_current) {
            let _ = self.other.try_insert_unique(old_current);
        }
        true
    }
}

impl Default for SavedP2PTransportProfiles {
    fn default() -> Self {
        Self {
            current: P2PTransportProfile::default_production_profile(),
            other: P2PTransportProfiles::from_iter([
                P2PTransportProfile::default_development_profile(),
            ]),
        }
    }
}

impl P2PTransportProfile {
    const RADIX_DEVELOPMENT_SIGNALING_SERVER: &'static str =
        "wss://signaling-server-dev.rdx-works-main.extratools.works/";

    pub fn is_radix_development_signaling_server(
        url: impl AsRef<str>,
    ) -> bool {
        url.as_ref().trim_end_matches('/')
            == Self::RADIX_DEVELOPMENT_SIGNALING_SERVER.trim_end_matches('/')
    }

    fn google_stun_servers() -> P2PStunServer {
        P2PStunServer::new(vec![
            "stun:stun.l.google.com:19302".to_string(),
            "stun:stun1.l.google.com:19302".to_string(),
            "stun:stun2.l.google.com:19302".to_string(),
            "stun:stun3.l.google.com:19302".to_string(),
            "stun:stun4.l.google.com:19302".to_string(),
        ])
    }

    fn default_production_profile() -> Self {
        let stun = Self::google_stun_servers();
        let turn = P2PTurnServer::new(
            vec![
                "turn:turn-udp.radixdlt.com:80?transport=udp".to_string(),
                "turn:turn-tcp.radixdlt.com:80?transport=tcp".to_string(),
            ],
            Some("username".to_string()),
            Some("password".to_string()),
        );

        Self::new(
            "Radix Production",
            "wss://signaling-server.radixdlt.com/",
            stun,
            turn,
        )
    }

    fn default_development_profile() -> Self {
        let stun = Self::google_stun_servers();
        let turn = P2PTurnServer::new(
            vec![
                "turn:turn-dev-udp.rdx-works-main.extratools.works:80?transport=udp".to_string(),
                "turn:turn-dev-tcp.rdx-works-main.extratools.works:80?transport=tcp".to_string(),
            ],
            Some("username".to_string()),
            Some("password".to_string()),
        );

        Self::new(
            "Radix Development",
            Self::RADIX_DEVELOPMENT_SIGNALING_SERVER,
            stun,
            turn,
        )
    }
}

impl HasSampleValues for P2PStunServer {
    fn sample() -> Self {
        Self::new(vec!["stun:stun.l.google.com:19302".to_string()])
    }

    fn sample_other() -> Self {
        Self::new(vec!["stun:stun1.l.google.com:19302".to_string()])
    }
}

impl HasSampleValues for P2PTurnServer {
    fn sample() -> Self {
        Self::new(
            vec!["turn:turn-udp.radixdlt.com:80?transport=udp".to_string()],
            Some("username".to_string()),
            Some("password".to_string()),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            vec!["turn:turn-tcp.radixdlt.com:80?transport=tcp".to_string()],
            Some("username".to_string()),
            Some("password".to_string()),
        )
    }
}

impl HasSampleValues for P2PTransportProfile {
    fn sample() -> Self {
        Self::default_production_profile()
    }

    fn sample_other() -> Self {
        Self::default_development_profile()
    }
}

impl HasSampleValues for SavedP2PTransportProfiles {
    fn sample() -> Self {
        Self {
            current: P2PTransportProfile::sample(),
            other: P2PTransportProfiles::from_iter([
                P2PTransportProfile::sample_other(),
            ]),
        }
    }

    fn sample_other() -> Self {
        Self {
            current: P2PTransportProfile::sample_other(),
            other: P2PTransportProfiles::from_iter([
                P2PTransportProfile::sample(),
            ]),
        }
    }
}

impl HasSampleValues for P2PTransportProfiles {
    fn sample() -> Self {
        Self::from_iter([P2PTransportProfile::sample_other()])
    }

    fn sample_other() -> Self {
        Self::from_iter([P2PTransportProfile::sample()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SavedP2PTransportProfiles;

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
    fn change_current_swaps_current_into_other() {
        let mut sut = SUT::default();
        let production = sut.current.clone();
        let development = sut.other.items().first().cloned().unwrap();

        assert!(sut.change_current(development.clone()));
        assert_eq!(sut.current, development);
        assert!(sut.other.contains_by_id(&production));
    }

    #[test]
    fn all_available_for_selection_filters_out_radix_development_profile() {
        let sut = SUT::default();
        let all = sut.all_available_for_selection();

        assert_eq!(all.len(), 1);
        assert_eq!(all[0].name, "Radix Production");
        assert_eq!(all[0].signaling_server, "wss://signaling-server.radixdlt.com/");
    }

    #[test]
    fn all_available_for_selection_filters_out_current_radix_development_profile(
    ) {
        let sut = SUT::sample_other();
        let all = sut.all_available_for_selection();

        assert_eq!(all.len(), 1);
        assert_eq!(all[0].name, "Radix Production");
        assert_eq!(all[0].signaling_server, "wss://signaling-server.radixdlt.com/");
    }
}
