use crate::prelude::*;

use radix_engine_common::crypto::Hash;

/// A client the user have connected P2P with, typically a
/// WebRTC connections with a DApp, but might be Android or iPhone
/// client as well.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[debug(
    "P2PLink {{ display_name: '{display_name}', connection_password: '{connection_password}' }}"
)]
#[display("{}", self.to_obfuscated_string())]
pub struct P2PLink {
    /// The most important property of this struct, the `ConnectionPassword`,
    /// is used to be able to re-establish the P2P connection and also acts as the seed
    /// for the `ID`.
    pub connection_password: RadixConnectPassword,

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
        display_name: String,
    ) -> Self {
        Self {
            connection_password,
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
    type ID = Hash;

    fn id(&self) -> Self::ID {
        self.connection_password.hash()
    }
}

impl P2PLink {
    pub fn connection_password(&self) -> RadixConnectPassword {
        self.connection_password.clone()
    }
}

impl HasPlaceholder for P2PLink {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_chrome()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_brave()
    }
}

impl P2PLink {
    fn declare(password: RadixConnectPassword, display: &str) -> Self {
        Self::new(password, display.to_string())
    }

    /// `aced`... "Arc on MacStudio"
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_arc() -> Self {
        Self::declare(
            RadixConnectPassword::placeholder_aced(),
            "Arc on MacStudio",
        )
    }

    /// `babe`... "Brave on PC"
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_brave() -> Self {
        Self::declare(RadixConnectPassword::placeholder_babe(), "Brave on PC")
    }

    /// `cafe`... "Chrome on Macbook"
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_chrome() -> Self {
        Self::declare(
            RadixConnectPassword::placeholder_cafe(),
            "Chrome on Macbook",
        )
    }

    /// `dead`... "DuckDuckGo on Mac Pro"
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_duckduckgo() -> Self {
        Self::declare(
            RadixConnectPassword::placeholder_dead(),
            "DuckDuckGo on Mac Pro",
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(P2PLink::placeholder(), P2PLink::placeholder());
        assert_eq!(P2PLink::placeholder_other(), P2PLink::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(P2PLink::placeholder(), P2PLink::placeholder_other());
    }

    #[test]
    fn json_roundtrip() {
        let sut = P2PLink::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "connectionPassword": "cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe",
                "displayName": "Chrome on Macbook"
            }
            "#,
        )
    }

    #[test]
    fn display_name() {
        assert_eq!(P2PLink::placeholder().display_name, "Chrome on Macbook");
    }

    #[test]
    fn connection_password() {
        assert_eq!(
            P2PLink::placeholder()
                .connection_password()
                .hash()
                .to_string(),
            "98e140d9c01c069aa927797627b1bca4d25971a76549ca59df8ef9d8397afa97"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", P2PLink::placeholder()), "P2PLink { display_name: 'Chrome on Macbook', connection_password: 'cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe' }");
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", P2PLink::placeholder()),
            "P2PLink( name: 'Chrome on Macbook', password: <OMITTED>)"
        );
    }

    #[test]
    fn safe_to_log() {
        let sut = P2PLink::placeholder();
        assert_eq!(
            format!("{:?}", sut.to_string()),
            format!("{:?}", sut.non_sensitive())
        );
    }
}
