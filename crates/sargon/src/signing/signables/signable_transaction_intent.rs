use crate::prelude::*;

impl Signable for TransactionIntent {
    type ID = TransactionIntentHash;

    type Payload = CompiledTransactionIntent;

    type Signed = SignedIntent;

    fn entities_requiring_signing(
        &self,
        profile: &Profile,
    ) -> Result<IndexSet<AccountOrPersona>> {
        let summary = self.manifest_summary()?;

        ExtractorOfEntitiesRequiringAuth::extract(profile, summary)
    }

    fn signed(
        &self,
        intent_signatures: IntentSignatures,
    ) -> Result<Self::Signed> {
        SignedIntent::new(self.clone(), intent_signatures)
    }

    fn sample_entity_addresses_with_pub_key_hashes(
        all_addresses_with_hashes: Vec<(
            AddressOfAccountOrPersona,
            PublicKeyHash,
        )>,
        network_id: Option<NetworkID>,
    ) -> Self {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        let network_id = network_id.unwrap_or_default();

        for (address, hash) in all_addresses_with_hashes {
            builder = builder.set_metadata(
                address.scrypto(),
                MetadataKey::OwnerKeys,
                ScryptoMetadataValue::PublicKeyHashArray(vec![hash.into()]),
            );
        }

        let manifest = TransactionManifest::sargon_built(builder, network_id);

        Self::new(TransactionHeader::sample(), manifest, Message::None).unwrap()
    }
}

impl From<SignedIntent> for TransactionIntent {
    fn from(val: SignedIntent) -> Self {
        val.intent
    }
}

impl IntoIterator for SignedIntent {
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

impl SignableID for TransactionIntentHash {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn account_addresses_and_identity_addresses_require_auth() {
        let accounts = AccountAddress::sample_all();
        let identities = IdentityAddress::sample_all();

        let intent = TransactionIntent::sample_entity_addresses_requiring_auth(
            accounts.clone(),
            identities.clone(),
        );

        let summary = intent.manifest_summary().unwrap();
        assert_eq!(
            accounts.len(),
            HashSet::<AccountAddress>::from_iter(accounts.clone()).len()
        );
        pretty_assertions::assert_eq!(
            accounts.iter().sorted().collect_vec(),
            summary
                .clone()
                .addresses_of_accounts_requiring_auth
                .iter()
                .sorted()
                .collect_vec()
        );

        pretty_assertions::assert_eq!(
            identities.iter().sorted().collect_vec(),
            summary
                .addresses_of_personas_requiring_auth
                .iter()
                .sorted()
                .collect_vec()
        );
    }

    #[test]
    fn from_signed_intent() {
        let signed_intent = SignedIntent::sample();

        assert_eq!(
            <TransactionIntent as From::<SignedIntent>>::from(signed_intent),
            TransactionIntent::sample_other()
        )
    }

    #[test]
    fn signed_subintent_into_signatures() {
        let signed_intent = SignedIntent::sample();

        assert_eq!(
            signed_intent.clone().into_iter().collect_vec(),
            signed_intent.intent_signatures.signatures.into_iter().map(|s| s.0).collect_vec()
        )
    }
}
