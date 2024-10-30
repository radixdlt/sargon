use crate::prelude::*;

impl Signable for TransactionIntent {
    type ID = TransactionIntentHash;

    type Payload = CompiledTransactionIntent;

    fn entities_requiring_signing(
        &self,
        profile: &Profile,
    ) -> Result<IndexSet<AccountOrPersona>> {
        let summary = self.manifest_summary()?;

        ExtractorOfEntitiesRequiringAuth::extract(profile, summary)
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
            accounts.iter().sorted().collect_vec(),
            summary
                .clone()
                .addresses_of_accounts_requiring_auth
                .iter()
                .sorted()
                .collect_vec()
        );

        assert_eq!(
            identities.iter().sorted().collect_vec(),
            summary
                .addresses_of_personas_requiring_auth
                .iter()
                .sorted()
                .collect_vec()
        );
    }
}
