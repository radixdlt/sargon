use crate::prelude::*;

/// Utility to extract factor instances required to sign transactions.
pub struct ExtractorOfInstancesRequiredToSignTransactions;

impl ExtractorOfInstancesRequiredToSignTransactions {
    /// Extracts factor instances required to sign transactions.
    /// Returns a set of `HierarchicalDeterministicFactorInstance`.
    /// Returns an error if the `SignaturesCollectorPreprocessor` fails to initialize.
    pub fn extract(
        profile: &Profile,
        transactions: Vec<TransactionIntent>,
        for_any_securified_entity_select_role: RoleKind,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        let preprocessor =
            SignaturesCollectorPreprocessor::analyzing_transaction_intents(
                profile,
                transactions,
            )?;
        let (petitions, _) = preprocessor.preprocess(
            IndexSet::from_iter(profile.factor_sources.iter()),
            for_any_securified_entity_select_role,
        );

        let factor_instances = petitions
            .txid_to_petition
            .borrow()
            .values()
            .flat_map(|p| {
                p.for_entities
                    .borrow()
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
            .derive_entity_creation_factor_instance(NetworkID::Mainnet, 0);
        let account_creating_factor_instance_2 = private_hd_factor_source
            .derive_entity_creation_factor_instance(NetworkID::Mainnet, 1);

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
            .derive_entity_creation_factor_instance(NetworkID::Mainnet, 1);
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
