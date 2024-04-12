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

            $(
                #[doc = $expr]
            )*
            #[derive(Zeroize, ZeroizeOnDrop, derive_more::Debug, derive_more::Display)]
            #[debug("OBFUSCATED")]
            #[display("OBFUSCATED")]
            #[repr(C)]
            pub struct $struct_name(Box<[u8; Self::LENGTH]>);


            impl $struct_name {
                pub const LENGTH: usize = $byte_count;

                pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
                    Self(Box::new(bytes))
                }

                #[allow(unused)]
                pub(crate) fn is_zeroized(&self) -> bool {
                    *self.0 == [0; Self::LENGTH]
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
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $byte_count: literal,
        true // uniffi_export
    ) => {
        decl_secret_bytes!(
            $(
                #[doc = $expr]
            )*
            $struct_name,
            $byte_count
        );

        paste! {

            impl TryFrom<&[u8]> for $struct_name {
                type Error = CommonError;
                fn try_from(value: &[u8]) -> Result<$struct_name> {
                    [< Exactly $byte_count Bytes >]::try_from(value).map(Self::from)
                }
            }

            impl From<[< Exactly $byte_count Bytes >]> for $struct_name {
                fn from(value: [< Exactly $byte_count Bytes >]) -> $struct_name {
                    $struct_name::new(*val.bytes())
                }
            }

            impl From<$struct_name> for [< Exactly $byte_count Bytes >] {
                fn from(value: $struct_name) -> [< Exactly $byte_count Bytes >] {
                    [< Exactly $byte_count Bytes >]::from(*value.0)
                }
            }

			uniffi::custom_type!($struct_name, [< Exactly $byte_count Bytes >]);

			impl UniffiCustomTypeConverter for $struct_name {
				type Builtin = [< Exactly $byte_count Bytes >];

				fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
					Ok($struct_name::from(val))
				}

				fn from_custom(obj: Self) -> Self::Builtin {
					[< Exactly $byte_count Bytes >]::from(obj)
				}
			}
        }
    };
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $byte_count: literal,
        false // SKIP uniffi_export
    ) => {
        decl_secret_bytes!(
            $(
                #[doc = $expr]
            )*
            $struct_name,
            $byte_count
        );
    };
}
