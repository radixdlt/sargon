use radix_engine_common::crypto::Hash;
use serde::{Deserialize, Serialize};
use wallet_kit_common::{hash::hash, types::hex_32bytes::Hex32Bytes};

/// The hash of the connection password is used to connect to the Radix Connect Signaling Server,
/// over web sockets. The actual `ConnectionPassword` is used to encrypt all messages sent via
/// the Signaling Server.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct RadixConnectPassword(Hex32Bytes);

impl RadixConnectPassword {
    pub fn new(hex_32bytes: Hex32Bytes) -> Self {
        Self(hex_32bytes)
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::placeholder_deadbeef()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_deadbeef() -> Self {
        Self::new(Hex32Bytes::placeholder_deadbeef())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_fadedeaf() -> Self {
        Self::new(Hex32Bytes::placeholder_fadedeaf())
    }

    pub fn hash(&self) -> Hash {
        hash(self.0.bytes())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::json::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };

    use super::RadixConnectPassword;

    #[test]
    fn json_roundtrip() {
        let sut = RadixConnectPassword::placeholder();

        assert_json_value_eq_after_roundtrip(
            &sut,
            json!("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"),
        );
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(
            &sut,
            json!("fadedeaffadedeaffadedeaffadedeaffadedeaffadedeaffadedeaffadedeaf"),
        );
    }
}
