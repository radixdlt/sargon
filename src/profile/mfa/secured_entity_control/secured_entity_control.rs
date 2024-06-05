use crate::prelude::*;

/// Advanced security control of an entity which has been "securified",
/// meaning an MFA security structure (`SecurityStructureOfFactorSources`)
/// which user has created has been applied to it.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct SecuredEntityControl {
    /// The address of the access controller which controls this entity.
    ///
    /// Looking up the public key (hashes) set in the key-value store at
    /// this address reveils the true factors (public keys) used to protect
    /// this entity. It will be the same as the ones in `security_structure`
    /// if we have not changed them locally, which we should not do unless
    /// we are sure the Ledger corresponds to the values in `security_structure`.
    pub access_controller_address: AccessControllerAddress,

    /// The believed-to-be-current security structure of FactorInstances which
    /// secures this entity.
    pub security_structure: SecurityStructureOfFactorInstances,
}
