use crate::prelude::*;

impl Signable for TransactionIntent {
    type ID = TransactionIntentHash;

    type Payload = CompiledTransactionIntent;

    fn get_payload(&self) -> Self::Payload {
        self.compile()
    }

    fn entities_requiring_signing(&self, profile: &Profile) -> Result<IndexSet<AccountOrPersona>> {
        let summary = self.manifest_summary()?;

        ExtractorOfEntitiesRequiringAuth::extract(
            profile,
            summary
        )
    }
}

impl SignableID for TransactionIntentHash {}