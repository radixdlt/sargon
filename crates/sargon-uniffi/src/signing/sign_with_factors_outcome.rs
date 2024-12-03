use crate::prelude::*;
use paste::paste;

// macro_rules! decl_sign_with_factors_outcome {
//     (signable_id: $signable_id:ty) => {
//         paste! {
//             use sargon::[< $signable_id >] as [< Internal $signable_id >];
//
//             type [< InternalSignWithFactorsOutcomeOf $signable_id >] =
//                 sargon::SignWithFactorsOutcome<[< Internal $signable_id >]>;
//
//             /// A batch of keys (derivation paths) all being factor instances of a HDFactorSource
//             /// with id `factor_source_id` to sign a single transaction with, which hash
//             /// is `intent_hash`.
//             #[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
//             pub struct [< TransactionSignRequestInputOf $signable >] {
//                 /// The user successfully signed with the factor source(s), the associated
//                 /// value contains the produces signatures and any relevant metadata.
//                 #[debug("Signed: {:#?}", produced_signatures)]
//                 Signed {
//                     produced_signatures: SignResponse<ID>,
//                 },
//
//                 /// The factor source got neglected, either due to user explicitly skipping
//                 /// or due to failure
//                 #[debug("Neglected")]
//                 Neglected(NeglectedFactors),
//             }
//         }
//     };
// }
//
// decl_sign_with_factors_outcome!(signable_id: TransactionIntentHash);
// decl_sign_with_factors_outcome!(signable_id: SubintentHash);