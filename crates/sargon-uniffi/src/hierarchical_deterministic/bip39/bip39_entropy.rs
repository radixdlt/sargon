use crate::{decl_secret_bytes, prelude::*};

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

            use sargon::$enum_name as [< Internal $enum_name >];

            $(
                #[doc = $expr]
            )*
            #[derive(Clone, Eq, PartialEq, InternalConversion, uniffi::Enum)]
            pub enum $enum_name {
                $(
                    [< EntropyOf $byte_count Bytes >]([< Entropy $byte_count Bytes >]),
                )+
            }
        }
    }
}

entropy_with_byte_counts!(
    /// BIP39 entropy, ranging from 16-32 bytes with discrete values being multiples of in between the range.
    BIP39Entropy: 16, 20, 24, 28, 32,
);
