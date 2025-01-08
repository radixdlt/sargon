use crate::prelude::*;

/// The QR code data scanned from the Connector Extension
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
#[serde_as]
#[debug(
    "LinkConnectionQRData {{ purpose: '{purpose}', password: '{password}', public_key_of_other_party: '{public_key_of_other_party}', signature: '{signature}' }}"
)]
#[display("{}", self.to_obfuscated_string())]
pub struct LinkConnectionQRData {
    /// The purpose of the connection, set by the other client, typically Connector Extension or dApp.
    /// As part of the initial linking flow, user will be prompted about kind of link they're trying to make.
    /// The user needs to make a conscious decision about general purpose links (because it comes with security risk).
    pub purpose: RadixConnectPurpose,

    /// Used to be able to re-establish the P2P connection
    pub password: RadixConnectPassword,

    /// Each client generates a curve25119 keypair. The public key will be used as an identifier for the client.
    /// Each client keeps a record of linked clients' public keys to prevent duplicate links.
    /// This is the public key of the other client and it also serves as the seed for the link `ID`.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "publicKey")]
    pub public_key_of_other_party: Ed25519PublicKey,

    /// Represents a signature produced by Connector Extension by signing the hash of the `password`
    /// with the private key of the `public_key_of_other_party`.
    #[serde_as(as = "DisplayFromStr")]
    pub signature: Ed25519Signature,
}

impl LinkConnectionQRData {
    pub fn new(
        purpose: RadixConnectPurpose,
        password: RadixConnectPassword,
        public_key_of_other_party: Ed25519PublicKey,
        signature: Ed25519Signature,
    ) -> Self {
        Self {
            purpose,
            password,
            public_key_of_other_party,
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
            Ed25519Signature::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            RadixConnectPurpose::sample_other(),
            RadixConnectPassword::sample_other(),
            Ed25519PublicKey::sample_other(),
            Ed25519Signature::sample_other(),
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
            "LinkConnectionQRData { purpose: 'general', password: 'deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead', public_key_of_other_party: 'ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf', signature: 'fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103' }"
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
                "signature": "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
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
