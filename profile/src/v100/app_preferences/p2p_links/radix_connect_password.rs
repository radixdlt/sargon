use crate::prelude::*;
use radix_engine_common::crypto::Hash;

/// The hash of the connection password is used to connect to the Radix Connect Signaling Server,
/// over web sockets. The actual `ConnectionPassword` is used to encrypt all messages sent via
/// the Signaling Server.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[serde(transparent)]
#[debug("{value}")]
pub struct RadixConnectPassword {
    pub value: Hex32Bytes,
}

impl RadixConnectPassword {
    pub fn new(hex_32bytes: Hex32Bytes) -> Self {
        Self { value: hex_32bytes }
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
    use crate::prelude::*;

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
    fn display() {
        assert_eq!(
            format!("{}", RadixConnectPassword::placeholder()),
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
            HashSet::<RadixConnectPassword>::from_iter([
                RadixConnectPassword::placeholder(),
                RadixConnectPassword::placeholder_dead()
            ])
            .len(),
            1
        );

        assert_eq!(
            HashSet::<RadixConnectPassword>::from_iter([
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
