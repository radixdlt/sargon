use crate::prelude::*;
use delegate::delegate;
use paste::*;
use radix_engine_common::crypto::{Hash, IsHash};

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_exactly_n_bytes {
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
                Clone,
                PartialEq,
                Eq,
                Hash,
                Ord,
                PartialOrd,
                SerializeDisplay,
                DeserializeFromStr,
                derive_more::Display,
                derive_more::Debug,
                uniffi::Record,
            )]
            #[display("{}", self.to_hex())]
            #[debug("{}", self.to_hex())]
            pub struct [< Exactly $byte_count Bytes  >] {
                bag_of_bytes: BagOfBytes,
            }

            #[uniffi::export]
            pub fn [<new_exactly_ $byte_count _bytes>](
                bytes: BagOfBytes,
            ) -> Result<[< Exactly $byte_count Bytes  >]> {
                [< Exactly $byte_count Bytes  >]::try_from(bytes)
            }

            impl TryFrom<BagOfBytes> for [< Exactly $byte_count Bytes  >] {
                type Error = CommonError;

                fn try_from(value: BagOfBytes) -> Result<Self> {
                    if value.len() != $byte_count {
                        return Err(CommonError::InvalidByteCount {
                            expected: $byte_count as u64,
                            found: value.len() as u64,
                        });
                    }
                    Ok(Self {
                        bag_of_bytes: value,
                    })
                }
            }

            impl [< Exactly $byte_count Bytes  >] {

                /// Instantiates a new `BagOfBytes` from bytes generated by
                /// a CSPRNG.
                pub fn generate() -> Self {
                    [< Exactly $byte_count Bytes  >] {
                        bag_of_bytes: BagOfBytes::from(
                            generate_bytes::<$byte_count>(),
                        ),
                    }
                }

                /// Tries to decode the string `s` into a `[< Exactly $byte_count Bytes  >]`. Will fail
                /// if the string is not valid hex or if the decoded bytes does
                /// not have length `$byte_count`.
                pub fn from_hex(s: &str) -> Result<Self> {
                    Self::from_str(s)
                }

                /// Instantiates a new `[< Exactly $byte_count Bytes  >]` from the $byte_count bytes, by cloning them.
                pub fn from_bytes(bytes: &[u8; $byte_count]) -> Self {
                    let bytes: &[u8] = bytes.as_slice().into();
                    let bag_of_bytes: BagOfBytes = bytes.into();
                    Self { bag_of_bytes }
                }
            }

            impl [< Exactly $byte_count Bytes  >] {
                /// Returns a references to the inner array slice.
                pub fn bytes(&self) -> [u8; $byte_count] {
                    self.bag_of_bytes
                        .to_vec()
                        .as_slice()
                        .try_into()
                        .expect("$byte_count bytes")
                }
            }

            impl TryFrom<Vec<u8>> for [< Exactly $byte_count Bytes  >] {
                type Error = CommonError;

                fn try_from(value: Vec<u8>) -> Result<Self> {
                    BagOfBytes::from(value).try_into()
                }
            }

            impl AsRef<[u8]> for [< Exactly $byte_count Bytes  >] {
                fn as_ref(&self) -> &[u8] {
                    self.bag_of_bytes.as_ref()
                }
            }

            impl TryFrom<&[u8]> for [< Exactly $byte_count Bytes  >] {
                type Error = CommonError;

                fn try_from(value: &[u8]) -> Result<Self> {
                   Self::try_from(Vec::from(value))
                }
            }

            impl From<&[u8; $byte_count]> for [< Exactly $byte_count Bytes  >] {

                fn from(value: &[u8; $byte_count]) -> Self {
                    Self { bag_of_bytes: value.into() }
                }
            }

            impl From<&[u8; $byte_count]> for BagOfBytes {
                fn from(value: &[u8; $byte_count]) -> Self {
                    Self {
                        bytes: value.to_vec(),
                    }
                }
            }

            impl FromStr for [< Exactly $byte_count Bytes  >] {
                type Err = CommonError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    s.parse::<BagOfBytes>().and_then(|v| v.try_into())
                }
            }

            impl [< Exactly $byte_count Bytes  >] {
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

decl_exactly_n_bytes!(
    /// 29 bytes, typically used as PublicKeyHash, or otherwise NodeId payload,
    /// implementation wise those bytes are stored inside a `BagOfBytes`
    /// (wrapper of `Vec<u8>`) for UniFFI compat.
    29
);

decl_exactly_n_bytes!(
    /// 32 bytes, most commonly used fixed length bytes, used by PrivateKeys,
    /// Ed25519PublicKey, and BIP39 entropy, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    32
);

decl_exactly_n_bytes!(
    /// 64 bytes, used by Ed25519Signatures, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    64
);

decl_exactly_n_bytes!(
    /// 33 bytes, used by Secp256k1PublicKeys, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    33
);

decl_exactly_n_bytes!(
    /// 65 bytes, used by Secp256k1Signatures, implementation wise those bytes are
    /// stored inside a `BagOfBytes` (wrapper of `Vec<u8>`) for UniFFI compat.
    65
);

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_samples_for_bag_of_n_bytes {
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
                Self::from_str(&"aced".repeat($byte_count / 2))
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
                Self::from_str(&"cafe".repeat($byte_count / 2))
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
                Self::from_str(&"ecad".repeat($byte_count / 2))
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

// The impl of sample values require an equal number of bytes
decl_samples_for_bag_of_n_bytes!(Exactly32Bytes, 32);
decl_samples_for_bag_of_n_bytes!(Exactly64Bytes, 64);

impl HasSampleValues for Exactly33Bytes {
    /// `33deadbeefdead...``
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        let s = "dead".repeat(16);
        Self::from_str(&format!("33{s}"))
            .expect("Should have declared a valid Exactly33Bytes sample")
    }

    /// `33beefbeefbeef...``
    /// Another sample used to facilitate unit tests.
    fn sample_other() -> Self {
        let s = "beef".repeat(16);
        Self::from_str(&format!("33{s}"))
            .expect("Should have declared a valid Exactly33Bytes sample")
    }
}

impl HasSampleValues for Exactly65Bytes {
    /// `65deadbeefdead...``
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        let s = "dead".repeat(32);
        Self::from_str(&format!("65{s}"))
            .expect("Should have declared a valid Exactly65Bytes sample")
    }

    /// `65beefbeefbeef...``
    /// Another sample used to facilitate unit tests.
    fn sample_other() -> Self {
        let s = "beef".repeat(32);
        Self::from_str(&format!("65{s}"))
            .expect("Should have declared a valid Exactly65Bytes sample")
    }
}

