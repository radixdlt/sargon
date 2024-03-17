use crate::{prelude::*, UniffiCustomTypeConverter};

use bip32::secp256k1::PublicKey as BIP32Secp256k1PublicKey; // the bip32 crate actually does validation of the PublicKey whereas `radix_engine_common` does not.

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
    uniffi::Record,
)]
#[display("{}", self.to_hex())]
#[debug("{}", self.to_hex())]
pub struct Secp256k1PublicKey {
    secret_magic: ScryptoSecp256k1PublicKey,
}

impl From<Secp256k1PublicKey> for ScryptoSecp256k1PublicKey {
    fn from(value: Secp256k1PublicKey) -> Self {
        value.secret_magic
    }
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
pub fn new_secp256k1_public_key_sample() -> Secp256k1PublicKey {
    Secp256k1PublicKey::sample()
}

#[uniffi::export]
pub fn new_secp256k1_public_key_sample_other() -> Secp256k1PublicKey {
    Secp256k1PublicKey::sample_other()
}

impl IsPublicKey<Secp256k1Signature> for Secp256k1PublicKey {
    /// Verifies an ECDSA signature over Secp256k1.
    fn is_valid(
        &self,
        signature: &Secp256k1Signature,
        for_hash: &impl ScryptoIsHash,
    ) -> bool {
        scrypto_verify_secp256k1(
            for_hash.as_hash(),
            &self.scrypto(),
            &(*signature).into(),
        )
    }
}

impl Secp256k1PublicKey {
    pub(crate) fn scrypto(&self) -> ScryptoSecp256k1PublicKey {
        self.secret_magic
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.scrypto().to_vec()
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
            .map(|_| Self {
                secret_magic: value,
            })
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
        assert!(!SUT::sample()
            .is_valid(&Secp256k1Signature::sample(), &Hash::sample()));
        assert!(!SUT::sample()
            .is_valid(&Secp256k1Signature::sample(), &Hash::sample_other()));

        assert!(!SUT::sample()
            .is_valid(&Secp256k1Signature::sample_other(), &Hash::sample()));
        assert!(!SUT::sample().is_valid(
            &Secp256k1Signature::sample_other(),
            &Hash::sample_other()
        ));

        assert!(!SUT::sample_other()
            .is_valid(&Secp256k1Signature::sample(), &Hash::sample()));
        assert!(!SUT::sample_other()
            .is_valid(&Secp256k1Signature::sample(), &Hash::sample_other()));

        assert!(!SUT::sample_other()
            .is_valid(&Secp256k1Signature::sample_other(), &Hash::sample()));
        assert!(!SUT::sample_other().is_valid(
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
        assert!(sut.is_valid(&signature, &hash));
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{
        new_secp256k1_public_key_from_bytes, new_secp256k1_public_key_from_hex,
        new_secp256k1_public_key_sample, new_secp256k1_public_key_sample_other,
        secp256k1_public_key_to_bytes, secp256k1_public_key_to_hex,
        HasSampleValues,
    };

    use super::Secp256k1PublicKey;

    #[test]
    fn equality_samples() {
        assert_eq!(
            Secp256k1PublicKey::sample(),
            new_secp256k1_public_key_sample()
        );
        assert_eq!(
            Secp256k1PublicKey::sample_other(),
            new_secp256k1_public_key_sample_other()
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
