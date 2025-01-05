use crate::prelude::*;

impl Signable for AuthIntent {
    type ID = AuthIntentHash;
    type Payload = Self;
    type Signed = SignedAuthIntent;

    fn entities_requiring_signing(
        &self,
        entity_querying: &impl GetEntityByAddress,
    ) -> Result<IndexSet<AccountOrPersona>> {
        let entities = self
            .entities_to_sign
            .iter()
            .filter_map(|address| match address {
                AddressOfAccountOrPersona::Account(account_address) => {
                    entity_querying
                        .account_by_address(*account_address)
                        .map(AccountOrPersona::AccountEntity)
                        .ok()
                }
                AddressOfAccountOrPersona::Identity(identity_address) => {
                    entity_querying
                        .persona_by_address(*identity_address)
                        .map(AccountOrPersona::PersonaEntity)
                        .ok()
                }
            })
            .collect_vec();

        Ok(IndexSet::from_iter(entities))
    }

    fn signed(
        &self,
        signatures_per_owner: IndexMap<
            AddressOfAccountOrPersona,
            IntentSignature,
        >,
    ) -> Result<Self::Signed> {
        SignedAuthIntent::new(self.clone(), signatures_per_owner)
    }
}

impl From<SignedAuthIntent> for AuthIntent {
    fn from(val: SignedAuthIntent) -> Self {
        val.intent
    }
}

impl IntoIterator for SignedAuthIntent {
    type Item = SignatureWithPublicKey;
    type IntoIter = <Vec<SignatureWithPublicKey> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.intent_signatures_per_owner
            .values()
            .map(|s| s.0)
            .collect_vec()
            .into_iter()
    }
}

impl SignableID for AuthIntentHash {}

#[cfg(test)]
mod tests {
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
        let intent_signatures = IndexMap::kv(
            AddressOfAccountOrPersona::sample(),
            IntentSignature(signature),
        );

        let signed = sut.signed(intent_signatures.clone()).unwrap();

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
