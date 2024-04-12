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
            #[derive(Zeroize)]
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
                        let bytes: &[u8] = value.0.as_ref();
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

            uniffi::custom_type!(BIP39Entropy, NonEmptyMax32Bytes);

            impl TryFrom<NonEmptyMax32Bytes> for $enum_name {
                type Error = CommonError;
                fn try_from(value: NonEmptyMax32Bytes) -> Result<$enum_name> {
                    Err(CommonError::Unknown)
                        $(
                            .or([< Entropy $byte_count Bytes >]::try_from(value.clone()).map(Self::from))
                        )+
                }
            }

            impl $enum_name {
                fn into_bytes(self) -> Vec<u8> {
                    match self {
                        $(
                            Self::[< EntropyOf $byte_count Bytes >](bytes) => Vec::from_iter(*bytes.0),
                        )+
                    }
                }
            }

            impl From<$enum_name> for NonEmptyMax32Bytes {
                fn from(value: $enum_name) -> NonEmptyMax32Bytes {
                    NonEmptyMax32Bytes::try_from(value.into_bytes()).expect("Never more than 32 bytes, and never empty.")
                }
            }

            impl UniffiCustomTypeConverter for $enum_name {
                type Builtin = NonEmptyMax32Bytes;

                fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                    Self::try_from(val).map_err(|e| e.into())
                }

                fn from_custom(obj: Self) -> Self::Builtin {
                    NonEmptyMax32Bytes::from(obj)
                }
            }
        }
    }
}

entropy_with_byte_counts!(
    /// A BIP39 entropy
    BIP39Entropy: 16, 20, 24, 28, 32,
);

impl Mnemonic {
    pub fn from_entropy(entropy: BIP39Entropy) -> Self {
        let internal = bip39::Mnemonic::from_entropy(
            NonEmptyMax32Bytes::from(entropy).as_ref(),
        )
        .unwrap();
        Self::from_internal(internal)
    }

    pub fn generate_new() -> Self {
        Self::from_entropy(BIP39Entropy::from(Entropy32Bytes::new(
            generate_byte_array::<32>(),
        )))
    }
}
