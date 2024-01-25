use crate::prelude::*;

use radix_engine_common::crypto::IsHash;
use transaction::signing::secp256k1::{
    Secp256k1PrivateKey as EngineSecp256k1PrivateKey, Secp256k1Signature,
};

/// A secp256k1 private key used to create cryptographic signatures, more specifically
/// ECDSA signatures, that offer recovery of the public key.
#[derive(derive_more::Debug)]
#[debug("{}", self.to_hex())]
pub struct Secp256k1PrivateKey(EngineSecp256k1PrivateKey);

impl Secp256k1PrivateKey {
    /// Generates a new `Secp256k1PrivateKey` from random bytes
    /// generated by a CSRNG, note that this is typically never
    /// used by wallets, which tend to rather use a Mnemonic and
    /// derive hierarchical deterministic keys.
    pub fn generate() -> Self {
        Self::from_hex32_bytes(Hex32Bytes::generate())
            .expect("Should be able to generate 32 bytes")
    }
}

impl PartialEq for Secp256k1PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl Eq for Secp256k1PrivateKey {}

impl Secp256k1PrivateKey {
    pub fn from_engine(engine: EngineSecp256k1PrivateKey) -> Self {
        Self(engine)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }

    pub fn from_bytes(slice: &[u8]) -> Result<Self> {
        EngineSecp256k1PrivateKey::from_bytes(slice)
            .map_err(|_| {
                CommonError::InvalidSecp256k1PrivateKeyFromBytes(
                    slice.to_owned(),
                )
            })
            .map(Self::from_engine)
    }

    pub fn from_vec(bytes: Vec<u8>) -> Result<Self> {
        Self::from_bytes(bytes.as_slice())
    }

    pub fn from_hex32_bytes(bytes: Hex32Bytes) -> Result<Self> {
        Self::from_vec(bytes.to_vec())
    }
}

impl FromStr for Secp256k1PrivateKey {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Hex32Bytes::from_hex(s)
            .map_err(|_| {
                CommonError::InvalidSecp256k1PrivateKeyFromString(s.to_owned())
            })
            .and_then(Self::from_hex32_bytes)
    }
}

impl TryFrom<&[u8]> for Secp256k1PrivateKey {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Secp256k1PrivateKey, Self::Error> {
        Secp256k1PrivateKey::from_bytes(slice)
    }
}

impl IsPrivateKey<Secp256k1PublicKey> for Secp256k1PrivateKey {
    fn curve() -> SLIP10Curve {
        SLIP10Curve::Secp256k1
    }

    type Signature = Secp256k1Signature;

    fn public_key(&self) -> Secp256k1PublicKey {
        Secp256k1PublicKey::try_from(self.0.public_key()).expect(
            "Public Key from EC scalar multiplication should always be valid.",
        )
    }

    fn sign(&self, msg_hash: &impl IsHash) -> Secp256k1Signature {
        self.0.sign(msg_hash)
    }
}

impl HasPlaceholder for Secp256k1PrivateKey {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_alice()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_bob()
    }
}

impl Secp256k1PrivateKey {
    /// `d78b6578b33f3446bdd9d09d057d6598bc915fec4008a54c509dc3b8cdc7dbe5`
    /// expected public key uncompressed:
    /// `04517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa71159e5614fb40739f4d22004380670cbc99ee4a2a73899d084098f3a139130c4`
    /// expected public key compressed:
    /// `02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7`
    ///
    /// https://github.com/Sajjon/K1/blob/main/Tests/K1Tests/TestVectors/cyon_ecdh_two_variants_with_kdf.json#L10
    pub fn placeholder_alice() -> Self {
        Self::from_str(
            "d78b6578b33f3446bdd9d09d057d6598bc915fec4008a54c509dc3b8cdc7dbe5",
        )
        .unwrap()
    }