impl HasSampleValues for Exactly29Bytes {
    /// `29deadbeefdead...``
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        let s = "dead".repeat(14);
        Self::from_str(&format!("29{s}"))
            .expect("Should have declared a valid Exactly29Bytes sample")
    }

    /// `20beefbeefbeef...``
    /// Another sample used to facilitate unit tests.
    fn sample_other() -> Self {
        let s = "beef".repeat(14);
        Self::from_str(&format!("29{s}"))
            .expect("Should have declared a valid Exactly29Bytes sample")
    }
}

impl From<Hash> for Exactly32Bytes {
    /// Instantiates a new `[< Exactly $byte_count Bytes  >]` from the `Hash` ($byte_count bytes).
    fn from(value: Hash) -> Self {
        Self::from_bytes(&value.into_bytes())
    }
}

#[cfg(test)]
mod tests_exactly32_bytes {

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Exactly32Bytes;

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
    fn from_string_roundtrip() {
        let str =
            "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(SUT::from_hex(str).unwrap().to_string(), str);
    }

    #[test]
    fn debug() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{:?}", hex_bytes), str);
    }

    #[test]
    fn display() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{}", hex_bytes), str);
    }

    #[test]
    fn to_hex() {
        let str =
            "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(hex_bytes.to_string(), str);
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
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!("not even hex"));
        assert_json_value_fails::<SUT>(json!("deadbeef"));
        assert_json_value_fails::<SUT>(json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"));
    }

    #[test]
    fn from_bytes_roundtrip() {
        let bytes = [0u8; 32];
        assert_eq!(SUT::from_bytes(&bytes).bytes(), bytes);
    }

    #[test]
    fn from_vec_roundtrip() {
        let vec = Vec::from([0u8; 32]);
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
                expected: 32,
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
}

#[cfg(test)]
mod hex32_uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn new_from_bag_of_bytes() {
        let bytes = generate_bytes::<32>();
        assert_eq!(
            new_exactly_32_bytes(bytes.clone().into()).unwrap().to_vec(),
            bytes
        );
    }

    #[test]
    fn new_fail() {
        assert!(new_exactly_32_bytes(generate_bytes::<5>().into()).is_err());
    }
}

// Copy paste

