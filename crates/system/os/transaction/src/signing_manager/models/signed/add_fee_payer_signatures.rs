use crate::prelude::*;

pub(crate) trait AddFeePayerSignatures {
    fn add_fee_payer_signatures(
        &mut self,
        signatures: IndexSet<OwnedIntentSignature>,
    );
}

// impl AddFeePayerSignatures for SignedIntent {
//     fn add_fee_payer_signatures(
//         &mut self,
//         signatures: IndexSet<IntentSignature>,
//     ) {
//         let mut existing_signatures = self.intent_signatures.signatures.clone();
//         existing_signatures.extend(signatures.into_iter().collect_vec());
//         self.intent_signatures = IntentSignatures::new(existing_signatures);
//     }
// }

impl AddFeePayerSignatures for SignedIntentWithOwners {
    fn add_fee_payer_signatures(
        &mut self,
        signatures: IndexSet<OwnedIntentSignature>,
    ) {
        self.intent_signatures
            .extend(signatures.into_iter().collect_vec());
    }
}
