use crate::prelude::*;
use radix_engine_toolkit::models::canonical_address_types::NetworkId;
use reqwest::Identity;

impl TransactionIntent {

    /// Returns a sample intent that its transaction summary will involve all the
    /// `accounts_requiring_auth` and `personas_requiring_auth` in entities requiring auth.
    /// This can be accomplished by building a manifest that constructs owner keys from these
    /// entities
    pub fn entities_requiring_auth<'a, 'p>(
        accounts_requiring_auth: impl IntoIterator<Item = &'a Account>,
        personas_requiring_auth: impl IntoIterator<Item = &'p Persona>,
    ) -> Self {
        Self::new_requiring_auth(
            accounts_requiring_auth.into_iter().map(|a| a.address),
            personas_requiring_auth.into_iter().map(|p| p.address),
        )
    }

    /// Returns a sample intent that its transaction summary will involve all the
    /// `account_addresses_requiring_auth` and `identity_addresses_requiring_auth` in
    /// entities requiring auth.
    /// This can be accomplished by building a manifest that constructs owner keys from these
    /// entity addresses.
    pub fn new_requiring_auth(
        account_addresses_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        identity_addresses_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> Self {
        let mut network_id: Option<NetworkID> = None;

        let all_addresses = account_addresses_requiring_auth
            .into_iter()
            .map(|addresses| AddressOfAccountOrPersona::from(addresses))
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

        Self::new(
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
    fn account_addresses_and_identity_addresses_require_auth() {
        let accounts = AccountAddress::sample_all();
        let identities = IdentityAddress::sample_all();

        let intent =
            TransactionIntent::new_requiring_auth(accounts.clone(), identities.clone());

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
