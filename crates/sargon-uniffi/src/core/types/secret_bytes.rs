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
            #[derive(uniffi::Record)]
            pub struct $struct_name {
                secret_magic: BagOfBytes
            }

            impl From<[< Internal $struct_name >]> for $struct_name {
                fn from(value: [< Internal $struct_name >]) -> Self {
                    Self {
                        secret_magic: value.into()
                    }
                }
            }

            impl Into<[< Internal $struct_name >]> for $struct_name {
                fn into(self) -> [< Internal $struct_name >] {
                    [< Internal $struct_name >]::try_from(self.secret_magic.into::<InternalBagOfBytes>()).unwrap()
                }
            }

            #[uniffi::export]
            pub fn [< new_ $struct_name:snake _from_bytes >](bytes: BagOfBytes) -> Result<$struct_name> {
                [< Internal $struct_name SecretMagic >]::try_from(bytes).map_result()
            }
            
            #[uniffi::export]
            pub fn [< new_ $struct_name:snake _sample >]() -> $struct_name {
                [< Internal $struct_name SecretMagic >]::sample().into()
            }

            #[uniffi::export]
            pub fn [< new_ $struct_name:snake _sample_other >]() -> $struct_name {
                [< Internal $struct_name SecretMagic >]::sample_other().into()
            }

            #[uniffi::export]
            pub fn [< $struct_name:snake _to_bytes >](bytes: &$struct_name) -> BagOfBytes {
                bytes.to_bytes()
            }
        }
    };
}

pub(crate) use decl_secret_bytes;
