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
        Self::new(Hex32Bytes::placeholder())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_aced() -> Self {
        Self::new(Hex32Bytes::placeholder_aced())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_babe() -> Self {
        Self::new(Hex32Bytes::placeholder_babe())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_cafe() -> Self {
        Self::new(Hex32Bytes::placeholder_cafe())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_dead() -> Self {
        Self::new(Hex32Bytes::placeholder_dead())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_ecad() -> Self {
        Self::new(Hex32Bytes::placeholder_ecad())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_fade() -> Self {
        Self::new(Hex32Bytes::placeholder_fade())
    }

    pub fn hash(&self) -> Hash {
        hash(self.0.bytes())
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_common::prelude::HashSet;
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
            json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"),
        );
        assert_json_roundtrip(&sut);
        assert_json_value_ne_after_roundtrip(
            &sut,
            json!("fadedeaffadedeaffadedeaffadedeaffadedeaffadedeaffadedeaffadedeaf"),
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::from_iter([
                RadixConnectPassword::placeholder(),
                RadixConnectPassword::placeholder_dead()
            ])
            .len(),
            1
        );

        assert_eq!(
            HashSet::from_iter([
                RadixConnectPassword::placeholder_aced(),
                RadixConnectPassword::placeholder_babe(),
                RadixConnectPassword::placeholder_cafe(),
                RadixConnectPassword::placeholder_dead(),
                RadixConnectPassword::placeholder_ecad(),
                RadixConnectPassword::placeholder_fade(),
            ])
            .len(),
            6
        );
    }
}
