use crate::prelude::*;

use k256::ecdsa::VerifyingKey as K256PublicKey;

/// A `secp256k1` public key used to verify cryptographic signatures (ECDSA signatures).
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
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Secp256k1PublicKey(pub ScryptoSecp256k1PublicKey);

impl From<Secp256k1PublicKey> for ScryptoSecp256k1PublicKey {
    fn from(value: Secp256k1PublicKey) -> Self {
        value.0
    }
}

impl IsPublicKey<Secp256k1Signature> for Secp256k1PublicKey {
    /// Verifies an ECDSA signature over Secp256k1.
    fn is_valid_signature_for_hash(
        &self,
        signature: &Secp256k1Signature,
        hash: &impl ScryptoIsHash,
    ) -> bool {
        scrypto_verify_secp256k1(
            hash.as_hash(),
            &self.scrypto(),
            &(*signature).into(),
        )
    }
}

impl Secp256k1PublicKey {
    pub(crate) fn scrypto(&self) -> ScryptoSecp256k1PublicKey {
        self.0
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.scrypto().to_vec()
    }

    pub fn to_bag_of_bytes(&self) -> BagOfBytes {
        self.to_bytes().into()
    }

    pub fn to_hex(&self) -> String {
        hex_encode(self.to_bytes())
    }

    pub fn uncompressed(&self) -> Vec<u8> {
        K256PublicKey::from_sec1_bytes(&self.to_bytes())
            .expect("should always be able to create a BIP32 PublicKey")
            .to_encoded_point(false)
            .as_bytes()
            .to_owned()
    }
}

impl TryFrom<ScryptoSecp256k1PublicKey> for Secp256k1PublicKey {
    type Error = CommonError;

    fn try_from(value: ScryptoSecp256k1PublicKey) -> Result<Self, Self::Error> {
        <Self as TryFrom<&[u8]>>::try_from(&value.to_vec())
    }
}

impl TryFrom<Vec<u8>> for Secp256k1PublicKey {
    type Error = CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_slice())
    }
}

/// Correct amount of bytes, however, the contents of the bytes
/// have not been checked to see it is a valid Secp256k1PublicKey.
pub enum Secp256k1PublicKeyUncheckedBytes {
    Compressed(Exactly33Bytes),
    Uncompressed(Exactly65Bytes),
}
impl AsRef<[u8]> for Secp256k1PublicKeyUncheckedBytes {
    fn as_ref(&self) -> &[u8] {
        match self {
            Secp256k1PublicKeyUncheckedBytes::Compressed(bytes) => {
                bytes.as_ref()
            }
            Secp256k1PublicKeyUncheckedBytes::Uncompressed(bytes) => {
                bytes.as_ref()
            }
        }
    }
}
impl TryFrom<&[u8]> for Secp256k1PublicKeyUncheckedBytes {
    type Error = CommonError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Exactly33Bytes::try_from(value)
            .map(Self::Compressed)
            .or(Exactly65Bytes::try_from(value).map(Self::Uncompressed))
            .map_err(|_| CommonError::InvalidSecp256k1PublicKeyFromBytes {
                bad_value: BagOfBytes::from(value),
            })
    }
}

impl TryFrom<Secp256k1PublicKeyUncheckedBytes> for Secp256k1PublicKey {
    type Error = CommonError;

    fn try_from(
        value: Secp256k1PublicKeyUncheckedBytes,
    ) -> Result<Self, Self::Error> {
        K256PublicKey::from_sec1_bytes(value.as_ref())
            .map_err(|_| CommonError::InvalidSecp256k1PublicKeyPointNotOnCurve)
            .map(|key| {
                ScryptoSecp256k1PublicKey::try_from(
                    key.to_encoded_point(true).as_ref(),
                )
                .expect("Discrepancy: Scrypto's Secp256k1PublicKey library considers key invalid, but BIP32 crate considers it valid. We trust BIP32 crate.")
            })
            .map(Self)
    }
}

