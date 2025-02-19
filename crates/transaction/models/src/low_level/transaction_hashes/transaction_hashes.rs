use crate::prelude::*;

use crate::low_level::transaction_hashes::validate_and_decode_hash::validate_and_decode_hash;
use core_utils::prelude::format_string;
use radix_common::crypto::IsHash;

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

        $(
            #[doc = $expr]
        )*
        #[derive(
            Clone, Copy, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr, derive_more::Display, derive_more::Debug,
        )]
        #[display("{}", self.bech32_encoded_tx_id)]
        #[debug("{}", self.bech32_encoded_tx_id)]
        pub struct $struct_name {
            /// Which network this transaction hash is used on
            pub network_id: NetworkID,
            /// the hash of the intent
            pub hash: Hash,
            /// Bech32 encoded TX id
            pub bech32_encoded_tx_id: short_string::prelude::ShortString,
        }


        impl $struct_name {
            pub(crate) fn from_scrypto(
                scrypto: $scrypto_struct_name,
                network_id: NetworkID,
            ) -> Self {
                let bech32_encoder = ScryptoTransactionHashBech32Encoder::new(
                    &network_id.network_definition(),
                );
                let bech32_encoded_tx_id = bech32_encoder
                    .encode(&scrypto)
                    .expect("should never fail");
                let scrypto_hash: ScryptoHash = *scrypto.as_hash();
                Self {
                    network_id,
                    hash: scrypto_hash.into(),
                    bech32_encoded_tx_id: short_string::prelude::ShortString::new(bech32_encoded_tx_id).expect("Bech32 encoded tx id should not be longer than 255 chars"),
                }
            }
            pub fn new(hash: Hash, network_id: NetworkID) -> Self {
                let scrypto_hash: ScryptoHash = hash.clone().into_hash();
                Self::from_scrypto(
                    $scrypto_struct_name::from_hash(scrypto_hash),
                    network_id,
                )
            }
            pub fn from_bech32(s: &str) -> Result<Self> {
                validate_and_decode_hash::<$scrypto_struct_name>(s)
                    .map(|t| Self::from_scrypto(t.0, t.1))
            }

            pub fn formatted(&self, format: AddressFormat) -> String {
                match format {
                    AddressFormat::Default => format_string(self.bech32_encoded_tx_id.to_string(), 4, 6),
                    AddressFormat::Full | AddressFormat::Raw => self.bech32_encoded_tx_id.to_string(),
                }
            }
        }

        impl FromStr for $struct_name {
            type Err = crate::CommonError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_bech32(s)
            }
        }

        impl From<$struct_name> for Hash {
            fn from(value: $struct_name) -> Hash {
                value.hash.clone()
            }
        }

        #[cfg(test)]
        mod $mod_test_name {
            use super::*;

            #[allow(clippy::upper_case_acronyms)]
            type SUT = $struct_name;

            #[test]
            fn equality() {
                assert_eq!(SUT::sample(), SUT::sample());
                assert_eq!(SUT::sample_other(), SUT::sample_other());
            }

            #[test]
            fn inequality() {
                assert_ne!(SUT::sample(), SUT::sample_other());
            }

            #[test]
            fn to_string() {
                assert_eq!(SUT::sample().to_string(), $expected_sample_str);
            }

            #[test]
            fn from_str() {
                assert_eq!(SUT::sample(), $expected_sample_str.parse::<SUT>().unwrap());
            }

            #[test]
            fn formatted() {
                let sut = $expected_sample_str.parse::<SUT>().unwrap();
                assert_eq!($expected_sample_str_formatted, sut.formatted(AddressFormat::Default));
                assert_eq!($expected_sample_str, sut.formatted(AddressFormat::Raw));
                assert_eq!($expected_sample_str, sut.formatted(AddressFormat::Full));
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
        paste::paste! {
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

decl_tx_hash!(
    /// `TransactionIntentHash` used to identify transactions.
    /// Representation is bech32 encoded string starting with `txid_` e.g.:
    /// `"txid_rdx19rpveua6xuhvz0axu0mwpqk8fywr83atv8mkrugchvw6uuslgppqh9cnj4"`
    TransactionIntent,
    "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd",
    "txid...zm3ltd",
);

decl_tx_hash!(
    /// A Signed Transaction Intent Hash is a bech32 encoded string starting with `"signedintent_"
    SignedTransactionIntent,
    "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl",
    "sign...xsk6nl",
);

decl_tx_hash!(
    /// A hash of the subintent.
    /// A Subintent Hash is a bech32 encoded string starting with `"subtxid_"
    Subintent,
    "subtxid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sy6hgte",
    "subt...y6hgte",
);
