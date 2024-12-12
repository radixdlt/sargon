use crate::prelude::*;

impl ProfileNetwork {
    /// Returns the entities linked to a given `FactorSource` on the current `ProfileNetwork`.
    pub fn entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        integrity: FactorSourceIntegrity,
    ) -> Result<EntitiesLinkedToFactorSource> {
        let accounts = self
            .accounts_non_hidden()
            .iter()
            .filter(|a| {
                a.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect::<Accounts>();
        let hidden_accounts = self
            .accounts
            .hidden()
            .iter()
            .filter(|a| {
                a.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect::<Accounts>();
        let personas = self
            .personas_non_hidden()
            .iter()
            .filter(|p| {
                p.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect::<Personas>();
        let hidden_personas = self
            .personas
            .hidden()
            .iter()
            .filter(|p| {
                p.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect::<Personas>();

        Ok(EntitiesLinkedToFactorSource::new(
            integrity,
            accounts,
            hidden_accounts,
            personas,
            hidden_personas,
        ))
    }
}
