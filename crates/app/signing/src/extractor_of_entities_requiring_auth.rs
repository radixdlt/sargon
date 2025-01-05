use crate::prelude::*;

/// Utility to extract entities requiring auth from a profile and a manifest summary.
pub struct ExtractorOfEntitiesRequiringAuth;
impl ExtractorOfEntitiesRequiringAuth {
    /// Matches entities requiring auth from a manifest summary with the entities in the given profile.
    /// Returns a set of `AccountOrPersona` or empty if the manifest summary does not require auth.
    /// Returns an error if persona is unknown.
    pub fn extract(
        entity_querying: &impl GetEntityByAddress,
        summary: ManifestSummary,
    ) -> Result<IndexSet<AccountOrPersona>> {
        let mut entities_requiring_auth: IndexSet<AccountOrPersona> =
            IndexSet::new();

        let accounts = summary
            .addresses_of_accounts_requiring_auth
            .iter()
            .map(|a| entity_querying.account_by_address(*a))
            .filter_map(|a| a.ok())
            .collect::<Vec<_>>();

        entities_requiring_auth.extend(
            accounts
                .into_iter()
                .map(AccountOrPersona::from)
                .collect_vec(),
        );

        let personas = summary
            .addresses_of_personas_requiring_auth
            .into_iter()
            .map(|a| entity_querying.persona_by_address(a))
            .collect::<Result<Vec<_>>>()?;

        entities_requiring_auth.extend(
            personas
                .into_iter()
                .map(AccountOrPersona::from)
                .collect_vec(),
        );
        Ok(entities_requiring_auth)
    }
}
