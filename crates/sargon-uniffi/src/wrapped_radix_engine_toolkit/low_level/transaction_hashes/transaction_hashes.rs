use crate::prelude::*;

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_tx_hash {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
    ) => {
        paste! {
        use sargon::[< $struct_name Hash >] as [< Internal $struct_name Hash>];

        $(
            #[doc = $expr]
        )*
        #[derive(
            Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
        )]
        pub struct [< $struct_name Hash >] {
            /// Which network this transaction hash is used on
            pub network_id: NetworkID,
            /// the hash of the intent
            pub hash: Hash,
            /// Bech32 encoded TX id
            pub bech32_encoded_tx_id: String,
        }

        impl From<[< Internal $struct_name Hash>]> for [< $struct_name Hash >] {
            fn from(value: [< Internal $struct_name Hash>]) -> Self {
                Self {
                    network_id: value.network_id.into(),
                    hash: value.hash.into(),
                    bech32_encoded_tx_id: value.bech32_encoded_tx_id,
                }
            }
        }

        impl Into<[< Internal $struct_name Hash>]> for [< $struct_name Hash >] {
            fn into(self) -> [< Internal $struct_name Hash>] {
                [< Internal $struct_name Hash>] {
                    network_id: self.network_id.into(),
                    hash: self.hash.into(),
                    bech32_encoded_tx_id: self.bech32_encoded_tx_id,
                }
            }
        }

            #[uniffi::export]
            pub fn [< new_$struct_name:snake _from_string>](string: String) -> Result<[< $struct_name Hash >]> {
                [< Internal $struct_name Hash>]::from_str(&string).map_result()
            }

            #[uniffi::export]
            pub fn [< $struct_name:snake _formatted>](address: &[< $struct_name Hash >], format: AddressFormat) -> String {
                address.into_internal().formatted(format.into())
            }
        }
    }
}

pub(crate) use decl_tx_hash;
