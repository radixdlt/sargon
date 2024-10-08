use crate::{decl_secret_bytes, prelude::*, UniffiCustomTypeConverter};

macro_rules! entropy_with_byte_counts {
    (
        $(
            #[doc = $expr: expr]
        )*
        $enum_name: ident:
        $(
            $byte_count: literal,
        )+
    ) => {
        paste! {
            $(
                decl_secret_bytes!(
                    [< Entropy $byte_count Bytes >],
                    $byte_count
                );
            )+

            $(
                #[doc = $expr]
            )*
            #[derive(Zeroize, uniffi::Enum)]
            pub enum $enum_name {
                $(
                    [< EntropyOf $byte_count Bytes >]([< Entropy $byte_count Bytes >]),
                )+
            }

            $(
                impl From< [< Entropy $byte_count Bytes >] > for $enum_name {
                    fn from(value: [< Entropy $byte_count Bytes >]) -> Self {
                        Self::[< EntropyOf $byte_count Bytes >](value)
                    }
                }

                impl From< [u8; $byte_count] > for $enum_name {
                    fn from(value: [u8; $byte_count]) -> Self {
                        Self::from([< Entropy $byte_count Bytes >]::new(value))
                    }
                }

                impl From<[< Entropy $byte_count Bytes >]> for NonEmptyMax32Bytes {
                    fn from(value: [< Entropy $byte_count Bytes >]) -> Self {
                        let bytes: &[u8] = value.secret_magic.0.as_ref();
                        assert!(bytes.len() <= 32);
                        NonEmptyMax32Bytes::try_from(bytes).unwrap()
                    }
                }

                impl TryFrom<NonEmptyMax32Bytes> for [< Entropy $byte_count Bytes >] {
                    type Error = CommonError;
                    fn try_from(value: NonEmptyMax32Bytes) -> Result<Self> {
                        let b: &[u8; $byte_count] = value.as_ref().try_into().map_err(|_| CommonError::Unknown)?;
                        Ok(Self::from([< Entropy $byte_count Bytes >]::new(*b)))
                    }
                }
            )+
        }
    }
}

entropy_with_byte_counts!(
    /// BIP39 entropy, ranging from 16-32 bytes with discrete values being multiples of in between the range.
    BIP39Entropy: 16, 20, 24, 28, 32,
);