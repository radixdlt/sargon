use crate::prelude::*;

use crate::wrapped_radix_engine_toolkit::low_level::transaction_hashes::validate_and_decode_hash::validate_and_decode_hash;

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_tx_hash {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident,
        $scrypto_struct_name: ident,
        $mod_test_name: ident,
        $expected_sample_str: literal,
        $expected_sample_str_formatted: literal
    ) => {

        use sargon::$struct_name as Internal$struct_name;

        $(
            #[doc = $expr]
        )*
        #[derive(
            Clone, PartialEq, Eq, Hash, derive_more::Display, derive_more::Debug, uniffi::Record,
        )]
        #[display("{}", self.bech32_encoded_tx_id)]
        #[debug("{}", self.bech32_encoded_tx_id)]
        pub struct $struct_name {
            /// Which network this transaction hash is used on
            pub network_id: NetworkID,
            /// the hash of the intent
            pub hash: Hash,
            /// Bech32 encoded TX id
            pub bech32_encoded_tx_id: String,
        }

        impl From<Internal$struct_name> for $struct_name {
            fn from(value: Internal$struct_name) -> Self {
                Self {
                    network_id: value.network_id.into(),
                    hash: value.hash.into(),
                    bech32_encoded_tx_id: value.bech32_encoded_tx_id,
                }
            }
        }

        impl Into<Internal$struct_name> for $struct_name {
            fn into(self) -> Internal$struct_name {
                Internal$struct_name {
                    network_id: self.network_id.into(),
                    hash: self.hash.into(),
                    bech32_encoded_tx_id: self.bech32_encoded_tx_id,
                }
            }
        }

        paste! {
            #[uniffi::export]
            pub fn [< new_$struct_name:snake _from_string>](string: String) -> Result<$struct_name> {
                map_result_from_internal(Internal$struct_name::from_str(&string))
            }

            #[uniffi::export]
            pub fn [< $struct_name:snake _formatted>](address: &$struct_name, format: AddressFormat) -> String {
                address.into::<Internal$struct_name>.formatted(format)
            }

            #[cfg(test)]
            mod [< uniffi_ $struct_name:snake _tests>] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_name;

                #[test]
                fn from_str() {
                    assert_eq!(SUT::sample(), [< new_$struct_name:snake _from_string>]($expected_sample_str.to_owned()).unwrap());
                }

                #[test]
                fn formatted() {
                    let sut = SUT::sample();
                    assert_eq!(sut.formatted(AddressFormat::Default), [< $struct_name:snake _formatted>](&sut, AddressFormat::Default));
                    assert_eq!(sut.formatted(AddressFormat::Raw), [< $struct_name:snake _formatted>](&sut, AddressFormat::Raw));
                    assert_eq!(sut.formatted(AddressFormat::Full), [< $struct_name:snake _formatted>](&sut, AddressFormat::Full));
                }
            }
        }
    };

    (
        $(
            #[doc = $expr: expr]
        )*
        $hash_type: ident,
        $expected_sample_str: literal,
        $expected_sample_str_formatted: literal,
    ) => {
        paste! {
            decl_tx_hash!(
                $(
                    #[doc = $expr]
                )*
                [< $hash_type Hash >],
                [< Scrypto $hash_type Hash >],
                [< tests_ $hash_type:snake >],
                $expected_sample_str,
                $expected_sample_str_formatted
            );
        }
    };
}
