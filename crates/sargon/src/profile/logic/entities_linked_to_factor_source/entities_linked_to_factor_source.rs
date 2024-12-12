use crate::prelude::*;

impl Profile {
    /// Returns the entities linked to a given `FactorSource` on the current network.
    pub fn entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        integrity: FactorSourceIntegrity,
    ) -> Result<EntitiesLinkedToFactorSource> {
        let accounts = self
            .accounts_on_current_network()?
            .iter()
            .filter(|a| {
                a.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect::<Accounts>();
        let hidden_accounts = self
            .hidden_accounts_on_current_network()?
            .iter()
            .filter(|a| {
                a.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect::<Accounts>();
        let personas = self
            .personas_on_current_network()?
            .iter()
            .filter(|p| {
                p.security_state
                    .is_linked_to_factor_source(factor_source.clone())
            })
            .collect::<Personas>();
        let hidden_personas = self
            .hidden_personas_on_current_network()?
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
