use crate::prelude::*;
use bytes::*;

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
        paste::paste! {
            $(
                decl_secret_bytes!(
                    [< Entropy $byte_count Bytes >],
                    $byte_count
                );
            )+

            $(
                #[doc = $expr]
            )*
            #[derive(Clone, PartialEq, derive_more::Debug, Zeroize)]
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
                #[allow(clippy::wrong_self_convention)] // cannot be `(self)` since we impl drop.
                fn into_bytes(&self) -> Vec<u8> {
                    match self {
                        $(
                            Self::[< EntropyOf $byte_count Bytes >](bytes) => Vec::from_iter(*bytes.0),
                        )+
                    }
                }

                #[allow(unused)]
                pub(crate) fn is_zeroized(&self) -> bool {
                    match self {
                        $(
                            Self::[< EntropyOf $byte_count Bytes >](bytes) => bytes.is_zeroized(),
                        )+
                    }
                }
            }

            impl From<$enum_name> for NonEmptyMax32Bytes {
                fn from(value: $enum_name) -> NonEmptyMax32Bytes {
                    NonEmptyMax32Bytes::try_from(value.into_bytes()).expect("Never more than 32 bytes, and never empty.")
                }
            }
        }
    }
}

entropy_with_byte_counts!(
    /// BIP39 entropy, ranging from 16-32 bytes with discrete values being multiples of in between the range.
    BIP39Entropy: 16, 20, 24, 28, 32,
);

impl Mnemonic {
    pub fn from_entropy_in(
        entropy: BIP39Entropy,
        language: BIP39Language,
    ) -> Self {
        let internal = bip39::Mnemonic::from_entropy_in(
            language.into(),
            NonEmptyMax32Bytes::from(entropy).as_ref(),
        )
        .unwrap();
        Self::from_internal(internal)
    }

    pub fn from_entropy(entropy: BIP39Entropy) -> Self {
        Self::from_entropy_in(entropy, BIP39Language::English)
    }

    pub fn generate_new() -> Self {
        Self::from_entropy(BIP39Entropy::from(Entropy32Bytes::new(
            generate_byte_array::<32>(),
        )))
    }

    pub fn generate_new_with_word_count(word_count: BIP39WordCount) -> Self {
        let entropy = match word_count {
            BIP39WordCount::TwentyFour => BIP39Entropy::from(
                Entropy32Bytes::new(generate_byte_array::<32>()),
            ),
            BIP39WordCount::TwentyOne => BIP39Entropy::from(
                Entropy28Bytes::new(generate_byte_array::<28>()),
            ),
            BIP39WordCount::Eighteen => BIP39Entropy::from(
                Entropy24Bytes::new(generate_byte_array::<24>()),
            ),
            BIP39WordCount::Fifteen => BIP39Entropy::from(Entropy20Bytes::new(
                generate_byte_array::<20>(),
            )),
            BIP39WordCount::Twelve => BIP39Entropy::from(Entropy16Bytes::new(
                generate_byte_array::<16>(),
            )),
        };
        Self::from_entropy(entropy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP39Entropy;

    #[test]
    fn zeroize() {
        let mut sut = SUT::from(Entropy16Bytes::new([0xff; 16]));
        assert!(!sut.is_zeroized());
        sut.zeroize();
        assert!(sut.is_zeroized());
    }

    #[test]
    fn mnemonic_from_entropy_of_16_bytes() {
        let sut = SUT::from(Entropy16Bytes::new([0xff; 16]));
        let mnemonic = Mnemonic::from_entropy(sut);
        assert_eq!(
            mnemonic.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
        )
    }

    #[test]
    fn mnemonic_from_entropy_of_20_bytes() {
        let sut = SUT::from(Entropy20Bytes::new([0xff; 20]));
        let mnemonic = Mnemonic::from_entropy(sut);
        assert_eq!(
            mnemonic.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrist"
        )
    }

    #[test]
    fn mnemonic_from_entropy_of_24_bytes() {
        let sut = SUT::from(Entropy24Bytes::new([0xff; 24]));
        let mnemonic = Mnemonic::from_entropy(sut);
        assert_eq!(
            mnemonic.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo when"
        )
    }

    #[test]
    fn mnemonic_from_entropy_of_28_bytes() {
        let sut = SUT::from(Entropy28Bytes::new([0xff; 28]));
        let mnemonic = Mnemonic::from_entropy(sut);
        assert_eq!(
            mnemonic.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo veteran"
        );
    }

    #[test]
    fn mnemonic_from_entropy_of_32_bytes() {
        let sut = SUT::from(Entropy32Bytes::new([0xff; 32]));
        let mnemonic = Mnemonic::from_entropy(sut);
        assert_eq!(
            mnemonic.phrase(),
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote"
        )
    }
}
