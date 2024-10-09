use crate::prelude::*;
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

        use sargon::$struct_name as InternalTxHash;

        $(
            #[doc = $expr]
        )*
        #[derive(
            Clone, PartialEq, Eq, Hash,  uniffi::Record,
        )]
        pub struct $struct_name {
            /// Which network this transaction hash is used on
            pub network_id: NetworkID,
            /// the hash of the intent
            pub hash: Hash,
            /// Bech32 encoded TX id
            pub bech32_encoded_tx_id: String,
        }

        impl From<InternalTxHash> for $struct_name {
            fn from(value: InternalTxHash) -> Self {
                Self {
                    network_id: value.network_id.into(),
                    hash: value.hash.into(),
                    bech32_encoded_tx_id: value.bech32_encoded_tx_id,
                }
            }
        }

        impl Into<InternalTxHash> for $struct_name {
            fn into(self) -> InternalTxHash {
                InternalTxHash {
                    network_id: self.network_id.into(),
                    hash: self.hash.into(),
                    bech32_encoded_tx_id: self.bech32_encoded_tx_id,
                }
            }
        }

        paste! {
            #[uniffi::export]
            pub fn [< new_$struct_name:snake _from_string>](string: String) -> Result<$struct_name> {
                InternalTxHash::from_str(&string).map_result()
            }

            #[uniffi::export]
            pub fn [< $struct_name:snake _formatted>](address: &$struct_name, format: AddressFormat) -> String {
                address.into_internal().formatted(format)
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

pub(crate) use decl_tx_hash;