use crate::prelude::*;
use crypto::keys::x25519::SecretKey as X25519PrivateKey;
use hkdf::Hkdf;
use k256::sha2::Sha256;

#[derive(derive_more::Debug)]
#[debug("{}", self.to_hex())]
pub struct KeyAgreementPrivateKey(X25519PrivateKey);

impl KeyAgreementPrivateKey {
    pub fn generate() -> Result<Self> {
        Exactly32Bytes::generate().to_vec().try_into()
    }

    pub fn public_key(&self) -> KeyAgreementPublicKey {
        self.0.public_key().into()
    }

    pub fn hkdf_key_agreement(
        &self,
        other: &KeyAgreementPublicKey,
        salt: &[u8],
        info: &[u8],
    ) -> Result<Exactly32Bytes> {
        let shared_secret = self.0.diffie_hellman(&other.0).to_bytes();
        let mut symmetric_key = [0u8; 32]; // 32-byte buffer for the symmetric key

        let hkdf = Hkdf::<Sha256>::new(Some(salt), &shared_secret);
        hkdf.expand(info, &mut symmetric_key).map_err(|err| {
            CommonError::HkdfExpandFailed {
                underlying: err.to_string(),
            }
        })?;

        Ok(Exactly32Bytes::from(&symmetric_key))
    }
}

impl PartialEq for KeyAgreementPrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
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
    fn hkdf_key_agreement() {
        // Test vector
        let json_test_vector = r#"
        {
            "wallet_private_key": "98df1ecbf042f5dc986f79c332ef64efed6240f407a664b2e9261c1af78e1063",
            "dapp_public_key": "8679bc1fe3210b2ce84793668b05218fdc4c220bc05387b7d2ac0d4c7b7c5d10",
            "salt": "000102030405060708090a0b0c0d0e0f",
            "info": "f0f1f2f3f4f5f6f7f8f9",
            "expected_encryption_key": "9dd1c0134cf081d7d4e4bdeeac578bfe9d09fb35810fad9567ef6f2ec3f0201e"
        }"#;

        let decoded: serde_json::Value =
            serde_json::from_str(json_test_vector).unwrap();
        let private_key = KeyAgreementPrivateKey::try_from(
            hex_decode(decoded["wallet_private_key"].as_str().unwrap())
                .unwrap(),
        )
        .unwrap();

        let public_key = KeyAgreementPublicKey::from_hex(
            decoded["dapp_public_key"].as_str().unwrap().to_owned(),
        )
        .unwrap();

        let salt = hex_decode(decoded["salt"].as_str().unwrap()).unwrap();
        let info = hex_decode(decoded["info"].as_str().unwrap()).unwrap();

        let encryption_key = private_key
            .hkdf_key_agreement(&public_key, &salt, &info)
            .unwrap();
        let expected_encryption_key = Exactly32Bytes::from_hex(
            decoded["expected_encryption_key"].as_str().unwrap(),
        )
        .unwrap();

        assert_eq!(encryption_key, expected_encryption_key);
    }
}
