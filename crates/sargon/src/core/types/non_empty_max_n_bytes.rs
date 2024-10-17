use crate::prelude::*;
use delegate::delegate;
use paste::*;
use radix_common::crypto::{Hash, IsHash};

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_non_empty_max_n_bytes {
    (
        $(
            #[doc = $expr: expr]
        )*
        $byte_count:literal
    ) => {
        paste! {
            $(
                #[doc = $expr]
            )*
            #[derive(
                Zeroize, // Not `ZeroizeOnDrop`: we dont wanna zeroize all byte types: use `decl_secret_bytes!` for secrets.
                Clone,
                PartialEq,
                Eq,
                Hash,
                SerializeDisplay,
                Ord,
                PartialOrd,
                DeserializeFromStr,
                derive_more::Display,
                derive_more::Debug,
            )]
            #[display("{}", self.to_hex())]
            #[debug("{}", self.to_hex())]
            pub struct [< NonEmptyMax $byte_count Bytes  >] {
                pub bag_of_bytes: BagOfBytes,
            }

            impl TryFrom<BagOfBytes> for [< NonEmptyMax $byte_count Bytes  >] {
                type Error = CommonError;

                fn try_from(value: BagOfBytes) -> Result<Self> {
                    if value.is_empty() {
                        return Err(CommonError::BytesEmpty);
                    }
                    if value.len() > $byte_count {
                        return Err(CommonError::TooManyBytes {
                            max: $byte_count as u64,
                            found: value.len() as u64,
                        });
                    }
                    Ok(Self {
                        bag_of_bytes: value,
                    })
                }
            }

            impl [< NonEmptyMax $byte_count Bytes  >] {

                /// Instantiates from bytes generated by a CSPRNG.
                pub fn generate() -> Self {
                    [< NonEmptyMax $byte_count Bytes  >] {
                        bag_of_bytes: BagOfBytes::from(
                            generate_bytes::<$byte_count>(),
                        ),
                    }
                }

                /// Tries to decode the string `s` into a `[< NonEmptyMax $byte_count Bytes  >]`. Will fail
                /// if the string is not valid hex or if the decoded bytes does
                /// not have length `$byte_count`.
                pub fn from_hex(s: &str) -> Result<Self> {
                    Self::from_str(s)
                }

                /// Instantiates a new `[< NonEmptyMax $byte_count Bytes  >]` from the $byte_count bytes, by cloning them.
                pub fn from_bytes(bytes: &[u8; $byte_count]) -> Self {
                    let bytes: &[u8] = bytes.as_slice().into();
                    let bag_of_bytes: BagOfBytes = bytes.into();
                    Self { bag_of_bytes }
                }
            }

            impl [< NonEmptyMax $byte_count Bytes  >] {
                /// Returns a references to the inner array slice.
                pub fn bytes(&self) -> [u8; $byte_count] {
                    self.bag_of_bytes
                        .to_vec()
                        .as_slice()
                        .try_into()
                        .expect("$byte_count bytes")
                }
            }

            impl TryFrom<Vec<u8>> for [< NonEmptyMax $byte_count Bytes  >] {
                type Error = CommonError;

                fn try_from(value: Vec<u8>) -> Result<Self> {
                    BagOfBytes::from(value).try_into()
                }
            }

            impl AsRef<[u8]> for [< NonEmptyMax $byte_count Bytes  >] {
                fn as_ref(&self) -> &[u8] {
                    self.bag_of_bytes.as_ref()
                }
            }

            impl TryFrom<&[u8]> for [< NonEmptyMax $byte_count Bytes  >] {
                type Error = CommonError;

                fn try_from(value: &[u8]) -> Result<Self> {
                   Self::try_from(Vec::from(value))
                }
            }

            impl FromStr for [< NonEmptyMax $byte_count Bytes  >] {
                type Err = CommonError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    s.parse::<BagOfBytes>().and_then(|v| v.try_into())
                }
            }

            impl [< NonEmptyMax $byte_count Bytes  >] {
                delegate! {
                    to self.bag_of_bytes{
                        pub fn to_hex(&self) -> String;
                        pub fn to_vec(&self) -> Vec<u8>;
                    }
                }
            }
        }
    };
}