    /// `871761c9921a467059e090a0422ae76af87fa8eb905da91c9b554bd6a028c760``
    /// expected public key uncompressed:
    /// `043083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8ab3efd3320b8f893cb421ed7ff0aa9ff43b43cad4e00e194f89845c6ac8233a7`
    /// expected public key compressed:
    /// `033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8`
    ///
    /// https://github.com/Sajjon/K1/blob/main/Tests/K1Tests/TestVectors/cyon_ecdh_two_variants_with_kdf.json#L12
    pub fn placeholder_bob() -> Self {
        Self::from_str(
            "871761c9921a467059e090a0422ae76af87fa8eb905da91c9b554bd6a028c760",
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;
    use transaction::signing::secp256k1::Secp256k1Signature;

    #[test]
    fn equality() {
        assert_eq!(
            Secp256k1PrivateKey::placeholder(),
            Secp256k1PrivateKey::placeholder()
        );
        assert_eq!(
            Secp256k1PrivateKey::placeholder_other(),
            Secp256k1PrivateKey::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            Secp256k1PrivateKey::placeholder(),
            Secp256k1PrivateKey::placeholder_other()
        );
    }

    #[test]
    fn curve() {
        assert_eq!(Secp256k1PrivateKey::curve(), SLIP10Curve::Secp256k1);
    }

    #[test]
    fn sign_and_verify() {
        let msg = hash("Test");
        let sk: Secp256k1PrivateKey =
            "0000000000000000000000000000000000000000000000000000000000000001"
                .parse()
                .unwrap();
        let pk = sk.public_key();
        assert_eq!(
            pk.to_hex(),
            "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
        );
        let sig = Secp256k1Signature::from_str("00eb8dcd5bb841430dd0a6f45565a1b8bdb4a204eb868832cd006f963a89a662813ab844a542fcdbfda4086a83fbbde516214113051b9c8e42a206c98d564d7122").unwrap();

        assert_eq!(sk.sign(&msg), sig);
        assert!(pk.is_valid(&sig, &msg))
    }

    #[test]
    fn bytes_roundtrip() {
        let bytes = hex_decode(
            "0000000000000000000000000000000000000000000000000000000000000001",
        )
        .unwrap();
        assert_eq!(
            Secp256k1PrivateKey::from_bytes(bytes.as_slice())
                .unwrap()
                .to_bytes(),
            bytes.as_slice()
        );
    }

    #[test]
    fn hex_roundtrip() {
        let hex =
            "0000000000000000000000000000000000000000000000000000000000000001";
        assert_eq!(Secp256k1PrivateKey::from_str(hex).unwrap().to_hex(), hex);
    }

    #[test]
    fn invalid_hex() {
        assert_eq!(
            Secp256k1PrivateKey::from_str("not hex"),
            Err(CommonError::InvalidSecp256k1PrivateKeyFromString(
                "not hex".to_owned()
            ))
        );
    }

    #[test]
    fn invalid_hex_too_short() {
        assert_eq!(
            Secp256k1PrivateKey::from_str("dead"),
            Err(CommonError::InvalidSecp256k1PrivateKeyFromString(
                "dead".to_owned()
            ))
        );
    }

    #[test]
    fn invalid_bytes() {
        assert_eq!(
            Secp256k1PrivateKey::from_bytes(&[0u8] as &[u8]),
            Err(CommonError::InvalidSecp256k1PrivateKeyFromBytes(vec![0]))
        );
    }

    #[test]
    fn invalid_too_large() {
        let bytes = [0xFFu8; 32];
        assert_eq!(
            Secp256k1PrivateKey::from_bytes(&bytes),
            Err(CommonError::InvalidSecp256k1PrivateKeyFromBytes(
                bytes.to_vec()
            ))
        );
    }

    #[test]
    fn invalid_zero() {
        let bytes = [0u8; 32];
        assert_eq!(
            Secp256k1PrivateKey::from_bytes(&bytes),
            Err(CommonError::InvalidSecp256k1PrivateKeyFromBytes(
                bytes.to_vec()
            ))
        );
    }

    #[test]
    fn debug() {
        let hex =
            "0000000000000000000000000000000000000000000000000000000000000001";
        assert_eq!(
            format!("{:?}", Secp256k1PrivateKey::from_str(hex).unwrap()),
            hex
        );
    }

    #[test]
    fn from_hex32_bytes() {
        let str =
            "0000000000000000000000000000000000000000000000000000000000000001";
        let hex32 = Hex32Bytes::from_hex(str).unwrap();
        let key = Secp256k1PrivateKey::from_hex32_bytes(hex32).unwrap();
        assert_eq!(key.to_hex(), str);
    }

    #[test]
    fn try_from_bytes() {
        let str =
            "0000000000000000000000000000000000000000000000000000000000000001";
        let vec = hex_decode(str).unwrap();
        let key = Secp256k1PrivateKey::try_from(vec.as_slice()).unwrap();
        assert_eq!(key.to_hex(), str);
    }

    #[test]
    fn generate_new() {
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        let n = 100;
        for _ in 0..n {
            let key = Secp256k1PrivateKey::generate();
            let bytes = key.to_bytes();
            assert_eq!(bytes.len(), 32);
            set.insert(bytes);
        }
        assert_eq!(set.len(), n);
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            Secp256k1PrivateKey::placeholder().public_key().to_hex(),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
    }
}
