use crate::prelude::*;

pub fn build_signed_partial_transaction(
    intent_core: IntentCoreV2,
    signatures: IntentSignaturesV2,
) -> ScryptoSignedPartialTransaction {
    ScryptoSignedPartialTransaction {
        partial_transaction: ScryptoPartialTransaction {
            root_subintent: ScryptoSubintent {
                intent_core: intent_core.into(),
            },
            non_root_subintents: ScryptoNonRootSubintents(vec![]),
        },
        root_subintent_signatures: signatures.into(),
        non_root_subintent_signatures: ScryptoNonRootSubintentSignatures {
            by_subintent: vec![],
        },
    }
}
