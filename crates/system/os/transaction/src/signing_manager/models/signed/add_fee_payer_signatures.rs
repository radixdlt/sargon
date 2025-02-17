use crate::prelude::*;

trait AddFeePayerSignatures {
    fn add_fee_payer_signatures(
        &mut self,
        signatures: IndexSet<IntentSignature>,
    );
}

impl AddFeePayerSignatures for SignedIntent {
    fn add_fee_payer_signatures(
        &mut self,
        signatures: IndexSet<IntentSignature>,
    ) {
        let mut existing_signatures = self.intent_signatures.signatures.clone();
        existing_signatures.extend(signatures.into_iter().collect_vec());
        self.intent_signatures = IntentSignatures::new(existing_signatures);
    }
}
