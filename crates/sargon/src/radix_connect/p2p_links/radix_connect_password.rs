use crate::prelude::*;

json_string_convertible!(RadixConnectPassword);

/// The hash of the connection password is used to connect to the Radix Connect Signaling Server,
/// over web sockets. The actual `ConnectionPassword` is used to encrypt all messages sent via
/// the Signaling Server.
#[derive(
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
#[serde(transparent)]
#[debug("{value}")]
pub struct RadixConnectPassword {
    pub value: Exactly32Bytes,
}

impl RadixConnectPassword {
    pub fn new(hex_32bytes: Exactly32Bytes) -> Self {
        Self { value: hex_32bytes }
    }

    pub fn hash(&self) -> Hash {
        hash_of(self.value.bytes())
    }

    /// Represents the message to be signed and sent to Connector Extension.
    /// Connector Extension uses the same logic to compute its own message.
    pub fn message_hash(&self) -> Hash {
        let message = ["L".as_bytes(), self.value.bytes()].concat();
        hash_of(message)
    }
}

impl HasSampleValues for RadixConnectPassword {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(Exactly32Bytes::sample())
    }

    fn sample_other() -> Self {
        Self::new(Exactly32Bytes::sample_other())
    }
}

impl RadixConnectPassword {
    /// A sample used to facilitate unit tests.
    pub fn sample_aced() -> Self {
        Self::new(Exactly32Bytes::sample_aced())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_babe() -> Self {
        Self::new(Exactly32Bytes::sample_babe())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_cafe() -> Self {
        Self::new(Exactly32Bytes::sample_cafe())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_dead() -> Self {
        Self::new(Exactly32Bytes::sample_dead())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_ecad() -> Self {
        Self::new(Exactly32Bytes::sample_ecad())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_fade() -> Self {
        Self::new(Exactly32Bytes::sample_fade())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            RadixConnectPassword::sample(),
            RadixConnectPassword::sample()
        );
        assert_eq!(
            RadixConnectPassword::sample_other(),
            RadixConnectPassword::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            RadixConnectPassword::sample(),
            RadixConnectPassword::sample_other()
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", RadixConnectPassword::sample()),
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", RadixConnectPassword::sample()),
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = RadixConnectPassword::sample();

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
                RadixConnectPassword::sample(),
                RadixConnectPassword::sample_dead()
            ])
            .len(),
            1
        );

        assert_eq!(
            HashSet::<RadixConnectPassword>::from_iter([
                RadixConnectPassword::sample_aced(),
                RadixConnectPassword::sample_babe(),
                RadixConnectPassword::sample_cafe(),
                RadixConnectPassword::sample_dead(),
                RadixConnectPassword::sample_ecad(),
                RadixConnectPassword::sample_fade(),
            ])
            .len(),
            6
        );
    }

    #[test]
    fn message_hash() {
        assert_eq!(
            RadixConnectPassword::sample().message_hash(),
            Exactly32Bytes::from_str("479ae13d3983de8ab520e519cfba01a25fafbbc1e7438ba52e5ed4a40cd2f56a").map(Hash::from).unwrap()
        );
    }
}