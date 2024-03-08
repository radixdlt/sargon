use crate::prelude::*;
use paste::*;
use transaction::model::{
    HashHasHrp as ScryptoHashHasHrp, IntentHash as ScryptoIntentHash,
    NotarizedTransactionV1 as ScryptoNotarizedTransaction,
    SignedIntentHash as ScryptoSignedIntentHash,
    TransactionHashBech32Decoder as ScryptoTransactionHashBech32Decoder,
    TransactionHashBech32Encoder as ScryptoTransactionHashBech32Encoder,
};

use crate::wrapped_radix_engine_toolkit::low_level::transaction_hashes::validate_and_decode_hash::validate_and_decode_hash;

use radix_engine_common::crypto::{
    Hash as ScryptoHash, IsHash as ScryptoIsHash,
};

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_tx_hash {
    (
        $(
            #[doc = $expr: expr]
        )*
        $hash_type:ident
    ) => {
        paste! {
            $(
                #[doc = $expr]
            )*

            #[derive(
                Debug, Clone, PartialEq, Eq, Hash, derive_more::Display, uniffi::Record,
            )]
            #[display("{}", self.bech32_encoded_tx_id)]
            pub struct [< $hash_type:camel Hash >] {
                /// Which network this transaction hash is used on
                pub network_id: NetworkID,
                /// the hash of the intent
                pub hash: Hash,
                /// Bech32 encoded TX id
                pub bech32_encoded_tx_id: String,
            }

            impl [< $hash_type:camel Hash >] {
                pub(crate) fn from_scrypto(
                    [< $hash_type:snake _hash >]: [<Scrypto $hash_type:camel Hash >],
                    network_id: NetworkID,
                ) -> Self {
                    let scrypto = [< $hash_type:snake _hash >];
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
                        [<Scrypto $hash_type:camel Hash >]::from_hash(scrypto_hash),
                        network_id,
                    )
                }

                pub fn from_bech32(s: &str) -> Result<Self> {
                    validate_and_decode_hash::<[<Scrypto $hash_type:camel Hash >]>(s)
                        .map(|t| Self::from_scrypto(t.0, t.1))
                }
            }

            impl FromStr for [< $hash_type:camel Hash >] {
                type Err = crate::CommonError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::from_bech32(s)
                }
            }

            impl From<[< $hash_type:camel Hash >]> for Hash {
                fn from(value: [< $hash_type:camel Hash >]) -> Hash {
                    value.hash.clone()
                }
            }
        }
    };
}

decl_tx_hash!(
    /// `IntentHash` used to identify transactions.
    /// Representation is bech32 encoded string starting with `txid_` e.g.:
    /// `"txid_rdx19rpveua6xuhvz0axu0mwpqk8fywr83atv8mkrugchvw6uuslgppqh9cnj4"`
    Intent
);

decl_tx_hash!(
    /// A Signed Intent Hash is a bech32 encoded string starting with `"signedintent_"
    SignedIntent
);
