use crate::prelude::*;

impl Signable for Subintent {
    type ID = SubintentHash;

    type Payload = CompiledSubintent;

    fn entities_requiring_signing(
        &self,
        profile: &Profile,
    ) -> Result<IndexSet<AccountOrPersona>> {
        let summary = self.intent_core.manifest.summary().unwrap();

        ExtractorOfEntitiesRequiringAuth::extract(profile, summary)
    }
}

impl SignableID for SubintentHash {}