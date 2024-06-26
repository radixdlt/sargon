use crate::prelude::*;
use crate::{prelude::*, UniffiCustomTypeConverter};
use crypto::keys::x25519::PublicKey as X25519PublicKey;

/// A public key for the X25519 key exchange algorithm.
/// An Ed25519 public key used to verify cryptographic signatures (EdDSA signatures).
#[serde_as]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct KeyAgreementPublicKey {
    pub secret_magic: X25519PublicKey,
}

impl From<KeyAgreementPrivateKey> for KeyAgreementPublicKey {
    fn from(value: KeyAgreementPrivateKey) -> Self {
        value.public_key()
    }
}

impl From<X25519PublicKey> for KeyAgreementPublicKey {
    fn from(value: X25519PublicKey) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl FromStr for KeyAgreementPublicKey {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s.to_owned())
    }
}

impl TryFrom<Vec<u8>> for KeyAgreementPublicKey {
    type Error = CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for KeyAgreementPublicKey {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Self> {
        X25519PublicKey::try_from_slice(slice)
            .map_err(|_| CommonError::InvalidKeyAgreementPublicKeyFromBytes {
                bad_value: slice.to_vec().into(),
            })
            .map(|k| k.into())
    }
}

impl KeyAgreementPublicKey {
    pub fn from_hex(hex: String) -> Result<Self> {
        Exactly32Bytes::from_str(hex.as_str())
            .map_err(|_| CommonError::InvalidKeyAgreementPublicKeyFromHex {
                bad_value: hex,
            })
            .and_then(|b| b.to_vec().try_into())
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.secret_magic.to_bytes().to_vec()
    }
}

impl HasSampleValues for KeyAgreementPublicKey {
    fn sample() -> Self {
        Self::sample_alice()
    }

    fn sample_other() -> Self {
        Self::sample_bob()
    }
}

impl KeyAgreementPublicKey {
    /// A sample used to facilitate unit tests.
    ///
    /// `8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10`
    pub fn sample_alice() -> Self {
        Self::from_hex(
            "8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10"
                .to_owned(),
        )
        .unwrap()
    }

    /// A sample used to facilitate unit tests.
    ///
    /// `c0f0d9d1b1f9c8c9f9b9d1f0c9f0c8c9f9b9d1f0c9f0c8c9f9b9d1f0c9f0c8c9`
    pub fn sample_bob() -> Self {
        Self::from_hex(
            "c0f0d9d1b1f9c8c9f9b9d1f0c9f0c8c9f9b9d1f0c9f0c8c9f9b9d1f0c9f0c8c9"
                .to_owned(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = KeyAgreementPublicKey;

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
    fn from_hex() {
        assert_eq!(
            SUT::from_hex("8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10".to_owned()),
            Ok(SUT::sample())
        );
    }

    #[test]
    fn to_hex() {
        assert_eq!(
            SUT::sample().to_hex(),
            "8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10"
        );
    }

    #[test]
    fn to_bytes() {
        assert_eq!(
            SUT::sample().to_bytes(),
            vec![
                134, 121, 188, 31, 227, 33, 11, 44, 232, 71, 147, 102, 139, 5,
                33, 143, 220, 76, 34, 11, 192, 83, 135, 183, 210, 172, 13, 76,
                123, 124, 93, 16
            ]
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            SUT::from_str("8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10"),
            Ok(SUT::sample())
        );
    }

    #[test]
    fn try_from_vec() {
        assert_eq!(
            SUT::try_from(vec![
                134, 121, 188, 31, 227, 33, 11, 44, 232, 71, 147, 102, 139, 5,
                33, 143, 220, 76, 34, 11, 192, 83, 135, 183, 210, 172, 13, 76,
                123, 124, 93, 16
            ]),
            Ok(SUT::sample())
        );
    }

    #[test]
    fn from_key_agreement_private_key() {
        assert_eq!(SUT::from(KeyAgreementPrivateKey::sample()), SUT::sample());
    }

    #[test]
    fn sample() {
        assert_eq!(
            SUT::sample().to_hex(),
            "8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10"
        );
    }

    #[test]
    fn sample_other() {
        assert_eq!(
            SUT::sample_other().to_hex(),
            "c0f0d9d1b1f9c8c9f9b9d1f0c9f0c8c9f9b9d1f0c9f0c8c9f9b9d1f0c9f0c8c9"
        );
    }

    #[test]
    fn from_invalid_hex() {
        assert_eq!(
            SUT::from_hex("bad".to_owned()),
            Err(CommonError::InvalidKeyAgreementPublicKeyFromHex {
                bad_value: "bad".to_owned()
            })
        );
    }

    #[test]
    fn from_invalid_bytes() {
        assert_eq!(
            SUT::try_from(vec![0]),
            Err(CommonError::InvalidKeyAgreementPublicKeyFromBytes {
                bad_value: vec![0].into()
            })
        );
    }

    #[test]
    fn from_invalid_str() {
        assert_eq!(
            "bad".parse::<SUT>(),
            Err(CommonError::InvalidKeyAgreementPublicKeyFromHex {
                bad_value: "bad".to_owned()
            })
        );
    }

    #[test]
    fn json_roundrip() {
        assert_json_value_eq_after_roundtrip(
            &SUT::sample(),
            json!("8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10")
        );
    }
}
