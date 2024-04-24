use crate::prelude::*;

#[macro_export]
macro_rules! decl_secret_bytes {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $byte_count: literal
    ) => {
        paste! {

            #[derive(Zeroize, ZeroizeOnDrop, derive_more::Debug, derive_more::Display)]
            #[debug("OBFUSCATED")]
            #[display("OBFUSCATED")]
            pub struct [< $struct_name SecretMagic >](Box<[u8; $byte_count]>);

            uniffi::custom_type!([< $struct_name SecretMagic >], BagOfBytes);

            impl TryFrom<BagOfBytes> for [< $struct_name SecretMagic >] {
                type Error = CommonError;
                fn try_from(value: BagOfBytes) -> Result<Self> {
                    let fixed_size: &[u8; $byte_count] = value.as_ref().try_into().map_err(|_| CommonError::InvalidByteCount { expected: $byte_count as u64, found: value.len() as u64 })?;
                    Ok(Self(Box::new(*fixed_size)))
                }
            }

            impl $crate::UniffiCustomTypeConverter for [< $struct_name SecretMagic >] {
                type Builtin = BagOfBytes;

                fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                    Self::try_from(val).map_err(|e| e.into())
                }

                fn from_custom(obj: Self) -> Self::Builtin {
                    BagOfBytes::from(obj.0.as_slice())
                }
            }

            $(
                #[doc = $expr]
            )*
            #[derive(Zeroize, derive_more::Debug, derive_more::Display, uniffi::Record)]
            #[debug("OBFUSCATED")]
            #[display("OBFUSCATED")]
            pub struct $struct_name {
                secret_magic: [< $struct_name SecretMagic >]
            }

            #[uniffi::export]
            pub fn [< new_ $struct_name:snake _from_bytes >](bytes: BagOfBytes) -> Result<$struct_name> {
                [< $struct_name SecretMagic >]::try_from(bytes)
                    .map(|secret_magic| $struct_name { secret_magic })
            }

            impl $struct_name {
                pub const LENGTH: usize = $byte_count;

                pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
                    Self {
                        secret_magic: [< $struct_name SecretMagic >](Box::new(bytes))
                    }
                }

                #[allow(unused)]
                pub(crate) fn is_zeroized(&self) -> bool {
                    *self.secret_magic.0 == [0; Self::LENGTH]
                }
            }

            #[cfg(test)]
            mod [< uniffi_ $struct_name:snake tests >] {
                use super::*;

                #[test]
                fn test_from_bytes() {
                    let too_few_bytes = BagOfBytes::from_str("dead").unwrap();
                    assert!([< new_ $struct_name:snake _from_bytes >](too_few_bytes).is_err());
                }

            }

            #[cfg(test)]
            mod [< $struct_name:snake tests >] {
                use super::*;
                use std::mem;
                use std::ops::Range;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_name;

                #[test]
                fn zeroize() {
                    let mut sut = SUT::new([0xff; SUT::LENGTH]);
                    assert!(!sut.is_zeroized());
                    sut.zeroize();
                    assert!(sut.is_zeroized());
                }

                #[test]
                fn debug_obfuscates_secret() {
                    let sut = SUT::new([0xff; SUT::LENGTH]);
                    assert_eq!(format!("{:?}", sut), "OBFUSCATED");
                }

                #[test]
                fn display_obfuscates_secret() {
                    let sut = SUT::new([0xff; SUT::LENGTH]);
                    assert_eq!(format!("{}", sut), "OBFUSCATED");
                }
            }

        }
    };
}
