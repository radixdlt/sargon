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
        use sargon::$struct_name as [< Internal $struct_name>];

        $(
            #[doc = $expr]
        )*
        #[derive(
            Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record,
        )]
        pub struct $struct_name {
            /// Which network this transaction hash is used on
            pub network_id: NetworkID,
            /// the hash of the intent
            pub hash: Hash,
            /// Bech32 encoded TX id
            pub bech32_encoded_tx_id: String,
        }

            #[uniffi::export]
            pub fn [< new_$struct_name:snake _from_string>](string: String) -> Result<$struct_name> {
                [< Internal $struct_name>]::from_str(&string).map_result()
            }

            #[uniffi::export]
            pub fn [< $struct_name:snake _formatted>](address: &$struct_name, format: AddressFormat) -> String {
                address.into_internal().formatted(format.into())
            }
        }
    }
}

pub(crate) use decl_tx_hash;