decl_non_empty_max_n_bytes!(
    /// 64 bytes, typically used by NonFungibleLocalId::Bytes
    64
);

decl_non_empty_max_n_bytes!(
    /// 32 bytes, typically used as entropy for Mnemonics.
    32
);

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_samples_for_max_n_bytes {
    ($struct_name:ident, $byte_count:expr) => {
        impl HasSampleValues for $struct_name {
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

        impl $struct_name {
            /// `aced...``
            /// A sample used to facilitate unit tests.
            pub fn sample_aced() -> Self {
                Self::from_str(&"aced".repeat($byte_count / 4))
                    .expect("aced...")
            }

            /// `babe...``
            /// A sample used to facilitate unit tests.
            pub fn sample_babe() -> Self {
                Self::from_str(&"babe".repeat($byte_count / 2))
                    .expect("babe...")
            }

            /// `cafe...``
            /// A sample used to facilitate unit tests.
            pub fn sample_cafe() -> Self {
                Self::from_str(&"cafe".repeat($byte_count / 4))
                    .expect("cafe...")
            }

            /// `dead...``
            /// A sample used to facilitate unit tests.
            pub fn sample_dead() -> Self {
                Self::from_str(&"dead".repeat($byte_count / 2))
                    .expect("dead...")
            }

            /// `ecad...``
            /// A sample used to facilitate unit tests.
            pub fn sample_ecad() -> Self {
                Self::from_str(&"ecad".repeat($byte_count / 4))
                    .expect("ecad...")
            }

            /// `fade...``
            /// A sample used to facilitate unit tests.
            pub fn sample_fade() -> Self {
                Self::from_str(&"fade".repeat($byte_count / 2))
                    .expect("fade...")
            }
        }
    };
}

// The impl of sample values require an max number of bytes
decl_samples_for_max_n_bytes!(NonEmptyMax64Bytes, 64);
decl_samples_for_max_n_bytes!(NonEmptyMax32Bytes, 32);

#[cfg(test)]
mod tests_non_empty_max_64_bytes {

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonEmptyMax64Bytes;

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
    fn cannot_be_empty() {
        assert_eq!(SUT::from_hex(""), Err(CommonError::BytesEmpty));
    }

    #[test]
    fn can_have_len_1() {
        assert_eq!(SUT::from_hex("de").unwrap().to_vec(), vec![0xde]);
    }

    #[test]
    fn zeroize() {
        let mut sut = SUT::sample();
        sut.zeroize();
        assert_ne!(sut, SUT::sample());
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
    fn from_string_roundtrip() {
        let str =
            "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002";
        assert_eq!(SUT::from_hex(str).unwrap().to_string(), str);
    }

    #[test]
    fn debug() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{:?}", hex_bytes), str);
    }

    #[test]
    fn display() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{}", hex_bytes), str);
    }

    #[test]
    fn to_hex() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(hex_bytes.to_string(), str);
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!("not even hex"));
        assert_json_value_fails::<SUT>(json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"));
    }

    #[test]
    fn from_bytes_roundtrip() {
        let bytes = [0u8; 64];
        assert_eq!(SUT::from_bytes(&bytes).bytes(), bytes);
    }

    #[test]
    fn from_vec_roundtrip() {
        let vec = Vec::from([0u8; 64]);
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
    fn too_many_len() {
        assert_eq!(
            SUT::try_from(Vec::from([0u8; 65])),
            Err(CommonError::TooManyBytes { max: 64, found: 65 })
        )
    }

    #[test]
    fn as_ref() {
        let b: &[u8] = &hex_decode(
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead",
        )
        .unwrap();
        assert_eq!(SUT::try_from(b).unwrap().as_ref(), b);
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
}
