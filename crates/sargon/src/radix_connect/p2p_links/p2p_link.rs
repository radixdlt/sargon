use crate::prelude::*;

/// A client the user have connected P2P with, typically a WebRTC connection with the dApp or Connector Extension.
/// Each client generates a curve25119 keypair. The public key is used as an identifier for the client.
/// The hash of the connection password is used to establish the P2P connection.
/// There can be multiple types of links (trusted vs untrusted) differentiated by `RadixConnectPurpose`.
/// Here are the [CAP-36][doc] requirements.
///
/// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3251863610/CAP-36+WebRTC+Clients+Protocol
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Debug,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[debug(
    "P2PLink {{ display_name: '{display_name}', connection_password: '{connection_password}', connection_purpose: '{connection_purpose}', public_key: '{public_key}' }}"
)]
#[display("{}", self.to_obfuscated_string())]
pub struct P2PLink {
    /// The most important property of this struct, the `RadixConnectPassword`,
    /// is used to be able to re-establish the P2P connection
    pub connection_password: RadixConnectPassword,

    /// The purpose of the connection, set by the other client, typically Connector Extension or dApp.
    /// As part of the initial linking flow, user will be prompted about kind of link they're trying to make.
    /// The user needs to make a conscious decision about general purpose links (because it comes with security risk).
    pub connection_purpose: RadixConnectPurpose,

    /// Each client generates a curve25119 keypair. The public key will be used as an identifier for the client.
    /// Each client keeps a record of linked clients' public keys to prevent duplicate links.
    /// This is the public key of the other client and it also serves as the seed for the link `ID`.
    pub public_key: Ed25519PublicKey,

    /// Client name, e.g. "Chrome on Macbook" or "My work Android" or "My wifes iPhone SE".
    pub display_name: String,
}

impl SafeToLog for P2PLink {
    fn non_sensitive(&self) -> impl std::fmt::Debug {
        self.to_obfuscated_string()
    }
}

impl P2PLink {
    pub fn new(
        connection_password: RadixConnectPassword,
        connection_purpose: RadixConnectPurpose,
        public_key: Ed25519PublicKey,
        display_name: String,
    ) -> Self {
        Self {
            connection_password,
            connection_purpose,
            public_key,
            display_name,
        }
    }
}

impl P2PLink {
    pub fn to_obfuscated_string(&self) -> String {
        format!(
            "P2PLink( name: '{}', password: <OMITTED>)",
            self.display_name
        )
    }
}

impl Identifiable for P2PLink {
    type ID = PublicKeyHash;

    fn id(&self) -> PublicKeyHash {
        PublicKeyHash::hash(self.public_key)
    }
}

impl P2PLink {
    pub fn connection_password(&self) -> RadixConnectPassword {
        self.connection_password
    }
}

impl HasSampleValues for P2PLink {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_chrome()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_brave()
    }
}

impl P2PLink {
    fn declare(
        password: RadixConnectPassword,
        purpose: RadixConnectPurpose,
        public_key: Ed25519PublicKey,
        display: &str,
    ) -> Self {
        Self::new(password, purpose, public_key, display.to_string())
    }

    /// `aced`... "Arc on MacStudio"
    /// A sample used to facilitate unit tests.
    pub fn sample_arc() -> Self {
        Self::declare(
            RadixConnectPassword::sample_aced(),
            RadixConnectPurpose::sample(),
            Ed25519PublicKey::sample(),
            "Arc on MacStudio",
        )
    }

    /// `babe`... "Brave on PC"
    /// A sample used to facilitate unit tests.
    pub fn sample_brave() -> Self {
        Self::declare(
            RadixConnectPassword::sample_babe(),
            RadixConnectPurpose::sample_other(),
            Ed25519PublicKey::sample_other(),
            "Brave on PC",
        )
    }

    /// `cafe`... "Chrome on Macbook"
    /// A sample used to facilitate unit tests.
    pub fn sample_chrome() -> Self {
        Self::declare(
            RadixConnectPassword::sample_cafe(),
            RadixConnectPurpose::sample(),
            Ed25519PublicKey::sample_fade(),
            "Chrome on Macbook",
        )
    }

    /// `dead`... "DuckDuckGo on Mac Pro"
    /// A sample used to facilitate unit tests.
    pub fn sample_duckduckgo() -> Self {
        Self::declare(
            RadixConnectPassword::sample_dead(),
            RadixConnectPurpose::sample_other(),
            Ed25519PublicKey::sample_aced(),
            "DuckDuckGo on Mac Pro",
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(P2PLink::sample(), P2PLink::sample());
        assert_eq!(P2PLink::sample_other(), P2PLink::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(P2PLink::sample(), P2PLink::sample_other());
    }

    #[test]
    fn json_roundtrip() {
        let sut = P2PLink::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "connectionPassword": "cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe",
                "connectionPurpose": "general",
                "publicKey": "37842830eca0d08dd684adcb9705b3a473c0c070a322322b53c35e09a1bff298",
                "displayName": "Chrome on Macbook"
            }
            "#,
        )
    }

    #[test]
    fn display_name() {
        assert_eq!(P2PLink::sample().display_name, "Chrome on Macbook");
    }

    #[test]
    fn connection_password() {
        assert_eq!(
            P2PLink::sample().connection_password().hash().to_string(),
            "98e140d9c01c069aa927797627b1bca4d25971a76549ca59df8ef9d8397afa97"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", P2PLink::sample()), "P2PLink { display_name: 'Chrome on Macbook', connection_password: 'cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe', connection_purpose: 'general', public_key: '37842830eca0d08dd684adcb9705b3a473c0c070a322322b53c35e09a1bff298' }");
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", P2PLink::sample()),
            "P2PLink( name: 'Chrome on Macbook', password: <OMITTED>)"
        );
    }

    #[test]
    fn safe_to_log() {
        let sut = P2PLink::sample();
        assert_eq!(
            format!("{:?}", sut.to_string()),
            format!("{:?}", sut.non_sensitive())
        );
    }
}
