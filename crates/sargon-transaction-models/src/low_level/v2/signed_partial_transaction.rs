use crate::prelude::*;

pub fn build_signed_partial_transaction(
    subintent: Subintent,
    signatures: Vec<IntentSignature>,
) -> ScryptoSignedPartialTransaction {
    ScryptoSignedPartialTransaction {
        partial_transaction: ScryptoPartialTransaction {
            root_subintent: ScryptoSubintent::from(subintent),
            non_root_subintents: ScryptoNonRootSubintents(vec![]),
        },
        root_subintent_signatures: ScryptoIntentSignaturesV2 {
            signatures: signatures.into_iter().map(|s| s.into()).collect(),
        },
        non_root_subintent_signatures: ScryptoNonRootSubintentSignatures {
            by_subintent: vec![],
        },
    }
}
