use crate::prelude::*;

/// Utility to extract factor instances required to sign transactions.
pub struct ExtractorOfInstancesRequiredToSignTransactions;

impl ExtractorOfInstancesRequiredToSignTransactions {
    /// Extracts factor instances required to sign transactions.
    /// Returns a set of `HierarchicalDeterministicFactorInstance`.
    /// Returns an error if the `SignaturesCollectorPreprocessor` fails to initialize.
    pub fn extract<S: Signable>(
        profile: &Profile,
        transactions: Vec<S>,
        for_any_securified_entity_select_role: RoleKind,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        let preprocessor =
            SignaturesCollectorPreprocessor::analyzing_signables(
                profile,
                transactions,
            )?;
        let (petitions, _) = preprocessor.preprocess(
            IndexSet::from_iter(profile.factor_sources.iter()),
            for_any_securified_entity_select_role,
        );

        let factor_instances = petitions
            .txid_to_petition
            .read()
            .expect("Petitions lock should not have been poisoned.")
            .values()
            .flat_map(|p| {
                p.for_entities
                    .read()
                    .expect("PetitionForTransaction lock should not have been poisoned.")
                    .values()
                    .flat_map(|p| p.all_factor_instances())
                    .collect::<Vec<_>>()
            })
            .map(|p| p.factor_instance().clone())
            .collect::<IndexSet<HierarchicalDeterministicFactorInstance>>();
        Ok(factor_instances)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preprocessor_init_fail() {
        let result = ExtractorOfInstancesRequiredToSignTransactions::extract(
            &Profile::sample_other(),
            vec![TransactionIntent::sample()],
            RoleKind::Primary,
        );

        assert!(matches!(result, Err(CommonError::UnknownAccount)));
    }

    #[test]
    fn success() {
        let private_hd_factor_source =
            PrivateHierarchicalDeterministicFactorSource::sample();
        let account_creating_factor_instance_1 = private_hd_factor_source
            ._derive_entity_creation_factor_instance(
                NetworkID::Mainnet,
                HDPathComponent::unsecurified_hardened(0).unwrap(),
            );
        let account_creating_factor_instance_2 = private_hd_factor_source
            ._derive_entity_creation_factor_instance(
                NetworkID::Mainnet,
                HDPathComponent::unsecurified_hardened(1).unwrap(),
            );

        let account_1 = Account::new(
            account_creating_factor_instance_1.clone(),
            DisplayName::sample(),
            AppearanceID::sample(),
        );
        let account_2 = Account::new(
            account_creating_factor_instance_2.clone(),
            DisplayName::sample(),
            AppearanceID::sample(),
        );

        let persona_creating_factor_instance = private_hd_factor_source
            ._derive_entity_creation_factor_instance(
                NetworkID::Mainnet,
                HDPathComponent::unsecurified_hardened(1).unwrap(),
            );
        let persona = Persona::new(
            persona_creating_factor_instance.clone(),
            DisplayName::sample(),
            None,
        );

        let intent_1 =
            TransactionIntent::sample_entity_addresses_requiring_auth(
                vec![account_1.address],
                vec![persona.address],
            );
        let intent_2 =
            TransactionIntent::sample_entity_addresses_requiring_auth(
                vec![account_2.address],
                vec![],
            );

        let result = ExtractorOfInstancesRequiredToSignTransactions::extract(
            &Profile::sample(),
            vec![intent_1, intent_2],
            RoleKind::Primary,
        );

        let account_fi_1 = HierarchicalDeterministicFactorInstance::from(
            account_creating_factor_instance_1,
        );
        let account_fi_2 = HierarchicalDeterministicFactorInstance::from(
            account_creating_factor_instance_2,
        );
        let persona_fi = HierarchicalDeterministicFactorInstance::from(
            persona_creating_factor_instance,
        );

        assert_eq!(
            result,
            Ok(IndexSet::from_iter(vec![
                account_fi_1,
                persona_fi,
                account_fi_2
            ]))
        );
    }
}
