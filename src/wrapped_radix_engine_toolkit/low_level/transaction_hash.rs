// use crate::prelude::*;

// use radix_engine_common::crypto::IsHash as ScryptoIsHash;
// use radix_engine_toolkit::functions::utils::decode_transaction_id as RET_decode_transaction_id;
// use radix_engine_toolkit::models::transaction_hash::TransactionHash as RETTransactionHash;
// use transaction::model::TransactionHashBech32Encoder as ScryptoTransactionHashBech32Encoder;
// use transaction::prelude::HashHasHrp as ScryptoHashHasHrp;

// #[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
// pub struct TransactionHashSecretMagic {
//     pub(crate) hash: Hash,
//     pub(crate) bech32: String,
//     pub(crate) network_id: NetworkID,
// }

// #[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
// pub struct TransactionHash {
//     pub(crate) secret_magic: TransactionHashSecretMagic,
// }

// impl TransactionHash {
//     pub fn from_str(bech32: String, network_id: NetworkID) -> Result<Self> {
//         let network_definition = network_id.network_definition();
//         let hash = RET_decode_transaction_id(&bech32, &network_definition)
//             .map_err(|_| CommonError::FailedToDecodeTransactionHash {
//                 bad_value: bech32.clone(),
//             })?;
//         let secret_magic = TransactionHashSecretMagic {
//             hash: hash.into(),
//             bech32,
//             network_id,
//         };
//         Ok(Self { secret_magic })
//     }

//     pub fn as_hash(&self) -> Hash {
//         self.secret_magic.hash.clone()
//     }

//     pub fn as_str(&self) -> String {
//         self.secret_magic.bech32.clone()
//     }

//     pub fn bytes(&self) -> Vec<u8> {
//         self.secret_magic.hash.bytes()
//     }

//     pub fn network_id(&self) -> NetworkID {
//         self.secret_magic.network_id
//     }
// }

// impl TransactionHash {
//     pub fn new<T>(hash: &T, network_id: NetworkID) -> Self
//     where
//         T: ScryptoHashHasHrp + ScryptoIsHash,
//     {
//         let network_definition = network_id.network_definition();
//         let bech32_encoder =
//             ScryptoTransactionHashBech32Encoder::new(&network_definition);
//         let bech32 = bech32_encoder
//             .encode(hash)
//             .expect("Bech32m encoding tx hashes can't fail");
//         let hash = *hash.as_hash();
//         Self {
//             secret_magic: TransactionHashSecretMagic {
//                 hash: hash.into(),
//                 bech32,
//                 network_id,
//             },
//         }
//     }
// }

// /*

// pub trait HashHasHrp
// where
//     Self: IsHash,
// {
//     fn hrp<'h>(hrp_set: &'h HrpSet) -> &'h str;
// }

// impl HashHasHrp for IntentHash {
//     fn hrp<'h>(hrp_set: &'h HrpSet) -> &'h str {
//         &hrp_set.transaction_intent
//     }
// }

// impl HashHasHrp for SignedIntentHash {
//     fn hrp<'h>(hrp_set: &'h HrpSet) -> &'h str {
//         &hrp_set.signed_transaction_intent
//     }
// }

// impl HashHasHrp for NotarizedTransactionHash {
//     fn hrp<'h>(hrp_set: &'h HrpSet) -> &'h str {
//         &hrp_set.notarized_transaction
//     }
// }

// impl HashHasHrp for SystemTransactionHash {
//     fn hrp<'h>(hrp_set: &'h HrpSet) -> &'h str {
//         &hrp_set.system_transaction
//     }
// }

// */
