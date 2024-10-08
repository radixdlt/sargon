use crate::prelude::*;
use paste::*;

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
            use sargon::[< NonEmptyMax $byte_count Bytes  >] as [< InternalNonEmptyMax $byte_count Bytes  >];

            $(
                #[doc = $expr]
            )*
            #[derive(
                Zeroize, // Not `ZeroizeOnDrop`: we dont wanna zeroize all byte types: use `decl_secret_bytes!` for secrets.
                Clone,
                PartialEq,
                Eq,
                Hash,
                uniffi::Record,
            )]
            pub struct [< NonEmptyMax $byte_count Bytes  >] {
                bag_of_bytes: BagOfBytes,
            }

            impl From<[< InternalNonEmptyMax $byte_count Bytes  >]> for [< NonEmptyMax $byte_count Bytes  >] {
                fn from(value: [< InternalNonEmptyMax $byte_count Bytes  >]) -> Self {
                    Self {
                        bag_of_bytes: value.into(),
                    }
                }
            }

            impl Into<[< InternalNonEmptyMax $byte_count Bytes  >]> for [< NonEmptyMax $byte_count Bytes  >] {
                fn into(self) -> [< InternalNonEmptyMax $byte_count Bytes  >] {
                    [< InternalNonEmptyMax $byte_count Bytes  >]::try_from(self.bag_of_bytes).unwrap()
                }
            }

            #[uniffi::export]
            pub fn [<new_non_empty_max_ $byte_count _bytes>](
                bag_of_bytes: BagOfBytes,
            ) -> Result<[< NonEmptyMax $byte_count Bytes  >]> {
                [< NonEmptyMax $byte_count Bytes  >]::try_from(bag_of_bytes)
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

    // #[test]
    // fn json_roundtrip() {
    //     let model = SUT::sample();
    //     assert_json_value_eq_after_roundtrip(
    //         &model,
    //         json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"),
    //     );
    // }

    // #[test]
    // fn json_roundtrip_fails_for_invalid() {
    //     assert_json_value_fails::<SUT>(json!("not even hex"));
    //     assert_json_value_fails::<SUT>(json!("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead"));
    // }

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

#[cfg(test)]
mod non_empty_max_64_uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn new_from_bag_of_bytes() {
        let bytes = generate_bytes::<13>();
        assert_eq!(
            new_non_empty_max_64_bytes(bytes.clone().into())
                .unwrap()
                .to_vec(),
            bytes
        );
    }

    #[test]
    fn new_fail() {
        assert!(
            new_non_empty_max_64_bytes(generate_bytes::<65>().into()).is_err()
        );
    }
}
