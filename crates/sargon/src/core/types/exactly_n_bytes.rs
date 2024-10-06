use crate::prelude::*;
use delegate::delegate;

/// Small macro to facilitate generation of UniFFI exported functions.
macro_rules! decl_exactly_n_bytes {
    (
        $(
            #[doc = $expr: expr]
        )*
        $byte_count:literal,
        $exp_sample_value:literal,
    ) => {
        paste! {
            impl From<&[u8; $byte_count]> for BagOfBytes {
                fn from(value: &[u8; $byte_count]) -> BagOfBytes {
                    BagOfBytes::from(value.as_ref())
                }
            }

            $(
                #[doc = $expr]
            )*
            #[derive(
                Zeroize,
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                Ord,
                PartialOrd,
                SerializeDisplay,
                DeserializeFromStr,
                derive_more::Display,
                derive_more::Debug,
            )]
            #[display("{}", self.to_hex())]
            #[debug("{}", self.to_hex())]
            pub struct [<Exactly $byte_count Bytes>]([u8; $byte_count]);

            impl FromStr for [<Exactly $byte_count Bytes>] {
                type Err = crate::CommonError;
                fn from_str(s: &str) -> Result<Self> {
                    BagOfBytes::from_str(s).and_then(|b| Self::try_from(b.as_ref()))
                }
            }

            impl [<Exactly $byte_count Bytes>] {
                pub fn bytes(&self) -> &[u8; $byte_count] {
                    &self.0
                }

                /// Instantiates a new `BagOfBytes` from bytes generated by
                /// a CSPRNG.
                pub fn generate() -> Self {
                    Self::from(&generate_byte_array::<$byte_count>())
                }

                /// Tries to decode the string `s` into this type. Will fail
                // if the string is not valid hex or if the decoded bytes does
                // not have length `N`.
                pub fn from_hex(s: &str) -> Result<Self> {
                   Self::from_str(s)
                }

                pub fn to_hex(self) -> String {
                    hex_encode(self.0)
                }
                pub fn to_vec(self) -> Vec<u8> {
                    self.0.to_vec()
                }
            }


            impl TryFrom<&[u8]> for [<Exactly $byte_count Bytes>] {
                type Error = crate::CommonError;
                fn try_from(value: &[u8]) -> Result<Self> {
                    if value.len() == $byte_count {
                        Ok(Self(value.try_into().unwrap()))
                    } else {
                        Err(CommonError::InvalidByteCount {
                            expected: $byte_count as u64,
                            found: value.len() as u64,
                        })
                    }
                }
            }

            impl TryFrom<Vec<u8>> for [<Exactly $byte_count Bytes>] {
                type Error = CommonError;

                fn try_from(value: Vec<u8>) -> Result<Self> {
                    Self::try_from(value.as_slice())
                }
            }

            impl TryFrom<BagOfBytes> for [<Exactly $byte_count Bytes>] {
                type Error = CommonError;

                fn try_from(value: BagOfBytes) -> Result<Self> {
                    Self::try_from(value.as_ref())
                }
            }


            impl AsRef<[u8]> for [<Exactly $byte_count Bytes>] {
                fn as_ref(&self) -> &[u8] {
                    self.bytes()
                }
            }

            impl From<&[u8; $byte_count]> for [<Exactly $byte_count Bytes>] {
                fn from(value: &[u8; $byte_count]) -> Self {
                    Self(*value)
                }
            }

            impl HasSampleValues for [<Exactly $byte_count Bytes>] {
                /// `deadbeef...``
                /// A sample used to facilitate unit tests.
                fn sample() -> Self {
                    Self::sample_dead()
                }

                /// A sample used to facilitate unit tests.
                fn sample_other() -> Self {
                   Self::sample_fade()
                }
            }

            impl [<Exactly $byte_count Bytes>] {

                fn declare_sample(r: &str) -> Self {
                    let mut s = r.repeat($byte_count / 2).to_owned();

                    let target_len = $byte_count * 2;

                    if s.len() != target_len {
                        let bc = format!("{}", $byte_count);
                        let bc_len = bc.len();
                        let subs = &s[0..target_len-bc_len];
                        s = format!("{}{}", bc, subs);
                        assert_eq!(s.len(), target_len);
                    }
                    Self::from_str(&s).expect("Valid sample")
                }

                /// `aced...``
                /// A sample used to facilitate unit tests.
                pub fn sample_aced() -> Self {
                    Self::declare_sample("aced")
                }

                /// `babe...``
                /// A sample used to facilitate unit tests.
                pub fn sample_babe() -> Self {
                    Self::declare_sample("babe")
                }

                /// `cafe...``
                /// A sample used to facilitate unit tests.
                pub fn sample_cafe() -> Self {
                    Self::declare_sample("cafe")
                }

                /// `dead...``
                /// A sample used to facilitate unit tests.
                pub fn sample_dead() -> Self {
                    Self::declare_sample("dead")
                }

                /// `ecad...``
                /// A sample used to facilitate unit tests.
                pub fn sample_ecad() -> Self {
                    Self::declare_sample("ecad")
                }

                /// `fade...``
                /// A sample used to facilitate unit tests.
                pub fn sample_fade() -> Self {
                    Self::declare_sample("fade")
                }

            }

            #[cfg(test)]
            mod [<tests_ exactly_ $byte_count _bytes >] {

                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = [< Exactly $byte_count Bytes >];

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
                fn ord() {
                    assert!(SUT::sample() < SUT::sample_other());
                }

                #[test]
                fn hash() {
                    assert_eq!(
                        HashSet::<SUT>::from_iter([
                            SUT::sample_aced(),
                            SUT::sample_babe(),
                            SUT::sample_cafe(),
                            SUT::sample_dead(),
                            SUT::sample_ecad(),
                            SUT::sample_fade(),
                            // Duplicates should be removed
                            SUT::sample_aced(),
                            SUT::sample_babe(),
                            SUT::sample_cafe(),
                            SUT::sample_dead(),
                            SUT::sample_ecad(),
                            SUT::sample_fade(),
                        ])
                        .len(),
                        6
                    );
                }

                #[test]
                fn from_roundtrip() {
                    let bytes = &[0u8; $byte_count];
                    assert_eq!(SUT::from(bytes).bytes(), bytes);
                }

                #[test]
                fn from_vec_roundtrip() {
                    let vec = Vec::from([0u8; $byte_count]);
                    let sut: SUT = vec.clone().try_into().unwrap();
                    assert_eq!(sut.to_vec(), vec);
                }

                #[test]
                fn invalid_str() {
                    let s = "invalid str";
                    assert_eq!(
                        SUT::from_str(s),
                        Err(CommonError::StringNotHex {
                            bad_value: s.to_owned()
                        })
                    );
                }

                #[test]
                fn invalid_len() {
                    assert_eq!(
                        SUT::try_from(Vec::from([0u8; 5])),
                        Err(CommonError::InvalidByteCount {
                            expected: $byte_count,
                            found: 5
                        })
                    )
                }

                #[test]
                fn random() {
                    let mut set: HashSet<Vec<u8>> = HashSet::new();
                    let n = 100;
                    for _ in 0..n {
                        let bytes = SUT::generate();
                        set.insert(bytes.to_vec());
                    }
                    assert_eq!(set.len(), n);
                }

                #[test]
                fn from_string_roundtrip() {
                    assert_eq!(SUT::from_hex($exp_sample_value).unwrap().to_string(), $exp_sample_value);
                }

                #[test]
                fn debug() {
                    let hex_bytes = SUT::sample();
                    assert_eq!(format!("{:?}", hex_bytes), $exp_sample_value);
                }

                #[test]
                fn display() {
                    let hex_bytes = SUT::sample();
                    assert_eq!(format!("{}", hex_bytes), $exp_sample_value);
                }

                #[test]
                fn to_hex() {
                    let hex_bytes = SUT::sample();
                    assert_eq!(hex_bytes.to_string(), $exp_sample_value);
                }

                #[test]
                fn as_ref() {
                    let b: &[u8] = &hex_decode(
                        $exp_sample_value
                    )
                    .unwrap();
                    assert_eq!(SUT::try_from(b).unwrap().as_ref(), b);
                }

                #[test]
                fn json_roundtrip() {
                    let model = SUT::sample();
                    assert_json_value_eq_after_roundtrip(
                        &model,
                        json!($exp_sample_value),
                    );
                }

                #[test]
                fn json_roundtrip_fails_for_invalid() {
                    assert_json_value_fails::<SUT>(json!("not even hex"));
                    // too short
                    assert_json_value_fails::<SUT>(json!("deadbeef"));
                    // too long
                    assert_json_value_fails::<SUT>(json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"));
                }
            }
        }
    };
}

decl_exactly_n_bytes!(
    /// 29 bytes, typically used as PublicKeyHash, or otherwise NodeId payload,
    /// implementation wise those bytes are stored inside a `BagOfBytes`
    /// (wrapper of `Vec<u8>`) for UniFFI compat.
    29,
    "29deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 32 bytes, most commonly used fixed length bytes, used by PrivateKeys,
    /// Ed25519PublicKey, and BIP39 entropy, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    32,
    "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 64 bytes, used by Ed25519Signatures, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    64,
    "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 33 bytes, used by Secp256k1PublicKeys, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    33,
    "33deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 65 bytes, used by Secp256k1Signatures, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    65,
    "65deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 12 bytes, used by AES encryption, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    12,
    "deaddeaddeaddeaddeaddead", // expected sample value for tests
);

decl_exactly_n_bytes!(
    /// 60 bytes, used as encrypted mnemonic for security questions factor
    /// source. 32 bytes mnemonic when encrypted results in exactly this length.
    60,
    "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead", // expected sample value for tests
);
