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
        $expected_sample_str: literal
    ) => {

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

        paste! {
            #[uniffi::export]
            pub fn [< new_$struct_name:snake _from_string>](string: String) -> Result<$struct_name> {
                $struct_name::from_str(&string)
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
            }
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
                    bech32_encoded_tx_id,
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
        }
    };

    (
        $(
            #[doc = $expr: expr]
        )*
        $hash_type: ident,
        $expected_sample_str: literal
    ) => {
        paste! {
            decl_tx_hash!(
                $(
                    #[doc = $expr]
                )*
                [< $hash_type Hash >],
                [< Scrypto $hash_type Hash >],
                [< tests_ $hash_type:snake >],
                $expected_sample_str
            );
        }
    };
}

decl_tx_hash!(
    /// `IntentHash` used to identify transactions.
    /// Representation is bech32 encoded string starting with `txid_` e.g.:
    /// `"txid_rdx19rpveua6xuhvz0axu0mwpqk8fywr83atv8mkrugchvw6uuslgppqh9cnj4"`
    Intent,
    "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd"
);

decl_tx_hash!(
    /// A Signed Intent Hash is a bech32 encoded string starting with `"signedintent_"
    SignedIntent,
    "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl"
);
