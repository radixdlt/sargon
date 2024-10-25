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

    fn sample_entity_addresses_with_pub_key_hashes_requiring_auth(
        account_addresses_requiring_auth: impl IntoIterator<
            Item = (AccountAddress, PublicKeyHash),
        >,
        identity_addresses_requiring_auth: impl IntoIterator<
            Item = (IdentityAddress, PublicKeyHash),
        >,
    ) -> Self {
        let mut network_id: Option<NetworkID> = None;

        let all_addresses_with_hashes = account_addresses_requiring_auth
            .into_iter()
            .map(|(address, hash)| {
                (AddressOfAccountOrPersona::from(address), hash)
            })
            .chain(identity_addresses_requiring_auth.into_iter().map(
                |(address, hash)| {
                    (AddressOfAccountOrPersona::from(address), hash)
                },
            ))
            .collect::<Vec<_>>();

        all_addresses_with_hashes
            .iter()
            .for_each(|(address, _hash)| {
                if let Some(network_id) = network_id {
                    assert_eq!(network_id, address.network_id())
                } else {
                    network_id = Some(address.network_id())
                }
            });

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
