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
            .filter_map(|address| match address {
                AddressOfAccountOrPersona::Account(account_address) => profile
                    .account_by_address(*account_address)
                    .map(AccountOrPersona::AccountEntity)
                    .ok(),
                AddressOfAccountOrPersona::Identity(identity_address) => {
                    profile
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
        let profile = Profile::sample();
        let expected_accounts = profile.accounts_on_current_network().unwrap();

        let unknown_address_to_profile = AddressOfAccountOrPersona::Account(
            Account::sample_mainnet_bob().address,
        );
        let mut addresses_requested = expected_accounts
            .clone()
            .into_iter()
            .map(|account| AddressOfAccountOrPersona::Account(account.address))
            .collect_vec();
        // Push an unknown address, this should be filtered out from the result
        addresses_requested.push(unknown_address_to_profile);

        let auth_intent = SUT::new_from_request(
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
            addresses_requested,
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
    fn test_signed() {
        let intent = SUT::sample_other();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::sample();

        let signature = mnemonic_with_passphrase
            .sign(&intent.auth_intent_hash().hash(), &DerivationPath::sample());
        let intent_signatures = indexmap!(
           AddressOfAccountOrPersona::sample() => IntentSignature(signature)
        );

        assert_eq!(
            intent.signed(intent_signatures.clone()).unwrap(),
            SignedAuthIntent::new(intent.clone(), intent_signatures).unwrap()
        )
    }
}