use crate::prelude::*;

pub trait ProfileNetworkQueryEntitiesLinkedToFactorSource {
    fn entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        integrity: FactorSourceIntegrity,
    ) -> Result<EntitiesLinkedToFactorSource>;
}

impl ProfileNetworkQueryEntitiesLinkedToFactorSource for ProfileNetwork {
    /// Returns the entities linked to a given `FactorSource` on the current `ProfileNetwork`.
    fn entities_linked_to_factor_source(
        &self,
        factor_source: FactorSource,
        integrity: FactorSourceIntegrity,
    ) -> Result<EntitiesLinkedToFactorSource> {
        fn filter(
            e: &impl HasSecurityState,
            factor_source: FactorSource,
        ) -> bool {
            e.security_state()
                .is_linked_to_factor_source(factor_source.clone())
        }

        let accounts = self
            .accounts_non_hidden()
            .iter()
            .filter(|a| filter(a, factor_source.clone()))
            .collect::<Accounts>();
        let hidden_accounts = self
            .accounts_hidden()
            .iter()
            .filter(|a| filter(a, factor_source.clone()))
            .collect::<Accounts>();
        let personas = self
            .personas_non_hidden()
            .iter()
            .filter(|p| filter(p, factor_source.clone()))
            .collect::<Personas>();
        let hidden_personas = self
            .personas_hidden()
            .iter()
            .filter(|p| filter(p, factor_source.clone()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileNetwork;

    #[test]
    fn entities_linked_to_factor_source() {
        // Set up SUT

        // Two visible accounts
        let accounts = Accounts::from_iter([
            Account::sample_stokenet_nadia(),
            Account::sample_stokenet_paige(),
        ]);
        // One hidden account
        let hidden_accounts =
            Accounts::from_iter([Account::sample_stokenet_olivia()]);
        // Two visible personas
        let personas = Personas::from_iter([
            Persona::sample_stokenet_leia_skywalker(),
            Persona::sample_stokenet_connor(),
        ]);
        // One hidden persona
        let hidden_personas =
            Personas::from_iter([Persona::sample_stokenet_hermione()]);

        let all_accounts = accounts
            .iter()
            .chain(hidden_accounts.iter())
            .collect::<Accounts>();
        let all_personas = personas
            .iter()
            .chain(hidden_personas.iter())
            .collect::<Personas>();
        let sut = SUT::new(
            NetworkID::Stokenet,
            all_accounts,
            all_personas,
            AuthorizedDapps::sample_stokenet(),
            ResourcePreferences::sample_stokenet(),
            MFAFactorInstances::sample_stokenet(),
        );

        let result = sut
            .entities_linked_to_factor_source(
                FactorSource::sample(),
                FactorSourceIntegrity::sample(),
            )
            .unwrap();

        assert_eq!(result.accounts, accounts);
        assert_eq!(result.hidden_accounts, hidden_accounts);
        assert_eq!(result.personas, personas);
        assert_eq!(result.hidden_personas, hidden_personas);
    }
}
