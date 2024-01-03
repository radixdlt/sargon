use crate::KeyError as Error;
use bip32::secp256k1::PublicKey as BIP32Secp256k1PublicKey;
use radix_engine_common::crypto::{Hash, Secp256k1PublicKey as EngineSecp256k1PublicKey};
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use transaction::{signing::secp256k1::Secp256k1Signature, validation::verify_secp256k1};

use crate::HasPlaceholder;

use crate::Secp256k1PrivateKey;

/// A `secp256k1` public key used to verify cryptographic signatures (ECDSA signatures).
#[serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, uniffi::Record)]
#[serde(transparent)]
pub struct Secp256k1PublicKey {
    #[serde_as(as = "Hex")]
    bytes: Vec<u8>, // FIXME: change to either EngineSecp256k1PublicKey or bip32::secp256k1::PublicKey once we have proper UniFFI lift/lower/UniffiCustomTypeConverter
}

#[uniffi::export]
pub fn new_secp256k1_public_key_from_hex(
    hex: String,
) -> Result<Secp256k1PublicKey, crate::KeyError> {
    Secp256k1PublicKey::from_hex(hex)
}

#[uniffi::export]
pub fn new_secp256k1_public_key_from_bytes(
    bytes: Vec<u8>,
) -> Result<Secp256k1PublicKey, crate::KeyError> {
    Secp256k1PublicKey::from_bytes(bytes)
}

/// Encodes the compressed form (33 bytes) of a `Secp256k1PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"`
#[uniffi::export]
pub fn secp256k1_public_key_to_hex(public_key: &Secp256k1PublicKey) -> String {
    public_key.to_hex()
}

#[uniffi::export]
pub fn secp256k1_public_key_to_bytes(public_key: &Secp256k1PublicKey) -> Vec<u8> {
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

impl From<EngineSecp256k1PublicKey> for Secp256k1PublicKey {
    fn from(value: EngineSecp256k1PublicKey) -> Self {
        Self::from_engine(value).expect("EngineEd25519PublicKey should have been valid.")
    }
}

impl Secp256k1PublicKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

impl Secp256k1PublicKey {
    pub(crate) fn to_engine(&self) -> EngineSecp256k1PublicKey {
        EngineSecp256k1PublicKey::try_from(self.to_bytes().as_slice()).unwrap()
    }

    pub(crate) fn from_engine(engine: EngineSecp256k1PublicKey) -> Result<Self, Error> {
        BIP32Secp256k1PublicKey::from_sec1_bytes(engine.to_vec().as_slice())
            .map(|_| Self {
                bytes: engine.to_vec(),
            })
            .map_err(|_| Error::InvalidSecp256k1PublicKeyPointNotOnCurve)
    }

    /// Verifies an ECDSA signature over Secp256k1.
    pub fn is_valid(&self, signature: &Secp256k1Signature, for_hash: &Hash) -> bool {
        verify_secp256k1(for_hash, &self.to_engine(), signature)
    }
}

impl Secp256k1PublicKey {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, crate::KeyError> {
        EngineSecp256k1PublicKey::try_from(bytes.as_slice())
            .map_err(|_| Error::InvalidSecp256k1PublicKeyFromBytes)
            .and_then(|pk| Self::from_engine(pk))
    }
}

impl TryFrom<&[u8]> for Secp256k1PublicKey {
    type Error = crate::KeyError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(slice.to_vec())
    }
}

impl TryInto<Secp256k1PublicKey> for &str {
    type Error = crate::KeyError;

    fn try_into(self) -> Result<Secp256k1PublicKey, Self::Error> {
        Secp256k1PublicKey::from_str(self)
    }
}

impl FromStr for Secp256k1PublicKey {
    type Err = crate::KeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s.to_string())
    }
}

impl Secp256k1PublicKey {
    pub fn from_hex(hex: String) -> Result<Self, Error> {
        hex::decode(hex)
            .map_err(|_| Error::InvalidSecp256k1PublicKeyFromString)
            .and_then(|b| Secp256k1PublicKey::try_from(b.as_slice()))
    }
}

impl Debug for Secp256k1PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
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

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, str::FromStr};

    use super::Secp256k1PublicKey;
    use crate::{assert_json_value_eq_after_roundtrip, HasPlaceholder, KeyError as Error};
    use serde_json::json;

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
        let bytes: &[u8] =
            &[
                0x02, 0x51, 0x7b, 0x88, 0x91, 0x6e, 0x7f, 0x31, 0x5b, 0xb6, 0x82, 0xf9, 0x92, 0x6b,
                0x14, 0xbc, 0x67, 0xa0, 0xe4, 0x24, 0x6f, 0x8a, 0x41, 0x9b, 0x98, 0x62, 0x69, 0xe1,
                0xa7, 0xe6, 0x1f, 0xff, 0xa7,
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
    fn placeholder_bob() {
        assert_eq!(
            Secp256k1PublicKey::placeholder_bob().to_hex(),
            "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"
        );
    }

    #[test]
    fn invalid_hex_str() {
        assert_eq!(
            Secp256k1PublicKey::from_str("not a valid hex string"),
            Err(Error::InvalidSecp256k1PublicKeyFromString)
        );
    }

    #[test]
    fn invalid_str_too_short() {
        assert_eq!(
            Secp256k1PublicKey::from_str("dead"),
            Err(Error::InvalidSecp256k1PublicKeyFromBytes)
        );
    }

    #[test]
    fn invalid_bytes() {
        assert_eq!(
            Secp256k1PublicKey::try_from(&[0u8] as &[u8]),
            Err(Error::InvalidSecp256k1PublicKeyFromBytes)
        );
    }

    #[test]
    fn invalid_key_not_on_curve() {
        assert_eq!(
            Secp256k1PublicKey::from_str(
                "99deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
            ),
            Err(Error::InvalidSecp256k1PublicKeyPointNotOnCurve)
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
        let key: Secp256k1PublicKey = str.try_into().unwrap();
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
        new_secp256k1_public_key_placeholder, new_secp256k1_public_key_placeholder_other,
        secp256k1_public_key_to_bytes, secp256k1_public_key_to_hex, HasPlaceholder,
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
        let from_bytes = new_secp256k1_public_key_from_bytes(bytes.clone()).unwrap();
        assert_eq!(
            from_bytes,
            Secp256k1PublicKey::from_bytes(bytes.clone()).unwrap()
        );
        assert_eq!(secp256k1_public_key_to_bytes(&from_bytes), bytes);
    }

    #[test]
    fn new_from_hex() {
        let hex = "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8";
        let from_hex = new_secp256k1_public_key_from_hex(hex.to_string()).unwrap();
        assert_eq!(
            from_hex,
            Secp256k1PublicKey::from_hex(hex.to_string()).unwrap()
        );
        assert_eq!(secp256k1_public_key_to_hex(&from_hex), hex)
    }
}