impl TryFrom<Exactly33Bytes> for Secp256k1PublicKey {
    type Error = CommonError;

    fn try_from(value: Exactly33Bytes) -> Result<Self, Self::Error> {
        Self::try_from(Secp256k1PublicKeyUncheckedBytes::Compressed(value))
    }
}

impl TryFrom<Exactly65Bytes> for Secp256k1PublicKey {
    type Error = CommonError;

    fn try_from(value: Exactly65Bytes) -> Result<Self, Self::Error> {
        Self::try_from(Secp256k1PublicKeyUncheckedBytes::Uncompressed(value))
    }
}

impl TryFrom<&[u8]> for Secp256k1PublicKey {
    type Error = CommonError;

    fn try_from(slice: &[u8]) -> Result<Self> {
        Secp256k1PublicKeyUncheckedBytes::try_from(slice)
            .and_then(Self::try_from)
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

impl HasSampleValues for Secp256k1PublicKey {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_alice()
    }

    fn sample_other() -> Self {
        Self::sample_bob()
    }
}

impl Secp256k1PublicKey {
    pub fn sample_alice() -> Self {
        Secp256k1PrivateKey::sample_alice().public_key()
    }

    pub fn sample_bob() -> Self {
        Secp256k1PrivateKey::sample_bob().public_key()
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
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Secp256k1PublicKey;

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
    fn from_str() {
        assert!(SUT::from_str(
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        )
        .is_ok());
    }

    #[test]
    fn uncompressed_hex() {
        // https://github.com/Sajjon/K1/blob/main/Tests/K1Tests/TestCases/Keys/PublicKey/PublicKeyImportTests.swift#L48
        assert_eq!(
            SUT::from_str("040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2").unwrap().to_hex(),
            // compressed is this...
            "020202020202020202020202020202020202020202020202020202020202020202"
        );

        assert_eq!(
            hex_encode(SUT::from_str("020202020202020202020202020202020202020202020202020202020202020202").unwrap().uncompressed()),
            "040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2"
        );
    }

    #[test]
    fn from_exactly_65_bytes() {
        // https://github.com/Sajjon/K1/blob/main/Tests/K1Tests/TestCases/Keys/PublicKey/PublicKeyImportTests.swift#L48
        let bytes = Exactly65Bytes::from_str("040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2").unwrap();

        assert_eq!(
            SUT::try_from(bytes).unwrap().to_hex(),
            // compressed is this...
            "020202020202020202020202020202020202020202020202020202020202020202"
        );
    }

    #[test]
    fn bytes_roundtrip() {
        let bytes: &[u8] = &[
            0x02, 0x51, 0x7b, 0x88, 0x91, 0x6e, 0x7f, 0x31, 0x5b, 0xb6, 0x82,
            0xf9, 0x92, 0x6b, 0x14, 0xbc, 0x67, 0xa0, 0xe4, 0x24, 0x6f, 0x8a,
            0x41, 0x9b, 0x98, 0x62, 0x69, 0xe1, 0xa7, 0xe6, 0x1f, 0xff, 0xa7,
        ];
        let key = SUT::try_from(bytes).unwrap();
        assert_eq!(
            key.to_hex(),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn from_exactly_33_bytes() {
        let bytes = Exactly33Bytes::from_str("020202020202020202020202020202020202020202020202020202020202020202").unwrap();

        assert_eq!(
            hex_encode(SUT::try_from(bytes).unwrap().uncompressed()),
            // compressed is this...
            "040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2"
        );
    }

    #[test]
    fn sample_alice() {
        assert_eq!(
            SUT::sample_alice().to_hex(),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
    }

    #[test]
    fn from_scrypto() {
        let from_scrypto: SUT = ScryptoSecp256k1PublicKey::from_str(
            "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8",
        )
        .unwrap()
        .try_into()
        .unwrap();

        assert_eq!(
            from_scrypto,
            SUT::from_str(
                "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"
            )
            .unwrap()
        );

        // and back
        assert_eq!(
            SUT::try_from(ScryptoSecp256k1PublicKey::from(from_scrypto))
                .unwrap(),
            from_scrypto
        );
    }

    #[test]
    fn sample_bob() {
        assert_eq!(
            SUT::sample_bob().to_hex(),
            "033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"
        );
    }

    #[test]
    fn invalid_hex_str() {
        assert_eq!(
            SUT::from_str("hi"),
            Err(CommonError::InvalidSecp256k1PublicKeyFromString {
                bad_value: "hi".to_owned()
            })
        );
    }

    #[test]
    fn invalid_str_too_short() {
        assert_eq!(
            SUT::from_str("dead"),
            Err(CommonError::InvalidSecp256k1PublicKeyFromBytes {
                bad_value: vec![0xde, 0xad].into()
            })
        );
    }

    #[test]
    fn invalid_bytes() {
        let bytes: &[u8] = &[0u8];
        assert_eq!(
            SUT::try_from(bytes),
            Err(CommonError::InvalidSecp256k1PublicKeyFromBytes {
                bad_value: bytes.to_vec().into()
            })
        );
    }

    #[test]
    fn invalid_key_not_on_curve() {
        assert_eq!(
            SUT::from_str(
                "99deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
            ),
            Err(CommonError::InvalidSecp256k1PublicKeyPointNotOnCurve)
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::sample_alice()),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
    }

    #[test]
    fn json() {
        let model = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"),
        )
    }

    #[test]
    fn try_into_from_str() {
        let str = "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7";
        let key: SUT = str.parse().unwrap();
        assert_eq!(key.to_hex(), str);
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([SUT::sample_alice(), SUT::sample_alice()])
                .len(),
            1
        );
    }

    #[test]
    fn is_valid_is_false_for_mismatch() {
        assert!(!SUT::sample().is_valid_signature_for_hash(
            &Secp256k1Signature::sample(),
            &Hash::sample()
        ));
        assert!(!SUT::sample().is_valid_signature_for_hash(
            &Secp256k1Signature::sample(),
            &Hash::sample_other()
        ));

        assert!(!SUT::sample().is_valid_signature_for_hash(
            &Secp256k1Signature::sample_other(),
            &Hash::sample()
        ));
        assert!(!SUT::sample().is_valid_signature_for_hash(
            &Secp256k1Signature::sample_other(),
            &Hash::sample_other()
        ));

        assert!(!SUT::sample_other().is_valid_signature_for_hash(
            &Secp256k1Signature::sample(),
            &Hash::sample()
        ));
        assert!(!SUT::sample_other().is_valid_signature_for_hash(
            &Secp256k1Signature::sample(),
            &Hash::sample_other()
        ));

        assert!(!SUT::sample_other().is_valid_signature_for_hash(
            &Secp256k1Signature::sample_other(),
            &Hash::sample()
        ));
        assert!(!SUT::sample_other().is_valid_signature_for_hash(
            &Secp256k1Signature::sample_other(),
            &Hash::sample_other()
        ));
    }

    #[test]
    fn is_valid_is_true_for_valid() {
        let sut: SUT = "02f0d85a3b9082683f689e6115f37e1e24b7448fff14b14877e3a4e750e86fba8b".parse().unwrap();
        let message = "All those moments will be lost in time, like tears in rain. Time to die...";
        let hash = hash_of(message.as_bytes());
        let signature: Secp256k1Signature = "01aa1c4f46f8437b7f8ec9008ae10e6f33bb8be3e81e35c63f3498070dfbd6a20b2daee6073ead3c9e72d8909bc32a02e46cede3885cf8568d4c380ac97aa7fbcd".parse().unwrap();
        assert!(sut.is_valid_signature_for_hash(&signature, &hash));
    }
}
