use crate::{prelude::*, UniffiCustomTypeConverter};
use sargon::Secp256k1PublicKey as InternalSecp256k1PublicKey;
use sargon::BagOfBytes as InternalBagOfBytes;

/// A `secp256k1` public key used to verify cryptographic signatures (ECDSA signatures).
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    
    
    uniffi::Record,
)]
pub struct Secp256k1PublicKey {
    secret_magic: BagOfBytes,
}

impl From<InternalSecp256k1PublicKey> for Secp256k1PublicKey {
    fn from(value: InternalSecp256k1PublicKey) -> Self {
        Self {
            secret_magic: value.to_bytes().into(),
        }
    }
}

impl Into<InternalSecp256k1PublicKey> for Secp256k1PublicKey {
    fn into(self) -> InternalSecp256k1PublicKey {
        InternalSecp256k1PublicKey::try_from(self.secret_magic.to_vec()).unwrap()
    }
}

#[uniffi::export]
pub fn new_secp256k1_public_key_from_hex(
    hex: String,
) -> Result<Secp256k1PublicKey> {
    map_result_from_internal(hex.parse::<InternalSecp256k1PublicKey>())
}

/// Creates a Secp256k1PublicKey from either compressed form (33 bytes) or
/// from uncompressed form (65 bytes).
#[uniffi::export]
pub fn new_secp256k1_public_key_from_bytes(
    bytes: BagOfBytes,
) -> Result<Secp256k1PublicKey> {
    map_result_from_internal(InternalSecp256k1PublicKey::try_from(bytes.into::<InternalBagOfBytes>().to_vec()))
}

/// Encodes the compressed form (33 bytes) of a `Secp256k1PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"`
#[uniffi::export]
pub fn secp256k1_public_key_to_hex(public_key: &Secp256k1PublicKey) -> String {
    public_key.into::<InternalSecp256k1PublicKey>().to_hex()
}

/// Returns the public key on **compressed** form (33 bytes)
#[uniffi::export]
pub fn secp256k1_public_key_to_bytes(
    public_key: &Secp256k1PublicKey,
) -> BagOfBytes {
    public_key.into::<InternalSecp256k1PublicKey>().to_bag_of_bytes().into()
}

/// Returns the public key on **uncompressed** form (65 bytes)
#[uniffi::export]
pub fn secp256k1_public_key_to_bytes_uncompressed(
    public_key: &Secp256k1PublicKey,
) -> BagOfBytes {
    InternalBagOfBytes::from(public_key.into::<InternalSecp256k1PublicKey>().uncompressed()).into()
}

#[uniffi::export]
pub fn new_secp256k1_public_key_sample() -> Secp256k1PublicKey {
    InternalSecp256k1PublicKey::sample().into()
}

#[uniffi::export]
pub fn new_secp256k1_public_key_sample_other() -> Secp256k1PublicKey {
    InternalSecp256k1PublicKey::sample_other().into()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Secp256k1PublicKey;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_secp256k1_public_key_sample());
        assert_eq!(
            SUT::sample_other(),
            new_secp256k1_public_key_sample_other()
        );
    }

    #[test]
    fn new_from_bytes() {
        // https://github.com/Sajjon/K1/blob/main/Tests/K1Tests/TestCases/Keys/PublicKey/PublicKeyImportTests.swift#L48
        let bag_of_bytes: BagOfBytes = "040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2".parse().unwrap();
        let from_bytes =
            new_secp256k1_public_key_from_bytes(bag_of_bytes.clone()).unwrap();

        assert_eq!(from_bytes, SUT::try_from(bag_of_bytes.as_ref()).unwrap());
        // compressed is this...
        assert_eq!(secp256k1_public_key_to_bytes(&from_bytes).to_hex(), "020202020202020202020202020202020202020202020202020202020202020202");
        assert_eq!(secp256k1_public_key_to_bytes_uncompressed(&from_bytes).to_hex(), "040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2");
    }

    #[test]
    fn new_from_hex() {
        let hex = "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8";
        let from_hex =
            new_secp256k1_public_key_from_hex(hex.to_string()).unwrap();
        assert_eq!(from_hex, SUT::from_hex(hex.to_string()).unwrap());
        assert_eq!(secp256k1_public_key_to_hex(&from_hex), hex)
    }
}
