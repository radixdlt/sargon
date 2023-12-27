use super::radix_connect_password::RadixConnectPassword;
use identified_vec::Identifiable;
use radix_engine_common::crypto::Hash;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter},
    ops::Deref,
    sync::Arc,
};

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

/// A client the user have connected P2P with, typically a
/// WebRTC connections with a DApp, but might be Android or iPhone
/// client as well.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, uniffi::Object)]
#[serde(rename_all = "camelCase")]
pub struct P2PLink {
    /// The most important property of this struct, the `ConnectionPassword`,
    /// is used to be able to re-establish the P2P connection and also acts as the seed
    /// for the `ID`.
    connection_password: RadixConnectPassword,

    /// Client name, e.g. "Chrome on Macbook" or "My work Android" or "My wifes iPhone SE".
    display_name: String,
}

impl Debug for P2PLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("P2PLink")
            .field("connection_password", &self.connection_password)
            .field("display_name", &self.display_name)
            .finish()
    }
}

impl Identifiable for P2PLink {
    type ID = Hash;

    fn id(&self) -> Self::ID {
        self.connection_password.hash()
    }
}

#[uniffi::export]
impl P2PLink {
    #[uniffi::constructor]
    pub fn new(password: Arc<RadixConnectPassword>, display_name: String) -> Arc<Self> {
        Arc::new(Self {
            connection_password: password.deref().clone(),
            display_name,
        })
    }

    pub fn get_connection_password(&self) -> Arc<RadixConnectPassword> {
        self.connection_password().into()
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }
}

impl P2PLink {
    pub fn connection_password(&self) -> RadixConnectPassword {
        self.connection_password.clone()
    }
}

#[cfg(any(test, feature = "placeholder"))]
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

#[cfg(any(test, feature = "placeholder"))]
impl P2PLink {
    fn declare(password: RadixConnectPassword, display: &str) -> Self {
        Self::new(password.into(), display.to_string())
            .deref()
            .clone()
    }

    /// `aced`... "Arc on MacStudio"
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_arc() -> Self {
        Self::declare(RadixConnectPassword::placeholder_aced(), "Arc on MacStudio")
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
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::P2PLink;

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
        assert_eq!(P2PLink::placeholder().display_name(), "Chrome on Macbook");
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
        assert_eq!(format!("{:?}", P2PLink::placeholder()), "P2PLink { connection_password: RadixConnectPassword(cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe), display_name: \"Chrome on Macbook\" }");
    }
}
