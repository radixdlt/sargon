use crate::{prelude::*, UniffiCustomTypeConverter};

use bip32::secp256k1::PublicKey as BIP32Secp256k1PublicKey; // the bip32 crate actually does validation of the PublicKey whereas `radix_engine_common` does not.
use radix_engine_common::crypto::{
    verify_secp256k1, Hash, IsHash,
    Secp256k1PublicKey as ScryptoSecp256k1PublicKey,
    Secp256k1Signature as ScryptoSecp256k1Signature,
};
// use transaction::{
//     signing::secp256k1::Secp256k1Signature, validation::verify_secp256k1,
// };

/// A `secp256k1` public key used to verify cryptographic signatures (ECDSA signatures).
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
pub struct Secp256k1PublicKey {
    inner: ScryptoSecp256k1PublicKey,
}

uniffi::custom_type!(ScryptoSecp256k1PublicKey, BagOfBytes);

impl UniffiCustomTypeConverter for ScryptoSecp256k1PublicKey {
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
pub fn new_secp256k1_public_key_from_hex(
    hex: String,
) -> Result<Secp256k1PublicKey> {
    hex.parse()
}

#[uniffi::export]
pub fn new_secp256k1_public_key_from_bytes(
    bytes: Vec<u8>,
) -> Result<Secp256k1PublicKey> {
    bytes.try_into()
}

/// Encodes the compressed form (33 bytes) of a `Secp256k1PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"`
#[uniffi::export]
pub fn secp256k1_public_key_to_hex(public_key: &Secp256k1PublicKey) -> String {
    public_key.to_hex()
}

#[uniffi::export]
pub fn secp256k1_public_key_to_bytes(
    public_key: &Secp256k1PublicKey,
) -> Vec<u8> {
    public_key.to_bytes()
}

#[uniffi::export]
pub fn new_secp256k1_public_key_placeholder() -> Secp256k1PublicKey {
    Secp256k1PublicKey::placeholder()
}

#[uniffi::export]
pub fn new_secp256k1_public_key_placeholder_other() -> Secp256k1PublicKey {
    Secp256k1PublicKey::placeholder_other()
}

impl IsPublicKey<ScryptoSecp256k1Signature> for Secp256k1PublicKey {
    /// Verifies an ECDSA signature over Secp256k1.
    fn is_valid(
        &self,
        signature: &ScryptoSecp256k1Signature,
        for_hash: &impl IsHash,
    ) -> bool {
        verify_secp256k1(for_hash.as_hash(), &self.to_engine(), signature)
    }
}

impl Secp256k1PublicKey {
    pub(crate) fn to_engine(&self) -> ScryptoSecp256k1PublicKey {
        self.inner
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_engine().to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }
}

impl TryFrom<ScryptoSecp256k1PublicKey> for Secp256k1PublicKey {
    type Error = CommonError;

    fn try_from(value: ScryptoSecp256k1PublicKey) -> Result<Self, Self::Error> {
        BIP32Secp256k1PublicKey::from_sec1_bytes(value.to_vec().as_slice())
            .map_err(|_| CommonError::InvalidSecp256k1PublicKeyPointNotOnCurve)
            .map(|_| Self { inner: value })
    }
}

impl TryFrom<Vec<u8>> for Secp256k1PublicKey {
    type Error = CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for Secp256k1PublicKey {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Self> {
        ScryptoSecp256k1PublicKey::try_from(slice)
            .map_err(|_| CommonError::InvalidSecp256k1PublicKeyFromBytes {
                bad_value: slice.to_vec().into(),
            })
            .and_then(|k| k.try_into())
    }
}

impl Secp256k1PublicKey {
    pub fn from_hex(hex: String) -> Result<Self> {
        hex_decode(hex.clone())
            .map_err(|_| CommonError::InvalidSecp256k1PublicKeyFromString {
                bad_value: hex,
            })
            .and_then(|b| Secp256k1PublicKey::try_from(b.as_slice()))
    }
}

impl HasPlaceholder for Secp256k1PublicKey {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_alice()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_bob()
    }
}

impl Secp256k1PublicKey {
    pub fn placeholder_alice() -> Self {
        Secp256k1PrivateKey::placeholder_alice().public_key()
    }

    pub fn placeholder_bob() -> Self {
        Secp256k1PrivateKey::placeholder_bob().public_key()
    }
}

impl FromStr for Secp256k1PublicKey {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use radix_engine_common::crypto::Secp256k1PublicKey as ScryptoSecp256k1PublicKey;

    #[test]
    fn equality() {
        assert_eq!(
            Secp256k1PublicKey::placeholder(),
            Secp256k1PublicKey::placeholder()
        );
        assert_eq!(
            Secp256k1PublicKey::placeholder_other(),
            Secp256k1PublicKey::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            Secp256k1PublicKey::placeholder(),
            Secp256k1PublicKey::placeholder_other()
        );
    }

