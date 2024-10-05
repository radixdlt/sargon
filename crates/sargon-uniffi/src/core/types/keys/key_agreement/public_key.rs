use crate::prelude::*;
use crate::{prelude::*, UniffiCustomTypeConverter};
use crypto::keys::x25519::PublicKey as X25519PublicKey;

/// PublicKey on Curve25519 used for key agreement (ECDH) with some `KeyAgreementPrivateKey`.
#[serde_as]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
pub struct KeyAgreementPublicKey {
    pub secret_magic: X25519PublicKey,
}

#[uniffi::export]
pub fn new_key_agreement_public_key_from_hex(
    hex: String,
) -> Result<KeyAgreementPublicKey> {
    hex.parse()
}

/// Creates a Secp256k1PublicKey from either compressed form (33 bytes) or
/// from uncompressed form (65 bytes).
#[uniffi::export]
pub fn new_key_agreement_public_key_from_bytes(
    bytes: BagOfBytes,
) -> Result<KeyAgreementPublicKey> {
    KeyAgreementPublicKey::try_from(bytes.to_vec())
}

/// Encodes the compressed form (33 bytes) of a `Secp256k1PublicKey` to a hexadecimal string, lowercased, without any `0x` prefix, e.g.
/// `"033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"`
#[uniffi::export]
pub fn key_agreement_public_key_to_hex(
    public_key: &KeyAgreementPublicKey,
) -> String {
    public_key.to_hex()
}

/// Returns the public key on **compressed** form (33 bytes)
#[uniffi::export]
pub fn key_agreement_public_key_to_bytes(
    public_key: &KeyAgreementPublicKey,
) -> BagOfBytes {
    BagOfBytes::from(public_key.to_bytes())
}

#[uniffi::export]
pub fn new_key_agreement_public_key_sample() -> KeyAgreementPublicKey {
    KeyAgreementPublicKey::sample()
}

#[uniffi::export]
pub fn new_key_agreement_public_key_sample_other() -> KeyAgreementPublicKey {
    KeyAgreementPublicKey::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = KeyAgreementPublicKey;

    #[test]
    fn sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_key_agreement_public_key_sample(),
                new_key_agreement_public_key_sample_other(),
                // duplicates should get removed
                new_key_agreement_public_key_sample(),
                new_key_agreement_public_key_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_key_agreement_public_key_from_hex_valid() {
        let valid_hex = "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8".to_string();
        assert_eq!(
            new_key_agreement_public_key_from_hex(valid_hex.clone()),
            valid_hex.parse()
        );
    }

    #[test]
    fn test_new_key_agreement_public_key_from_hex_invalid() {
        let invalid_hex = "invalid_hex_string".to_string();
        assert_eq!(
            new_key_agreement_public_key_from_hex(invalid_hex.clone()),
            invalid_hex.parse()
        );
    }

    #[test]
    fn test_new_key_agreement_public_key_from_bytes() {
        let valid_bytes = BagOfBytes::sample();
        assert_eq!(
            new_key_agreement_public_key_from_bytes(valid_bytes.clone()),
            SUT::try_from(valid_bytes.to_vec())
        );
    }

    #[test]
    fn test_new_key_agreement_public_key_from_bytes_invalid() {
        let invalid_bytes: BagOfBytes = vec![0, 1, 2].into();
        assert_eq!(
            new_key_agreement_public_key_from_bytes(invalid_bytes.clone()),
            SUT::try_from(invalid_bytes.to_vec())
        );
    }

    #[test]
    fn test_key_agreement_public_key_to_hex() {
        let sut = SUT::sample();
        assert_eq!(key_agreement_public_key_to_hex(&sut), sut.to_hex());
    }

    #[test]
    fn test_key_agreement_public_key_to_bytes() {
        let sut = SUT::sample();
        assert_eq!(
            key_agreement_public_key_to_bytes(&sut),
            BagOfBytes::from(sut.to_bytes())
        );
    }
}
