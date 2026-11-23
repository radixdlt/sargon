use profile_supporting_types::AnySecurifiedEntity;
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

pub trait TransactionManifestSetRolaKeyOfSecurifiedEntity:
    TransactionManifestSetRolaKey
{
    /// A transaction of its own that sets the ROLA key of `securified_entity`
    /// to `authentication_signing_factor_instance`, authorized by a proof of
    /// its access controller's badge, so signed with the primary role.
    ///
    /// Securifying or recovering an entity doesn't set its ROLA key: those
    /// transactions only operate on access controllers, so that their review
    /// shows all they do. This one follows once the new rules are in place.
    fn set_rola_key_of_securified_entity(
        securified_entity: &AnySecurifiedEntity,
        authentication_signing_factor_instance:
            &HierarchicalDeterministicFactorInstance,
    ) -> TransactionManifest {
        let builder = ScryptoTransactionManifestBuilder::new().call_method(
            securified_entity.access_controller_address().scrypto(),
            SCRYPTO_ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
            (),
        );
        let builder = TransactionManifest::set_rola_key(
            builder,
            authentication_signing_factor_instance,
            &securified_entity.entity.address(),
        );
        TransactionManifest::sargon_built(
            builder,
            securified_entity.network_id(),
        )
    }
}

impl TransactionManifestSetRolaKeyOfSecurifiedEntity for TransactionManifest {}