#[cfg(test)]
mod tests_exactly64_bytes {

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Exactly64Bytes;

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
        assert_json_value_fails::<SUT>(json!("deadbeef"));
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
    fn invalid_len() {
        assert_eq!(
            SUT::try_from(Vec::from([0u8; 5])),
            Err(CommonError::InvalidByteCount {
                expected: 64,
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
}

#[cfg(test)]
mod hex64_uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn new_from_bag_of_bytes() {
        let bytes = generate_bytes::<64>();
        assert_eq!(
            new_exactly_64_bytes(bytes.clone().into()).unwrap().to_vec(),
            bytes
        );
    }

    #[test]
    fn new_fail() {
        assert!(new_exactly_64_bytes(generate_bytes::<5>().into()).is_err());
    }
}

// Copy paste

#[cfg(test)]
mod tests_exactly33_bytes {

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Exactly33Bytes;

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
    fn from_string_roundtrip() {
        let str =
            "100000000000000000000000000000000000000000000000000000000000000002";
        assert_eq!(SUT::from_hex(str).unwrap().to_string(), str);
    }

    #[test]
    fn debug() {
        let str =
            "33deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{:?}", hex_bytes), str);
    }

    #[test]
    fn display() {
        let str =
            "33deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{}", hex_bytes), str);
    }

    #[test]
    fn to_hex() {
        let str =
            "33deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(hex_bytes.to_string(), str);
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("33deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!("not even hex"));
        assert_json_value_fails::<SUT>(json!("deadbeef"));
        assert_json_value_fails::<SUT>(json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"));
    }

    #[test]
    fn from_bytes_roundtrip() {
        let bytes = [0u8; 33];
        assert_eq!(SUT::from_bytes(&bytes).bytes(), bytes);
    }

    #[test]
    fn from_vec_roundtrip() {
        let vec = Vec::from([0u8; 33]);
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
                expected: 33,
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
}

#[cfg(test)]
mod hex33_uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn new_from_bag_of_bytes() {
        let bytes = generate_bytes::<33>();
        assert_eq!(
            new_exactly_33_bytes(bytes.clone().into()).unwrap().to_vec(),
            bytes
        );
    }

    #[test]
    fn new_fail() {
        assert!(new_exactly_33_bytes(generate_bytes::<5>().into()).is_err());
    }
}

// Copy paste

#[cfg(test)]
mod tests_exactly65_bytes {

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Exactly65Bytes;

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
    fn from_string_roundtrip() {
        let str =
            "6510000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002";
        assert_eq!(SUT::from_hex(str).unwrap().to_string(), str);
    }

    #[test]
    fn debug() {
        let str =
            "65deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{:?}", hex_bytes), str);
    }

    #[test]
    fn display() {
        let str =
            "65deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{}", hex_bytes), str);
    }

    #[test]
    fn to_hex() {
        let str =
            "65deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(hex_bytes.to_string(), str);
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("65deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!("not even hex"));
        assert_json_value_fails::<SUT>(json!("deadbeef"));
        assert_json_value_fails::<SUT>(json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"));
    }

    #[test]
    fn from_bytes_roundtrip() {
        let bytes = [0u8; 65];
        assert_eq!(SUT::from_bytes(&bytes).bytes(), bytes);
    }

    #[test]
    fn from_vec_roundtrip() {
        let vec = Vec::from([0u8; 65]);
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
                expected: 65,
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
}

#[cfg(test)]
mod hex65_uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn new_from_bag_of_bytes() {
        let bytes = generate_bytes::<65>();
        assert_eq!(
            new_exactly_65_bytes(bytes.clone().into()).unwrap().to_vec(),
            bytes
        );
    }

    #[test]
    fn new_fail() {
        assert!(new_exactly_65_bytes(generate_bytes::<5>().into()).is_err());
    }
}

// Copy paste

#[cfg(test)]
mod tests_exactly29_bytes {

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Exactly29Bytes;

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
    fn from_string_roundtrip() {
        let str = "29deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        assert_eq!(SUT::from_hex(str).unwrap().to_string(), str);
    }

    #[test]
    fn debug() {
        let str = "29deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{:?}", hex_bytes), str);
    }

    #[test]
    fn display() {
        let str = "29deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(format!("{}", hex_bytes), str);
    }

    #[test]
    fn to_hex() {
        let str = "29deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead";
        let hex_bytes = SUT::sample();
        assert_eq!(hex_bytes.to_string(), str);
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("29deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!("not even hex"));
        assert_json_value_fails::<SUT>(json!("deadbeef"));
        assert_json_value_fails::<SUT>(json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"));
    }

    #[test]
    fn from_bytes_roundtrip() {
        let bytes = [0u8; 29];
        assert_eq!(SUT::from_bytes(&bytes).bytes(), bytes);
    }

    #[test]
    fn from_vec_roundtrip() {
        let vec = Vec::from([0u8; 29]);
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
                expected: 29,
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
}

#[cfg(test)]
mod hex29_uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn new_from_bag_of_bytes() {
        let bytes = generate_bytes::<29>();
        assert_eq!(
            new_exactly_29_bytes(bytes.clone().into()).unwrap().to_vec(),
            bytes
        );
    }

    #[test]
    fn new_fail() {
        assert!(new_exactly_29_bytes(generate_bytes::<5>().into()).is_err());
    }
}
