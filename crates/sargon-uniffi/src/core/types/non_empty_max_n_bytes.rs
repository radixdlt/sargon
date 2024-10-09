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
                        bag_of_bytes: value.bag_of_bytes.into(),
                    }
                }
            }

            impl Into<[< InternalNonEmptyMax $byte_count Bytes  >]> for [< NonEmptyMax $byte_count Bytes  >] {
                fn into(self) -> [< InternalNonEmptyMax $byte_count Bytes  >] {
                    [< InternalNonEmptyMax $byte_count Bytes  >]::try_from(self.bag_of_bytes.into()).unwrap()
                }
            }

            #[uniffi::export]
            pub fn [<new_non_empty_max_ $byte_count _bytes>](
                bag_of_bytes: BagOfBytes,
            ) -> Result<[< NonEmptyMax $byte_count Bytes  >]> {
                [< InternalNonEmptyMax $byte_count Bytes  >]::try_from(bag_of_bytes.into()).map_result()
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
