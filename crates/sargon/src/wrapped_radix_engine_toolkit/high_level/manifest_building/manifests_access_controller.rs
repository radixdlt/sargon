use radix_engine_interface::blueprints::{
    access_controller::AccessControllerCreateManifestInput as ScryptoAccessControllerCreateManifestInput,
    account::ACCOUNT_SECURIFY_IDENT as SCRYPTO_ACCOUNT_SECURIFY_IDENT,
    identity::IDENTITY_SECURIFY_IDENT as SCRYPTO_IDENTITY_SECURIFY_IDENT,
};

use crate::prelude::*;

/*
pub const ACCESS_CONTROLLER_CREATE_IDENT: &str = "create";

#[derive(Debug, Eq, PartialEq, ScryptoSbor)]
pub struct AccessControllerCreateInput {
    pub controlled_asset: Bucket,
    pub rule_set: RuleSet,
    pub timed_recovery_delay_in_minutes: Option<u32>,
    pub address_reservation: Option<GlobalAddressReservation>,
}

#[derive(Debug, Eq, PartialEq, ManifestSbor)]
pub struct AccessControllerCreateManifestInput {
    pub controlled_asset: ManifestBucket,
    pub rule_set: RuleSet,
    pub timed_recovery_delay_in_minutes: Option<u32>,
    pub address_reservation: Option<ManifestAddressReservation>,
}

pub type AccessControllerCreateOutput = Global<AccessControllerMarker>;
*/
impl From<SecurityStructureOfFactorInstances>
    for ScryptoAccessControllerCreateManifestInput
{
    fn from(value: SecurityStructureOfFactorInstances) -> Self {
        todo!()
    }
}

impl From<AddressOfAccountOrPersona> for Address {
    fn from(value: AddressOfAccountOrPersona) -> Self {
        todo!()
    }
}

impl TransactionManifest {
    pub fn securify_unsecurified_entity(
        entity: UnsecurifiedEntity,
        security_structure_of_factor_instances: SecurityStructureOfFactorInstances,
    ) -> Self {
        let entity_address = entity.address();
        let security_entity_identifier = if entity_address.is_identity() {
            SCRYPTO_IDENTITY_SECURIFY_IDENT
        } else {
            SCRYPTO_ACCOUNT_SECURIFY_IDENT
        };
        let entity_address = Address::from(entity_address);

        let mut builder = ScryptoTransactionManifestBuilder::new();
        let bucket_factory = BucketFactory::default();

        // Securify the account which will return an account owner badge onto the worktop.
        builder =
            builder.call_method(entity_address, security_entity_identifier, ());

        // Create a bucket out of the account owner badge.
        let owner_badge = SCRYPTO_ACCOUNT_OWNER_BADGE;
        let owner_badge_bucket = &bucket_factory.next();
        builder =
            builder.take_all_from_worktop(owner_badge, owner_badge_bucket);

        // Create a proof from the account owner badge
        let owner_badge_proof = "owner_badge_proof";
        builder = builder.create_proof_from_bucket_of_all(
            owner_badge_bucket,
            owner_badge_proof,
        );

        // Push the proof to the auth zone
        builder = builder.push_to_auth_zone(owner_badge_proof);

        // We've changed the account deposit rules, so now we can drop all the proofs in the auth zone (which
        // is pretty much a single proof of the account owner badge).
        builder = builder.drop_auth_zone_proofs();

        // We deposit the account's owner badge into it which locks it forever.
        builder = builder.try_deposit_or_abort(
            entity_address,
            None,
            owner_badge_bucket,
        );

        TransactionManifest::sargon_built(builder, entity_address.network_id())
    }
}
