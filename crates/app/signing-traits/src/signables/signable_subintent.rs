use crate::prelude::*;
use manifests::StaticallyAnalyzableManifest;

impl Signable for Subintent {
    type ID = SubintentHash;

    type Payload = CompiledSubintent;

    type Signed = SignedSubintent;

    fn entities_requiring_signing(
        &self,
        entity_querying: &impl GetEntityByAddress,
    ) -> Result<IndexSet<AccountOrPersona>> {
        let summary = self.manifest.summary().unwrap();

        ExtractorOfEntitiesRequiringAuth::extract(entity_querying, summary)
    }

    fn signed(
        &self,
        signatures: IndexSet<HDSignature<Self::ID>>,
    ) -> Result<Self::Signed> {
        let intent_signatures = signatures
            .into_iter()
            .map(|hd| IntentSignature(hd.signature))
            .collect_vec();
        SignedSubintent::new(
            self.clone(),
            IntentSignatures::new(intent_signatures),
        )
    }
}

impl SignableID for SubintentHash {}

impl ProvidesSamplesByBuildingManifest for Subintent {
    fn sample_entity_addresses_with_pub_key_hashes(
        all_addresses_with_hashes: Vec<(
            AddressOfAccountOrPersona,
            PublicKeyHash,
        )>,
        network_id: Option<NetworkID>,
    ) -> Self {
        let mut builder = ScryptoSubintentManifestV2Builder::new_subintent_v2();
        let network_id = network_id.unwrap_or_default();

        for (address, hash) in all_addresses_with_hashes {
            builder = builder.set_metadata(
                address.scrypto(),
                MetadataKey::OwnerKeys,
                ScryptoMetadataValue::PublicKeyHashArray(vec![hash.into()]),
            );
        }

        builder = builder.yield_to_parent(());

        let manifest = SubintentManifest::sargon_built(builder, network_id);

        Self::new(IntentHeaderV2::sample(), manifest, MessageV2::None).unwrap()
    }
}

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

        let summary = intent.manifest.summary().unwrap();

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

    #[test]
    fn from_signed_subintent() {
        let signed_subintent = SignedSubintent::sample();

        assert_eq!(
            <Subintent as From::<SignedSubintent>>::from(signed_subintent),
            Subintent::sample()
        )
    }

    #[test]
    fn signed_subintent_into_signatures() {
        let signed_subintent = SignedSubintent::sample();

        assert_eq!(
            signed_subintent.clone().into_iter().collect_vec(),
            signed_subintent
                .subintent_signatures
                .signatures
                .into_iter()
                .map(|s| s.0)
                .collect_vec()
        )
    }
}
