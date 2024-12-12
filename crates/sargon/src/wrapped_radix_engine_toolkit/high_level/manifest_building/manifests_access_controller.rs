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

impl TransactionManifest {
    pub fn securify_unsecurified_entity(
        entity: UnsecurifiedEntity,
        security_structure_of_factor_instances: SecurityStructureOfFactorInstances,
    ) -> Self {
        todo!()
    }
}