    #[test]
    fn from_str() {
        assert!(Secp256k1PublicKey::from_str(
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        )
        .is_ok());
    }

    #[test]
    fn bytes_roundtrip() {
        let bytes: &[u8] = &[
            0x02, 0x51, 0x7b, 0x88, 0x91, 0x6e, 0x7f, 0x31, 0x5b, 0xb6, 0x82,
            0xf9, 0x92, 0x6b, 0x14, 0xbc, 0x67, 0xa0, 0xe4, 0x24, 0x6f, 0x8a,
            0x41, 0x9b, 0x98, 0x62, 0x69, 0xe1, 0xa7, 0xe6, 0x1f, 0xff, 0xa7,
        ];
        let key = Secp256k1PublicKey::try_from(bytes).unwrap();
        assert_eq!(
            key.to_hex(),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn placeholder_alice() {
        assert_eq!(
            Secp256k1PublicKey::placeholder_alice().to_hex(),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
    }

    #[test]
    fn from_engine() {
        let from_engine: Secp256k1PublicKey = ScryptoSecp256k1PublicKey::from_str(
            "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8",
        )
        .unwrap()
        .try_into()
        .unwrap();

        assert_eq!(
            from_engine,
            Secp256k1PublicKey::from_str(
                "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"
            )
            .unwrap()
        );
    }

    #[test]
    fn placeholder_bob() {
        assert_eq!(
            Secp256k1PublicKey::placeholder_bob().to_hex(),
            "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"
        );
    }

    #[test]
    fn invalid_hex_str() {
        assert_eq!(
            Secp256k1PublicKey::from_str("hi"),
            Err(CommonError::InvalidSecp256k1PublicKeyFromString {
                bad_value: "hi".to_owned()
            })
        );
    }

    #[test]
    fn invalid_str_too_short() {
        assert_eq!(
            Secp256k1PublicKey::from_str("dead"),
            Err(CommonError::InvalidSecp256k1PublicKeyFromBytes {
                bad_value: vec![0xde, 0xad].into()
            })
        );
    }

    #[test]
    fn invalid_bytes() {
        let bytes: &[u8] = &[0u8];
        assert_eq!(
            Secp256k1PublicKey::try_from(bytes),
            Err(CommonError::InvalidSecp256k1PublicKeyFromBytes {
                bad_value: bytes.to_vec().into()
            })
        );
    }

    #[test]
    fn invalid_key_not_on_curve() {
        assert_eq!(
            Secp256k1PublicKey::from_str(
                "99deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
            ),
            Err(CommonError::InvalidSecp256k1PublicKeyPointNotOnCurve)
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", Secp256k1PublicKey::placeholder_alice()),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
    }

    #[test]
    fn json() {
        let model = Secp256k1PublicKey::placeholder();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"),
        )
    }

    #[test]
    fn try_into_from_str() {
        let str = "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7";
        let key: Secp256k1PublicKey = str.parse().unwrap();
        assert_eq!(key.to_hex(), str);
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([
                Secp256k1PublicKey::placeholder_alice(),
                Secp256k1PublicKey::placeholder_alice()
            ])
            .len(),
            1
        );
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{
        new_secp256k1_public_key_from_bytes, new_secp256k1_public_key_from_hex,
        new_secp256k1_public_key_placeholder,
        new_secp256k1_public_key_placeholder_other,
        secp256k1_public_key_to_bytes, secp256k1_public_key_to_hex,
        HasPlaceholder,
    };

    use super::Secp256k1PublicKey;

    #[test]
    fn equality_placeholders() {
        assert_eq!(
            Secp256k1PublicKey::placeholder(),
            new_secp256k1_public_key_placeholder()
        );
        assert_eq!(
            Secp256k1PublicKey::placeholder_other(),
            new_secp256k1_public_key_placeholder_other()
        );
    }

    #[test]
    fn new_from_bytes() {
        let bytes =
            hex::decode("033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8")
                .unwrap();
        let from_bytes =
            new_secp256k1_public_key_from_bytes(bytes.clone()).unwrap();
        assert_eq!(
            from_bytes,
            Secp256k1PublicKey::try_from(bytes.clone()).unwrap()
        );
        assert_eq!(secp256k1_public_key_to_bytes(&from_bytes), bytes);
    }

    #[test]
    fn new_from_hex() {
        let hex = "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8";
        let from_hex =
            new_secp256k1_public_key_from_hex(hex.to_string()).unwrap();
        assert_eq!(
            from_hex,
            Secp256k1PublicKey::from_hex(hex.to_string()).unwrap()
        );
        assert_eq!(secp256k1_public_key_to_hex(&from_hex), hex)
    }
}
