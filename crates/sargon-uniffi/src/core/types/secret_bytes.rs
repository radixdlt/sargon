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
            use sargon::$struct_name as [< Internal $struct_name >];

            $(
                #[doc = $expr]
            )*
            #[derive(Clone, Eq, PartialEq, uniffi::Record)]
            pub struct $struct_name {
                value: BagOfBytes
            }

            impl From<[< Internal $struct_name >]> for $struct_name {
                fn from(value: [< Internal $struct_name >]) -> Self {
                    Self {
                        value: value.to_vec().into()
                    }
                }
            }

            impl Into<[< Internal $struct_name >]> for $struct_name {
                fn into(self) -> [< Internal $struct_name >] {
                    [< Internal $struct_name >]::try_from(self.value.into_internal()).unwrap()
                }
            }

            #[uniffi::export]
            pub fn [< new_ $struct_name:snake _from_bytes >](bytes: BagOfBytes) -> Result<$struct_name> {
                [< Internal $struct_name >]::try_from(bytes.into_internal()).map_result()
            }

            #[uniffi::export]
            pub fn [< new_ $struct_name:snake _sample >]() -> $struct_name {
                [< Internal $struct_name >]::sample().into()
            }

            #[uniffi::export]
            pub fn [< new_ $struct_name:snake _sample_other >]() -> $struct_name {
                [< Internal $struct_name >]::sample_other().into()
            }

            #[uniffi::export]
            pub fn [< $struct_name:snake _to_bytes >](bytes: &$struct_name) -> BagOfBytes {
                bytes.value.clone()
            }
        }
    };
}

pub(crate) use decl_secret_bytes;
