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
pub struct P2PIceServer {
    /// ICE URL list. Supported URL schemes are `stun:` and `turn:`.
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub credential: Option<String>,
}

impl P2PIceServer {
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
    pub signaling_server: Url,
    /// Full ICE server configuration used by WebRTC peer connections.
    pub ice_servers: Vec<P2PIceServer>,
}

impl fmt::Display for P2PTransportProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.signaling_server)
    }
}

impl P2PTransportProfile {
    pub fn new(
        name: impl Into<String>,
        signaling_server: Url,
        ice_servers: Vec<P2PIceServer>,
    ) -> Self {
        Self {
            name: name.into(),
            signaling_server,
            ice_servers,
        }
    }
}

impl Identifiable for P2PTransportProfile {
    type ID = Url;

    fn id(&self) -> Url {
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

    pub fn has_signaling_server(&self, url: Url) -> bool {
        self.all().into_iter().any(|p| p.id() == url)
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
    fn google_stun_servers() -> Vec<P2PIceServer> {
        vec![
            P2PIceServer::new(
                vec!["stun:stun.l.google.com:19302".to_string()],
                None,
                None,
            ),
            P2PIceServer::new(
                vec!["stun:stun1.l.google.com:19302".to_string()],
                None,
                None,
            ),
            P2PIceServer::new(
                vec!["stun:stun2.l.google.com:19302".to_string()],
                None,
                None,
            ),
            P2PIceServer::new(
                vec!["stun:stun3.l.google.com:19302".to_string()],
                None,
                None,
            ),
            P2PIceServer::new(
                vec!["stun:stun4.l.google.com:19302".to_string()],
                None,
                None,
            ),
        ]
    }

    fn default_production_profile() -> Self {
        let mut ice_servers = Self::google_stun_servers();
        ice_servers.push(P2PIceServer::new(
            vec!["turn:turn-udp.radixdlt.com:80?transport=udp".to_string()],
            Some("username".to_string()),
            Some("password".to_string()),
        ));
        ice_servers.push(P2PIceServer::new(
            vec!["turn:turn-tcp.radixdlt.com:80?transport=tcp".to_string()],
            Some("username".to_string()),
            Some("password".to_string()),
        ));

        Self::new(
            "Radix Production",
            Url::parse("wss://signaling-server.radixdlt.com/")
                .expect("valid URL"),
            ice_servers,
        )
    }

    fn default_development_profile() -> Self {
        let mut ice_servers = Self::google_stun_servers();
        ice_servers.push(P2PIceServer::new(
            vec!["turn:turn-dev-udp.rdx-works-main.extratools.works:80?transport=udp".to_string()],
            Some("username".to_string()),
            Some("password".to_string()),
        ));
        ice_servers.push(P2PIceServer::new(
            vec!["turn:turn-dev-tcp.rdx-works-main.extratools.works:80?transport=tcp".to_string()],
            Some("username".to_string()),
            Some("password".to_string()),
        ));

        Self::new(
            "Radix Development",
            Url::parse(
                "wss://signaling-server-dev.rdx-works-main.extratools.works/",
            )
            .expect("valid URL"),
            ice_servers,
        )
    }
}

impl HasSampleValues for P2PIceServer {
    fn sample() -> Self {
        Self::new(vec!["stun:stun.l.google.com:19302".to_string()], None, None)
    }

    fn sample_other() -> Self {
        Self::new(
            vec!["turn:turn-udp.radixdlt.com:80?transport=udp".to_string()],
            Some("username".to_string()),
            Some("password".to_string()),
        )
    }
}

impl HasSampleValues for P2PTransportProfile {
    fn sample() -> Self {
        Self::new(
            "Sample Production",
            Url::parse("wss://signaling-server.radixdlt.com/")
                .expect("valid URL"),
            vec![],
        )
    }

    fn sample_other() -> Self {
        Self::new(
            "Sample Development",
            Url::parse(
                "wss://signaling-server-dev.rdx-works-main.extratools.works/",
            )
            .expect("valid URL"),
            vec![],
        )
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
}
