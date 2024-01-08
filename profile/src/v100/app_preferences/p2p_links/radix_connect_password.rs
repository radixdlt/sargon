use std::fmt::{Debug, Formatter};

use crate::{hash, Hex32Bytes};
use radix_engine_common::crypto::Hash;
use serde::{Deserialize, Serialize};

use crate::HasPlaceholder;

/// The hash of the connection password is used to connect to the Radix Connect Signaling Server,
/// over web sockets. The actual `ConnectionPassword` is used to encrypt all messages sent via
/// the Signaling Server.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, uniffi::Record)]
#[serde(transparent)]
pub struct RadixConnectPassword {
    pub value: Hex32Bytes,
}

impl Debug for RadixConnectPassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.to_hex(),)
    }
}

impl RadixConnectPassword {
    pub fn new(hex_32bytes: Hex32Bytes) -> Self {
        Self {
            value: hex_32bytes.into(),
        }
    }

    pub fn hash(&self) -> Hash {
        hash(self.value.bytes())
    }
}

impl HasPlaceholder for RadixConnectPassword {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new(Hex32Bytes::placeholder())
    }

    fn placeholder_other() -> Self {
        Self::new(Hex32Bytes::placeholder_other())
    }
}

impl RadixConnectPassword {
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
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip, HasPlaceholder,
    };
    use radix_engine_common::prelude::HashSet;
    use serde_json::json;

    use super::RadixConnectPassword;

    #[test]
    fn equality() {
        assert_eq!(
            RadixConnectPassword::placeholder(),
            RadixConnectPassword::placeholder()
        );
        assert_eq!(
            RadixConnectPassword::placeholder_other(),
            RadixConnectPassword::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            RadixConnectPassword::placeholder(),
            RadixConnectPassword::placeholder_other()
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", RadixConnectPassword::placeholder()),
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"
        );
    }

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
