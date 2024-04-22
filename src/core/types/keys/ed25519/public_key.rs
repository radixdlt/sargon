use crate::{prelude::*, UniffiCustomTypeConverter};

use crypto::signatures::ed25519 as IotaSlip10Ed25519;

/// An Ed25519 public key used to verify cryptographic signatures (EdDSA signatures).
#[serde_as]
#[derive(
    Clone,
    Copy,
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
    secret_magic: ScryptoEd25519PublicKey,
}

impl From<Ed25519PublicKey> for ScryptoEd25519PublicKey {
    fn from(value: Ed25519PublicKey) -> Self {
        value.secret_magic
    }
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
    bytes: BagOfBytes,
) -> Result<Ed25519PublicKey> {
    bytes.to_vec().try_into()
}

#[uniffi::export]
pub fn new_ed25519_public_key_sample() -> Ed25519PublicKey {
    Ed25519PublicKey::sample()
}

#[uniffi::export]
pub fn new_ed25519_public_key_sample_other() -> Ed25519PublicKey {
    Ed25519PublicKey::sample_other()
}

/// Encodes the `Ed25519PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde"`
#[uniffi::export]
pub fn ed25519_public_key_to_hex(public_key: &Ed25519PublicKey) -> String {
    public_key.to_hex()
}

#[uniffi::export]
pub fn ed25519_public_key_to_bytes(
    public_key: &Ed25519PublicKey,
) -> BagOfBytes {
    public_key.to_bytes().into()
}

impl IsPublicKey<Ed25519Signature> for Ed25519PublicKey {
    /// Verifies an EdDSA signature over Curve25519.
    fn is_valid_signature_for_hash(
        &self,
        signature: &Ed25519Signature,
        hash: &impl ScryptoIsHash,
    ) -> bool {
        scrypto_verify_ed25519(
            hash.as_hash(),
            &self.scrypto(),
            &(*signature).into(),
        )
    }
}

impl Ed25519PublicKey {
    pub(crate) fn scrypto(&self) -> ScryptoEd25519PublicKey {
        self.secret_magic
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.scrypto().to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }
}

impl TryFrom<ScryptoEd25519PublicKey> for Ed25519PublicKey {
    type Error = CommonError;

    fn try_from(value: ScryptoEd25519PublicKey) -> Result<Self, Self::Error> {
        IotaSlip10Ed25519::PublicKey::try_from_bytes(value.0)
            .map_err(|_| CommonError::InvalidEd25519PublicKeyPointNotOnCurve)
            .map(|_| Self {
                secret_magic: value,
            })
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
        Exactly32Bytes::from_str(hex.as_str())
            .map_err(|_| CommonError::InvalidEd25519PublicKeyFromString {
                bad_value: hex,
            })
            .and_then(|b| b.to_vec().try_into())
    }
}

impl HasSampleValues for Ed25519PublicKey {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_alice()
    }

    fn sample_other() -> Self {
        Self::sample_bob()
    }
}

impl FromStr for Ed25519PublicKey {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s.to_string())
    }
}

impl Ed25519PublicKey {
    pub fn sample_alice() -> Self {
        Ed25519PrivateKey::sample_alice().public_key()
    }

    pub fn sample_bob() -> Self {
        Ed25519PrivateKey::sample_bob().public_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(Ed25519PublicKey::sample(), Ed25519PublicKey::sample());
        assert_eq!(
            Ed25519PublicKey::sample_other(),
            Ed25519PublicKey::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            Ed25519PublicKey::sample(),
            Ed25519PublicKey::sample_other()
        );
    }

    #[test]
    fn json() {
        let model = Ed25519PublicKey::sample_alice();
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
    fn from_scrypto() {
        let from_scrypto: Ed25519PublicKey = ScryptoEd25519PublicKey::from_str(
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
        )
        .unwrap()
        .try_into()
        .unwrap();
        assert_eq!(
            from_scrypto,
            Ed25519PublicKey::from_str(
                "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
            )
            .unwrap()
        );

        // and back
        assert_eq!(
            TryInto::<Ed25519PublicKey>::try_into(Into::<
                ScryptoEd25519PublicKey,
            >::into(
                from_scrypto
            ))
            .unwrap(),
            from_scrypto
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
    fn sample_alice() {
        assert_eq!(
            Ed25519PublicKey::sample_alice().to_hex(),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
    }

    #[test]
    fn sample_bob() {
        assert_eq!(
            Ed25519PublicKey::sample_bob().to_hex(),
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
            format!("{:?}", Ed25519PublicKey::sample_alice()),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([
                Ed25519PublicKey::sample_alice(),
                Ed25519PublicKey::sample_alice()
            ])
            .len(),
            1
        );
    }
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[test]
    fn equality_samples() {
        assert_eq!(Ed25519PublicKey::sample(), new_ed25519_public_key_sample());
        assert_eq!(
            Ed25519PublicKey::sample_other(),
            new_ed25519_public_key_sample_other()
        );
    }

    #[test]
    fn new_from_bytes() {
        let bag_of_bytes = BagOfBytes::from_str(
            "b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde",
        )
        .unwrap();

        let from_bytes =
            new_ed25519_public_key_from_bytes(bag_of_bytes.clone()).unwrap();
        assert_eq!(
            from_bytes,
            Ed25519PublicKey::try_from(bag_of_bytes.as_ref()).unwrap()
        );
        assert_eq!(ed25519_public_key_to_bytes(&from_bytes), bag_of_bytes);
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
