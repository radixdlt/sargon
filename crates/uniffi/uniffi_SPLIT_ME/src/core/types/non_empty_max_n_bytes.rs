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

            impl [< NonEmptyMax $byte_count Bytes  >] {
                pub fn into_internal(&self) -> [< InternalNonEmptyMax $byte_count Bytes  >] {
                    self.clone().into()
                }
            }

            impl From<[< InternalNonEmptyMax $byte_count Bytes  >]> for [< NonEmptyMax $byte_count Bytes  >] {
                fn from(value: [< InternalNonEmptyMax $byte_count Bytes  >]) -> Self {
                    Self {
                        bag_of_bytes: value.bag_of_bytes.into(),
                    }
                }
            }

            #[allow(clippy::from_over_into)]
            impl Into<[< InternalNonEmptyMax $byte_count Bytes  >]> for [< NonEmptyMax $byte_count Bytes  >] {
                #[allow(clippy::from_over_into)]
                fn into(self) -> [< InternalNonEmptyMax $byte_count Bytes  >] {
                    [< InternalNonEmptyMax $byte_count Bytes  >]::try_from(self.bag_of_bytes.into_internal()).unwrap().into()
                }
            }

            #[uniffi::export]
            pub fn [<new_non_empty_max_ $byte_count _bytes>](
                bag_of_bytes: BagOfBytes,
            ) -> Result<[< NonEmptyMax $byte_count Bytes  >]> {
                [< InternalNonEmptyMax $byte_count Bytes  >]::try_from(bag_of_bytes.into_internal()).into_result()
            }

            decl_conversion_tests_for!([< NonEmptyMax $byte_count Bytes  >]);
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
