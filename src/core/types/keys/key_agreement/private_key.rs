use crate::prelude::*;
use crypto::keys::x25519::SecretKey as X25519PrivateKey;
use crypto::keys::x25519::SharedSecret;

/// PrivateKey on Curve25519 used for key agreement (ECDH) with some `KeyAgreementPublicKey`.
#[derive(derive_more::Debug)]
#[debug("{}", self.to_hex())]
pub struct KeyAgreementPrivateKey(X25519PrivateKey);

pub type KeyAgreementSharedSecret = SharedSecret;

impl KeyAgreementPrivateKey {
    pub fn generate() -> Result<Self> {
        Exactly32Bytes::generate().to_vec().try_into()
    }

    pub fn public_key(&self) -> KeyAgreementPublicKey {
        self.0.public_key().into()
    }

    pub fn shared_secret_from_key_agreement(
        &self,
        other: &KeyAgreementPublicKey,
    ) -> KeyAgreementSharedSecret {
        self.0.diffie_hellman(&other.secret_magic)
    }
}

impl PartialEq for KeyAgreementPrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.public_key() == other.public_key()
    }
}

impl From<X25519PrivateKey> for KeyAgreementPrivateKey {
    fn from(value: X25519PrivateKey) -> Self {
        Self(value)
    }
}

impl TryFrom<&[u8]> for KeyAgreementPrivateKey {
    type Error = crate::CommonError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        X25519PrivateKey::try_from_slice(slice)
            .map_err(|_| CommonError::InvalidKeyAgreementPrivateKeyFromBytes {
                bad_value: slice.into(),
            })
            .map(Self::from)
    }
}

impl TryFrom<Vec<u8>> for KeyAgreementPrivateKey {
    type Error = CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl TryFrom<Exactly32Bytes> for KeyAgreementPrivateKey {
    type Error = CommonError;

    fn try_from(value: Exactly32Bytes) -> Result<Self, Self::Error> {
        value.to_vec().try_into()
    }
}

impl KeyAgreementPrivateKey {
    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl HasSampleValues for KeyAgreementPrivateKey {
    /// A sample used to facilitate unit tests.
    /// `98df1ecbf042f5dc986f79c332ef64efed6240f407a664b2e9261c1af78e1063`
    ///
    /// expected public key:
    /// `8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10``
    fn sample() -> Self {
        Exactly32Bytes::from_hex(
            "98df1ecbf042f5dc986f79c332ef64efed6240f407a664b2e9261c1af78e1063",
        )
        .unwrap()
        .try_into()
        .unwrap()
    }

    // A sample used to facilitate unit tests.
    /// `4821a2820bb8243a92935ab2e90f86def4f9708c4b406f77c863aed9a35ea142`
    ///
    /// expected public key:
    /// `35478987427834c34a15cba2fe511f66a19a6c20169e24b5493c6a80ebd7334d`
    fn sample_other() -> Self {
        Exactly32Bytes::from_hex(
            "4821a2820bb8243a92935ab2e90f86def4f9708c4b406f77c863aed9a35ea142",
        )
        .unwrap()
        .try_into()
        .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use hex::ToHex;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = KeyAgreementPrivateKey;

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
    fn from_bytes() {
        let bytes = SUT::sample().to_bytes();
        let from_bytes = SUT::try_from(bytes.as_slice()).unwrap();
        assert_eq!(SUT::sample(), from_bytes);
    }

    #[test]
    fn from_bytes_invalid() {
        let bytes = BagOfBytes::new();
        let from_bytes = SUT::try_from(bytes.as_slice());
        assert_eq!(
            from_bytes,
            Err(CommonError::InvalidKeyAgreementPrivateKeyFromBytes {
                bad_value: bytes.into(),
            })
        );
    }

    #[test]
    fn generate() {
        let generate = SUT::generate();
        assert!(generate.is_ok());
    }

    #[test]
    fn public_key() {
        let private_key = SUT::sample();
        let public_key = private_key.public_key();
        println!("{:?}", public_key);
        assert_eq!(
            public_key,
             KeyAgreementPublicKey::from_hex("8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10".to_owned())
             .unwrap()
        );
    }

    #[test]
    fn shared_secret_from_key_agreement() {
        let sample_1 = SUT::sample();
        let other_pb_key = SUT::sample_other().public_key();

        let shared_secret =
            sample_1.shared_secret_from_key_agreement(&other_pb_key);
        let bytes: Exactly32Bytes = shared_secret.as_bytes().into();
        pretty_assertions::assert_eq!(
            bytes.to_hex(),
            "e893692f57baa2a99ece829b98ff4987cd7737e68ff97e00a27d7552e6633429"
                .to_string()
        );
    }

    #[test]
    fn to_hex() {
        let sut = SUT::sample();
        assert_eq!(
            sut.to_hex(),
            "98df1ecbf042f5dc986f79c332ef64efed6240f407a664b2e9261c1af78e1063"
        );
    }
}
