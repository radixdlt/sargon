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
            pub struct $struct_name(Box<[u8; Self::LENGTH]>);

            uniffi::custom_type!($struct_name, BagOfBytes);


            impl $crate::UniffiCustomTypeConverter for $struct_name {
                type Builtin = BagOfBytes;

                fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                    let fixed_size: &[u8; Self::LENGTH] =  val.as_ref().try_into()?;//.map_err(|e| e.into())?;
                   Ok(Self::new(*fixed_size))
                }

                fn from_custom(obj: Self) -> Self::Builtin {
                    BagOfBytes::from(obj.0.as_slice())
                }
            }

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
}
