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

    fn sample_entity_addresses_with_pub_key_hashes_requiring_auth(
        account_addresses_requiring_auth: impl IntoIterator<
            Item = (AccountAddress, PublicKeyHash),
        >,
        identity_addresses_requiring_auth: impl IntoIterator<
            Item = (IdentityAddress, PublicKeyHash),
        >,
    ) -> Self {
        todo!()
    }
}

impl SignableID for SubintentHash {}
