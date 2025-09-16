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

#[cfg(test)]
mod auth_intent_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthIntent;

    #[test]
    fn test_get_entities_requiring_signing() {
        let profile = ProtoProfile::new(
            [Account::sample_alice()],
            [Persona::sample_mainnet()],
            [FactorSource::sample_device()],
        );
        let accounts_in_profile = profile.accounts.clone();
        let personas_in_profile = profile.personas.clone();

        let unknown_account_address_to_profile =
            AddressOfAccountOrPersona::Account(
                Account::sample_mainnet_bob().address,
            );
        let unknown_identity_address_to_profile =
            AddressOfAccountOrPersona::Identity(
                Persona::sample_mainnet_third().address,
            );

        let mut addresses_requested = accounts_in_profile
            .clone()
            .into_iter()
            .map(|account| AddressOfAccountOrPersona::Account(account.address))
            .collect_vec();
        // Push an unknown address, this should be filtered out from the result
        addresses_requested.push(unknown_account_address_to_profile);

        addresses_requested.extend(
            personas_in_profile
                .clone()
                .into_iter()
                .map(|persona| {
                    AddressOfAccountOrPersona::Identity(persona.address)
                })
                .collect_vec(),
        );
        addresses_requested.push(unknown_identity_address_to_profile);

        let auth_intent = SUT::new_from_request(
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
            addresses_requested,
        )
        .unwrap();

        let mut expected_entities = accounts_in_profile
            .into_iter()
            .map(AccountOrPersona::AccountEntity)
            .collect_vec();

        expected_entities.extend(
            personas_in_profile
                .into_iter()
                .map(AccountOrPersona::PersonaEntity),
        );

        assert_eq!(
            auth_intent.entities_requiring_signing(&profile).unwrap(),
            IndexSet::<AccountOrPersona>::from_iter(expected_entities)
        )
    }

    #[test]
    fn test_signed() {
        let sut = SUT::sample_other();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();
        let signature = mnemonic_with_passphrase
            .sign(&sut.auth_intent_hash().hash(), &DerivationPath::sample());
        let hd_signature= HDSignature {
            input: HDSignatureInput {
                payload_id: sut.get_id(),
                owned_factor_instance: OwnedFactorInstance::sample(),
            },
            signature,
        };
        let intent_signatures = IndexMap::kv(
            AddressOfAccountOrPersona::sample(),
            IntentSignature(signature),
        );

        let signed = sut.signed(vec![hd_signature]).unwrap();

        assert_eq!(
            signed,
            SignedAuthIntent::new(sut.clone(), intent_signatures).unwrap()
        );
        assert_eq!(AuthIntent::from(signed), sut)
    }

    #[test]
    fn test_signed_get_signatures() {
        let sut = SUT::sample();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();
        let signature = mnemonic_with_passphrase
            .sign(&sut.auth_intent_hash().hash(), &DerivationPath::sample());
        let intent_signatures = IndexMap::kv(
            AddressOfAccountOrPersona::sample(),
            IntentSignature(signature),
        );

        let signed = sut.signed(intent_signatures.clone()).unwrap();

        assert_eq!(
            signed.into_iter().collect_vec(),
            intent_signatures
                .values()
                .cloned()
                .map(|i| i.0)
                .collect_vec()
        )
    }
}
