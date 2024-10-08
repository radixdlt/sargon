use crate::prelude::*;

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
            #[derive(Zeroize, derive_more::Debug, derive_more::Display)]
            #[debug("OBFUSCATED")]
            #[display("OBFUSCATED")]
            pub struct $struct_name(Box<[u8; $byte_count]>);

           impl $struct_name {
                pub fn to_bytes(&self) -> &[u8] {
                    &self.0.as_slice()
                }
           }

           impl TryFrom<BagOfBytes> for $struct_name {
            type Error = CommonError;
            fn try_from(value: BagOfBytes) -> Result<Self> {
                let fixed_size: &[u8; $byte_count] = value.as_ref().try_into().map_err(|_| CommonError::InvalidByteCount { expected: $byte_count as u64, found: value.len() as u64 })?;
                Ok(Self(Box::new(*fixed_size)))
            }
        }

            impl HasSampleValues for $struct_name {
                fn sample() -> Self {
                    Self (Box::new([0xab; $byte_count]))
                }

                fn sample_other() -> Self {
                    Self (Box::new([0xde; $byte_count]))
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

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_name;

                #[test]
                fn zeroize() {
                    let mut sut = SUT::sample();
                    assert!(!sut.is_zeroized());
                    sut.zeroize();
                    assert!(sut.is_zeroized());
                }

                #[test]
                fn debug_obfuscates_secret() {
                    let sut = SUT::sample_other();
                    assert_eq!(format!("{:?}", sut), "OBFUSCATED");
                }

                #[test]
                fn display_obfuscates_secret() {
                    let sut = SUT::sample_other();
                    assert_eq!(format!("{}", sut), "OBFUSCATED");
                }
            }

        }
    };
}

pub(crate) use decl_secret_bytes;
