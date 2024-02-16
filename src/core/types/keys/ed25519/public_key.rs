use crate::{prelude::*, UniffiCustomTypeConverter};

use radix_engine_common::crypto::{
    verify_ed25519 as scrypto_verify_ed25519,
    Ed25519PublicKey as ScryptoEd25519PublicKey,
    Ed25519Signature as ScryptoEd25519Signature, IsHash,
};

/// An Ed25519 public key used to verify cryptographic signatures (EdDSA signatures).
#[serde_as]
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay, // yes we could have #[serde(transparent)] since `ScryptoEd25519PublicKey` is Serialize, but we wanna be in control.
    DeserializeFromStr, // yes we could have #[serde(transparent)] since `ScryptoEd25519PublicKey` is Deserialize, but we wanna be in control.
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Ed25519PublicKey {
    inner: ScryptoEd25519PublicKey,
}

uniffi::custom_type!(ScryptoEd25519PublicKey, BagOfBytes);
impl UniffiCustomTypeConverter for ScryptoEd25519PublicKey {
    type Builtin = BagOfBytes;

    #[cfg(not(tarpaulin_include))] // false negative | tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Self::try_from(val.as_slice()).map_err(|e| e.into())
    }

    #[cfg(not(tarpaulin_include))] // false negative | tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_vec().into()
    }
}

#[uniffi::export]
pub fn new_ed25519_public_key_from_hex(
    hex: String,
) -> Result<Ed25519PublicKey> {
    hex.parse()
}

#[uniffi::export]
pub fn new_ed25519_public_key_from_bytes(
    bytes: Vec<u8>,
) -> Result<Ed25519PublicKey> {
    bytes.try_into()
}

#[uniffi::export]
pub fn new_ed25519_public_key_placeholder() -> Ed25519PublicKey {
    Ed25519PublicKey::placeholder()
}

#[uniffi::export]
pub fn new_ed25519_public_key_placeholder_other() -> Ed25519PublicKey {
    Ed25519PublicKey::placeholder_other()
}

/// Encodes the `Ed25519PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde"`
#[uniffi::export]
pub fn ed25519_public_key_to_hex(public_key: &Ed25519PublicKey) -> String {
    public_key.to_hex()
}

#[uniffi::export]
pub fn ed25519_public_key_to_bytes(public_key: &Ed25519PublicKey) -> Vec<u8> {
    public_key.to_bytes()
}

impl IsPublicKey<ScryptoEd25519Signature> for Ed25519PublicKey {
    /// Verifies an EdDSA signature over Curve25519.
    fn is_valid(
        &self,
        signature: &ScryptoEd25519Signature,
        for_hash: &impl IsHash,
    ) -> bool {
        scrypto_verify_ed25519(for_hash.as_hash(), &self.to_engine(), signature)
    }
}

impl Ed25519PublicKey {
    pub(crate) fn to_engine(&self) -> ScryptoEd25519PublicKey {
        self.inner
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_engine().to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }
}

impl TryFrom<ScryptoEd25519PublicKey> for Ed25519PublicKey {
    type Error = CommonError;

    fn try_from(value: ScryptoEd25519PublicKey) -> Result<Self, Self::Error> {
        ed25519_dalek::PublicKey::from_bytes(value.to_vec().as_slice())
            .map_err(|_| CommonError::InvalidEd25519PublicKeyPointNotOnCurve)
            .map(|_| Self { inner: value })
    }
}

impl TryFrom<Vec<u8>> for Ed25519PublicKey {
    type Error = CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for Ed25519PublicKey {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Self> {
        ScryptoEd25519PublicKey::try_from(slice)
            .map_err(|_| CommonError::InvalidEd25519PublicKeyFromBytes {
                bad_value: slice.to_vec().into(),
            })
            .and_then(|k| k.try_into())
    }
}

impl Ed25519PublicKey {
    pub fn from_hex(hex: String) -> Result<Self> {
        // We want to ALWAYS go via `try_from(slice: &[u8])` since validates the EC point (`ScryptoEd25519PublicKey` doesn't!)
        Hex32Bytes::from_str(hex.as_str())
            .map_err(|_| CommonError::InvalidEd25519PublicKeyFromString {
                bad_value: hex,
            })
            .and_then(|b| b.to_vec().try_into())
    }
}

impl HasPlaceholder for Ed25519PublicKey {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_alice()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_bob()
    }
}

impl FromStr for Ed25519PublicKey {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s.to_string())
    }
}

impl Ed25519PublicKey {
    pub fn placeholder_alice() -> Self {
        Ed25519PrivateKey::placeholder_alice().public_key()
    }

