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
    uniffi::Record,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct KeyAgreementPublicKey {
    pub secret_magic: X25519PublicKey,
}

uniffi::custom_type!(X25519PublicKey, BagOfBytes);

impl UniffiCustomTypeConverter for X25519PublicKey {
    type Builtin = BagOfBytes;

    #[cfg(not(tarpaulin_include))] // false negative | tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Self::try_from_slice(val.as_slice())
            .map_err(|e| uniffi::deps::anyhow::anyhow!(e.to_string()))
    }

    #[cfg(not(tarpaulin_include))] // false negative | tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_bytes().to_vec().into()
    }
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
    /// `833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42`
    pub fn sample_alice() -> Self {
        Self::from_hex(
            "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42"
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
            SUT::from_hex("833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42".to_owned()),
            Ok(SUT::sample())
        );
    }

    #[test]
    fn to_hex() {
        assert_eq!(
            SUT::sample().to_hex(),
            "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42"
        );
    }

    #[test]
    fn to_bytes() {
        assert_eq!(
            SUT::sample().to_bytes(),
            vec![
                131, 63, 230, 36, 9, 35, 123, 157, 98, 236, 119, 88, 117, 32,
                145, 30, 154, 117, 156, 236, 29, 25, 117, 91, 125, 169, 1, 185,
                109, 202, 61, 66
            ]
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            SUT::from_str("833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42"),
            Ok(SUT::sample())
        );
    }

    #[test]
    fn try_from_vec() {
        assert_eq!(
            SUT::try_from(vec![
                131, 63, 230, 36, 9, 35, 123, 157, 98, 236, 119, 88, 117, 32,
                145, 30, 154, 117, 156, 236, 29, 25, 117, 91, 125, 169, 1, 185,
                109, 202, 61, 66
            ]),
            Ok(SUT::sample())
        );
    }

    #[test]
    fn sample() {
        assert_eq!(
            SUT::sample().to_hex(),
            "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42"
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
            json!("833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42")
        );
    }
}
