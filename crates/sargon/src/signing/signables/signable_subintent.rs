use crate::prelude::*;
use radix_transactions::prelude::TransactionManifestV2Builder;

impl Signable for Subintent {
    type ID = SubintentHash;

    type Payload = CompiledSubintent;

    fn entities_requiring_signing(
        &self,
        profile: &Profile,
    ) -> Result<IndexSet<AccountOrPersona>> {
        let summary = self.intent_core.manifest.summary().unwrap();

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

        let mut builder = TransactionManifestV2Builder::new_v2();
        let network_id = network_id.unwrap_or_default();

        for (address, hash) in all_addresses_with_hashes {
            builder = builder.set_metadata(
                address.scrypto(),
                MetadataKey::OwnerKeys,
                ScryptoMetadataValue::PublicKeyHashArray(vec![hash.into()]),
            );
        }

        let manifest = TransactionManifestV2::sargon_built(builder, network_id);

        Self::new(IntentHeaderV2::sample(), manifest, MessageV2::None).unwrap()
    }
}

impl SignableID for SubintentHash {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn account_addresses_and_identity_addresses_require_auth() {
        let accounts = AccountAddress::sample_all();
        let identities = IdentityAddress::sample_all();

        let intent = Subintent::sample_entity_addresses_requiring_auth(
            accounts.clone(),
            identities.clone(),
        );

        let summary = intent.intent_core.manifest.summary().unwrap();

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
