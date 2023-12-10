use identified_vec::Identifiable;
use radix_engine_common::crypto::Hash;
use serde::{Deserialize, Serialize};

use super::radix_connect_password::RadixConnectPassword;

/// A client the user have connected P2P with, typically a
/// WebRTC connections with a DApp, but might be Android or iPhone
/// client as well.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "camelCase")]
pub struct P2PLink {
    /// The most important property of this struct, the `ConnectionPassword`,
    /// is used to be able to re-establish the P2P connection and also acts as the seed
    /// for the `ID`.
    pub connection_password: RadixConnectPassword,

    /// Client name, e.g. "Chrome on Macbook" or "My work Android" or "My wifes iPhone SE".
    pub display_name: String,
}

impl Identifiable for P2PLink {
    type ID = Hash;

    fn id(&self) -> Self::ID {
        self.connection_password.hash()
    }
}

impl P2PLink {
    pub fn new(password: RadixConnectPassword, name: &str) -> Self {
        Self {
            connection_password: password,
            display_name: name.to_string(),
        }
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::placeholder_chrome()
    }

    /// `deadbeef`... "Chrome on Macbook"
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_chrome() -> Self {
        Self::new(
            RadixConnectPassword::placeholder_deadbeef(),
            "Chrome on Macbook",
        )
    }

    /// `fadedeaf`... "Brave on PC"
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_brave() -> Self {
        Self::new(RadixConnectPassword::placeholder_fadedeaf(), "Brave on PC")
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::P2PLink;

    #[test]
    fn json_roundtrip() {
        let sut = P2PLink::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "connectionPassword": "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                "displayName": "Chrome on Macbook"
            }
            "#,
        )
    }
}
