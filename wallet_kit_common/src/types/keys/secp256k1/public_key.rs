use crate::{error::Error, types::keys::secp256k1::private_key::Secp256k1PrivateKey};
use bip32::secp256k1::PublicKey as BIP32Secp256k1PublicKey;
use radix_engine_common::crypto::{Hash, Secp256k1PublicKey as EngineSecp256k1PublicKey};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use transaction::{signing::secp256k1::Secp256k1Signature, validation::verify_secp256k1};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Secp256k1PublicKey(EngineSecp256k1PublicKey);

impl Secp256k1PublicKey {
    pub(crate) fn from_engine(engine: EngineSecp256k1PublicKey) -> Result<Self, Error> {
        BIP32Secp256k1PublicKey::from_sec1_bytes(engine.to_vec().as_slice())
            .map(|_| Self(engine))
            .map_err(|_| Error::InvalidSecp256k1PublicKeyPointNotOnCurve)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    /// Verifies an ECDSA signature over Secp256k1.
    pub fn is_valid(&self, signature: &Secp256k1Signature, for_hash: &Hash) -> bool {
        verify_secp256k1(for_hash, &self.0, signature)
    }
}

impl TryFrom<&[u8]> for Secp256k1PublicKey {
    type Error = crate::error::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        EngineSecp256k1PublicKey::try_from(slice)
            .map_err(|_| Error::InvalidSecp256k1PublicKeyFromBytes)
            .and_then(|pk| Self::from_engine(pk))
    }
}

impl TryInto<Secp256k1PublicKey> for &str {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<Secp256k1PublicKey, Self::Error> {
        Secp256k1PublicKey::from_str(self)
    }
}

impl Secp256k1PublicKey {
    pub fn from_str(hex: &str) -> Result<Self, Error> {
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

impl Secp256k1PublicKey {
    pub fn placeholder() -> Self {
        Self::placeholder_alice()
    }

    pub fn placeholder_alice() -> Self {
        Secp256k1PrivateKey::placeholder_alice().public_key()
    }

    pub fn placeholder_bob() -> Self {
        Secp256k1PrivateKey::placeholder_bob().public_key()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::Secp256k1PublicKey;
    use crate::{error::Error, json::assert_json_value_eq_after_roundtrip};
    use serde_json::json;

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
    fn inequality() {
        assert_ne!(
            Secp256k1PublicKey::placeholder_alice(),
            Secp256k1PublicKey::placeholder_bob()
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            Secp256k1PublicKey::placeholder_alice(),
            Secp256k1PublicKey::placeholder_alice()
        );
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