    pub fn placeholder_bob() -> Self {
        Ed25519PrivateKey::placeholder_bob().public_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            Ed25519PublicKey::placeholder(),
            Ed25519PublicKey::placeholder()
        );
        assert_eq!(
            Ed25519PublicKey::placeholder_other(),
            Ed25519PublicKey::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            Ed25519PublicKey::placeholder(),
            Ed25519PublicKey::placeholder_other()
        );
    }

    #[test]
    fn json() {
        let model = Ed25519PublicKey::placeholder_alice();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"),
        )
    }

    #[test]
    fn from_str() {
        assert!(Ed25519PublicKey::from_str(
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        )
        .is_ok());
    }

    #[test]
    fn from_engine() {
        let from_engine: Ed25519PublicKey = ScryptoEd25519PublicKey::from_str(
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
        )
        .unwrap()
        .try_into()
        .unwrap();
        assert_eq!(
            from_engine,
            Ed25519PublicKey::from_str(
                "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
            )
            .unwrap()
        );
    }

    #[test]
    fn bytes_roundtrip() {
        let bytes: &[u8] = &[
            0xec, 0x17, 0x2b, 0x93, 0xad, 0x5e, 0x56, 0x3b, 0xf4, 0x93, 0x2c,
            0x70, 0xe1, 0x24, 0x50, 0x34, 0xc3, 0x54, 0x67, 0xef, 0x2e, 0xfd,
            0x4d, 0x64, 0xeb, 0xf8, 0x19, 0x68, 0x34, 0x67, 0xe2, 0xbf,
        ];
        let key = Ed25519PublicKey::try_from(bytes).unwrap();
        assert_eq!(
            key.to_hex(),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn placeholder_alice() {
        assert_eq!(
            Ed25519PublicKey::placeholder_alice().to_hex(),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
    }

    #[test]
    fn placeholder_bob() {
        assert_eq!(
            Ed25519PublicKey::placeholder_bob().to_hex(),
            "b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde"
        );
    }

    #[test]
    fn invalid_bytes() {
        assert_eq!(
            Ed25519PublicKey::try_from(&[0u8] as &[u8]),
            Err(CommonError::InvalidEd25519PublicKeyFromBytes {
                bad_value: vec![0].into()
            })
        );
    }

    #[test]
    fn invalid_hex_str() {
        assert_eq!(
            Ed25519PublicKey::from_str("not a valid hex string"),
            Err(CommonError::InvalidEd25519PublicKeyFromString {
                bad_value: "not a valid hex string".to_owned()
            })
        );
    }

    #[test]
    fn invalid_str_too_short() {
        assert_eq!(
            Ed25519PublicKey::from_str("dead"),
            Err(CommonError::InvalidEd25519PublicKeyFromString {
                bad_value: "dead".to_owned()
            })
        );
    }

    #[test]
    fn try_into_from_str() {
        let str =
            "b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde";
        let key: Ed25519PublicKey = str.parse().unwrap();
        assert_eq!(key.to_hex(), str);
    }

    #[test]
    fn invalid_key_not_on_curve() {
        assert_eq!(
            Ed25519PublicKey::from_str(
                "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
            ),
            Err(CommonError::InvalidEd25519PublicKeyPointNotOnCurve)
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", Ed25519PublicKey::placeholder_alice()),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([
                Ed25519PublicKey::placeholder_alice(),
                Ed25519PublicKey::placeholder_alice()
            ])
            .len(),
            1
        );
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{
        ed25519_public_key_to_bytes, ed25519_public_key_to_hex,
        new_ed25519_public_key_from_bytes, new_ed25519_public_key_from_hex,
        new_ed25519_public_key_placeholder,
        new_ed25519_public_key_placeholder_other, HasPlaceholder,
    };

    use super::Ed25519PublicKey;

    #[test]
    fn equality_placeholders() {
        assert_eq!(
            Ed25519PublicKey::placeholder(),
            new_ed25519_public_key_placeholder()
        );
        assert_eq!(
            Ed25519PublicKey::placeholder_other(),
            new_ed25519_public_key_placeholder_other()
        );
    }

    #[test]
    fn new_from_bytes() {
        let bytes = hex::decode(
            "b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde",
        )
        .unwrap();
        let from_bytes =
            new_ed25519_public_key_from_bytes(bytes.clone()).unwrap();
        assert_eq!(
            from_bytes,
            Ed25519PublicKey::try_from(bytes.clone()).unwrap()
        );
        assert_eq!(ed25519_public_key_to_bytes(&from_bytes), bytes);
    }

    #[test]
    fn new_from_hex() {
        let hex =
            "b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde";
        let from_hex =
            new_ed25519_public_key_from_hex(hex.to_string()).unwrap();
        assert_eq!(
            from_hex,
            Ed25519PublicKey::from_hex(hex.to_string()).unwrap()
        );
        assert_eq!(ed25519_public_key_to_hex(&from_hex), hex)
    }
}
