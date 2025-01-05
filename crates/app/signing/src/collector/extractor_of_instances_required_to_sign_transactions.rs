use crate::prelude::*;

/// Utility to extract factor instances required to sign transactions.
pub struct ExtractorOfInstancesRequiredToSignTransactions;

impl ExtractorOfInstancesRequiredToSignTransactions {
    /// Extracts factor instances required to sign transactions.
    /// Returns a set of `HierarchicalDeterministicFactorInstance`.
    /// Returns an error if the `SignaturesCollectorPreprocessor` fails to initialize.
    pub fn extract<S: Signable, P: GetEntityByAddress + HasFactorSources>(
        proto_profile: &P,
        transactions: Vec<S>,
        signing_purpose: SigningPurpose,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        let preprocessor =
            SignaturesCollectorPreprocessor::analyzing_signables(
                proto_profile,
                transactions,
            )?;
        let (petitions, _) = preprocessor
            .preprocess(proto_profile.factor_sources(), signing_purpose);

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
#[derive(Debug, Default)]
pub(crate) struct ProtoProfile {
    pub(crate) accounts: Vec<Account>,
    pub(crate) personas: Vec<Persona>,
    pub(crate) factor_sources: Vec<FactorSource>,
}

#[cfg(test)]
impl ProtoProfile {
    pub(crate) fn new(
        accounts: impl IntoIterator<Item = Account>,
        personas: impl IntoIterator<Item = Persona>,
        factor_sources: impl IntoIterator<Item = FactorSource>,
    ) -> Self {
        Self {
            accounts: accounts.into_iter().collect(),
            personas: personas.into_iter().collect(),
            factor_sources: factor_sources.into_iter().collect(),
        }
    }
}

#[cfg(test)]
impl ProfileAccountByAddress for ProtoProfile {
    fn account_by_address(&self, address: AccountAddress) -> Result<Account> {
        self.accounts
            .iter()
            .find(|a| a.address == address)
            .cloned()
            .ok_or(CommonError::UnknownAccount)
    }
}
#[cfg(test)]
impl ProfilePersonaByAddress for ProtoProfile {
    fn persona_by_address(&self, address: IdentityAddress) -> Result<Persona> {
        self.personas
            .iter()
            .find(|p| p.address == address)
            .cloned()
            .ok_or(CommonError::UnknownPersona)
    }
}
#[cfg(test)]
impl ProfileEntityByAddress for ProtoProfile {
    fn entity_by_address(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona> {
        if let Some(account_address) = address.as_account() {
            return self
                .account_by_address(*account_address)
                .map(AccountOrPersona::AccountEntity);
        }

        if let Some(identity_address) = address.as_identity() {
            return self
                .persona_by_address(*identity_address)
                .map(AccountOrPersona::PersonaEntity);
        }

        Err(CommonError::Unknown)
    }
}
#[cfg(test)]
impl HasFactorSources for ProtoProfile {
    fn factor_sources(&self) -> IndexSet<FactorSource> {
        self.factor_sources.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use entity_foundation::prelude::AppearanceID;
    use profile_security_structures::prelude::DisplayName;

    use super::*;

    #[test]
    fn preprocessor_init_fail() {
        let intent_with_invalid_persona =
            TransactionIntent::sample_entity_addresses_requiring_auth(
                vec![],
                vec![Persona::sample_mainnet().address],
            );

        let result = ExtractorOfInstancesRequiredToSignTransactions::extract(
            &ProtoProfile::default(),
            vec![intent_with_invalid_persona],
            SigningPurpose::sign_transaction_primary(),
        );

        assert!(matches!(result, Err(CommonError::UnknownPersona)));
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

        let profile = ProtoProfile::new(
            vec![account_1.clone(), account_2.clone()],
            vec![persona.clone()],
            vec![private_hd_factor_source.factor_source.clone().into()],
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
            &profile,
            vec![intent_1, intent_2],
            SigningPurpose::sign_transaction_primary(),
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
