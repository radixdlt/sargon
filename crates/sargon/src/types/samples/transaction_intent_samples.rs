use crate::prelude::*;
use radix_engine_toolkit::models::canonical_address_types::NetworkId;

impl TransactionIntent {
    pub(crate) fn new_requiring_auth(
        account_addresses_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        identity_addresses_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> TransactionIntent {
        let mut network_id: Option<NetworkID> = None;

        let all_addresses = account_addresses_requiring_auth
            .into_iter()
            .map(|addresses| AddressOfAccountOrPersona::Account(addresses))
            .chain(identity_addresses_requiring_auth.into_iter().map(
                |addresses| AddressOfAccountOrPersona::Identity(addresses),
            ))
            .collect::<Vec<_>>();

        all_addresses.iter().for_each(|address| {
            if let Some(network_id) = network_id {
                assert_eq!(network_id, address.network_id())
            } else {
                network_id = Some(address.network_id())
            }
        });

        let metadata =
            HashMap::<AddressOfAccountOrPersona, Vec<PublicKeyHash>>::from_iter(
                all_addresses.into_iter().map(|address| {
                    let pub_key = Ed25519PrivateKey::generate().public_key();
                    (
                        address,
                        vec![PublicKeyHash::hash(PublicKey::Ed25519(pub_key))],
                    )
                }),
            );

        let mut builder = ScryptoManifestBuilder::new();
        let network_id = network_id.unwrap_or_default();

        for (address, public_key_hashes) in metadata {
            builder = builder.set_metadata(
                address.scrypto(),
                MetadataKey::OwnerKeys,
                ScryptoMetadataValue::PublicKeyHashArray(
                    public_key_hashes
                        .into_iter()
                        .map(|h| h.clone().into())
                        .collect_vec(),
                ),
            );
        }

        let manifest = TransactionManifest::sargon_built(builder, network_id);

        TransactionIntent::new(
            TransactionHeader::sample(),
            manifest,
            Message::None,
        )
        .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn account_addresses_and_persona_addresses_require_auth() {
        let accounts = ALL_ACCOUNT_ADDRESSES_SAMPLES.clone();
        let identities = ALL_IDENTITY_ADDRESSES_SAMPLES.clone();

        let intent =
            TransactionIntent::new_requiring_auth(accounts, identities);

        let summary = intent.manifest_summary();

        assert_eq!(
            accounts.iter().sorted().collect_vec(),
            summary
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
