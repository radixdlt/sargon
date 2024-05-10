use crate::prelude::*;

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
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[debug(
    "LinkConnectionQRData {{ purpose: '{purpose}', password: '{password}', public_key: '{public_key}', signature: '{signature}' }}"
)]
#[display("{}", self.to_obfuscated_string())]
pub struct LinkConnectionQRData {
    pub purpose: RadixConnectPurpose,
    pub password: RadixConnectPassword,
    pub public_key: Ed25519PublicKey,
    pub signature: Exactly64Bytes,
}

impl LinkConnectionQRData {
    pub fn new(
        purpose: RadixConnectPurpose,
        password: RadixConnectPassword,
        public_key: Ed25519PublicKey,
        signature: Exactly64Bytes,
    ) -> Self {
        Self {
            purpose,
            password,
            public_key,
            signature,
        }
    }
}

impl LinkConnectionQRData {
    pub fn to_obfuscated_string(&self) -> String {
        format!("LinkConnectionQRData with purpose: '{}'", self.purpose)
    }
}

impl HasSampleValues for LinkConnectionQRData {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(
            RadixConnectPurpose::sample(),
            RadixConnectPassword::sample(),
            Ed25519PublicKey::sample(),
            Exactly64Bytes::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            RadixConnectPurpose::sample_other(),
            RadixConnectPassword::sample_other(),
            Ed25519PublicKey::sample_other(),
            Exactly64Bytes::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LinkConnectionQRData;

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
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::sample()),
            "LinkConnectionQRData { purpose: 'general', password: 'deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead', public_key: 'ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf', signature: 'deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead' }"
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", SUT::sample()),
            "LinkConnectionQRData with purpose: 'general'"
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "purpose": "general",
                "password": "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead",
                "publicKey": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
                "signature": "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"
            }
            "#,
        );
    }

    #[test]
    fn purpose() {
        assert_eq!(SUT::sample().purpose, RadixConnectPurpose::General);
    }

    #[test]
    fn password() {
        assert_eq!(
            SUT::sample().password.hash().to_string(),
            "9059d2ac799749e2f9f18541015197051ff9f803741d566744fe34ea004a5908"
        );
    }
}
