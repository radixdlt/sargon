use crate::prelude::*;

impl Signable for AuthIntent {
    type ID = AuthIntentHash;
    type Payload = Self;
    type Signed = SignedAuthIntent;

    fn entities_requiring_signing(
        &self,
        profile: &Profile,
    ) -> Result<IndexSet<AccountOrPersona>> {
        let entities = self
            .entities_to_sign
            .iter()
            .map(|address| match address {
                AddressOfAccountOrPersona::Account(account_address) => profile
                    .account_by_address(*account_address)
                    .map(AccountOrPersona::AccountEntity),
                AddressOfAccountOrPersona::Identity(identity_address) => {
                    profile
                        .persona_by_address(*identity_address)
                        .map(AccountOrPersona::PersonaEntity)
                }
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(IndexSet::from_iter(entities))
    }

    fn signed(
        &self,
        intent_signatures: IntentSignatures,
    ) -> Result<Self::Signed> {
        SignedAuthIntent::with_signatures(
            self.clone(),
            intent_signatures.signatures,
        )
    }

    fn sample_entity_addresses_with_pub_key_hashes(
        _all_addresses_with_hashes: Vec<(
            AddressOfAccountOrPersona,
            PublicKeyHash,
        )>,
        _network_id: Option<NetworkID>,
    ) -> Self {
        todo!()
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
        self.intent_signatures
            .signatures
            .into_iter()
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
        let profile = Profile::sample();
        let expected_accounts = profile.accounts_on_current_network().unwrap();

        let auth_intent = SUT::new_from_request(
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
            expected_accounts
                .clone()
                .into_iter()
                .map(|account| {
                    AddressOfAccountOrPersona::Account(account.address)
                })
                .collect_vec(),
        )
        .unwrap();

        assert_eq!(
            auth_intent.entities_requiring_signing(&profile).unwrap(),
            IndexSet::<AccountOrPersona>::from_iter(
                expected_accounts
                    .into_iter()
                    .map(AccountOrPersona::AccountEntity)
            )
        )
    }

    #[test]
    fn test_get_entities_requiring_signing_fails_due_to_account_not_existing_in_profile(
    ) {
        let profile = Profile::sample();

        let auth_intent = SUT::new_from_request(
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
            vec![AddressOfAccountOrPersona::Account(
                AccountAddress::sample_frank(),
            )],
        )
        .unwrap();

        assert_eq!(
            auth_intent.entities_requiring_signing(&profile),
            Err(CommonError::UnknownAccount)
        )
    }

    #[test]
    fn test_get_entities_requiring_signing_fails_due_to_persona_not_existing_in_profile(
    ) {
        let profile = Profile::sample();

        let auth_intent = SUT::new_from_request(
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
            vec![AddressOfAccountOrPersona::Identity(
                Persona::sample_mainnet_ripley().address,
            )],
        )
        .unwrap();

        assert_eq!(
            auth_intent.entities_requiring_signing(&profile),
            Err(CommonError::UnknownPersona)
        )
    }

    #[test]
    fn test_signed() {
        let intent = SUT::sample_other();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();

        let signature = mnemonic_with_passphrase
            .sign(&intent.auth_intent_hash().hash(), &DerivationPath::sample());
        let intent_signatures =
            IntentSignatures::new(vec![IntentSignature(signature)]);

        assert_eq!(
            intent.signed(intent_signatures.clone()).unwrap(),
            SignedAuthIntent::new(intent.clone(), intent_signatures).unwrap()
        )
    }
}
