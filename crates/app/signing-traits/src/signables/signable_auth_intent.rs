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
        signatures: IndexSet<HDSignature<Self::ID>>,
    ) -> Result<Self::Signed> {
        let signatures_per_owner = signatures
        .iter()
        .map(|hd| (hd.input.owned_factor_instance.owner, IntentSignature(hd.signature)))
        .collect();
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
