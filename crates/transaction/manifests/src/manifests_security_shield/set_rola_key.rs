use radix_transactions::prelude::ManifestBuilder;

use crate::prelude::*;

pub trait TransactionManifestSetRolaKey: Sized {
    fn set_rola_key(
        builder: ManifestBuilder,
        authentication_signing_factor_instance: &HierarchicalDeterministicFactorInstance,
        entity_address: &AddressOfAccountOrPersona,
    ) -> ManifestBuilder;
}

impl TransactionManifestSetRolaKey for TransactionManifest {
    fn set_rola_key(
        builder: ManifestBuilder,
        authentication_signing_factor_instance:
        &HierarchicalDeterministicFactorInstance,
        entity_address: &AddressOfAccountOrPersona,
    ) -> ManifestBuilder {
        let rola_key_hash = PublicKeyHash::hash(
            authentication_signing_factor_instance.public_key(),
        );
        let owner_key_hashes = vec![rola_key_hash];
        Self::set_owner_keys_hashes_on_builder(
            entity_address,
            owner_key_hashes,
            builder,
        )
    }
}
