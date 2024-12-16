use crate::prelude::*;
use sargon::{
    ProvisionalSecurifiedConfig as InternalProvisionalSecurifiedConfig,
    ProvisionalSecurifiedTransactionQueued as InternalProvisionalSecurifiedTransactionQueued,
};

/// A tuple of a `SecurityStructureOfFactorInstances` and a `TransactionIntentHash`
/// which represents a queued transaction to which is changing the security structure
/// if some entity. Since this provisional state is set on the entity itself, no
/// need to store the entity address here.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ProvisionalSecurifiedTransactionQueued {
    /// The FactorInstances we are changing to.
    pub factor_instances: SecurityStructureOfFactorInstances,

    /// The ID of the queued transaction which is changing the security structure
    /// to `factor_instances`.
    pub txid: TransactionIntentHash,
}

/// The different intermediary states of changing the security structure of an entity.
/// This type is put in an `Option` on either `UnsecuredEntityControl` or `SecurifiedEntityControl`,
/// and if `None` it means user has no provisionally changed security structure. If set, it contains
/// these different variants:
/// * `ShieldSelected` - User has selected which security shield to use for some entity,
/// * `FactorInstancesDerived` - Sargon has provided a `SecurityStructureOfFactorInstances` but
///     user has not made a transaction to apply it to the entity yet.
/// * `TransactionQueued` - User has signed and queued a transaction changing to `SecurityStructureOfFactorInstances`
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum ProvisionalSecurifiedConfig {
    /// User has selected which security shield to use for some entity,
    /// but no FactorInstances has been provided yet.
    ShieldSelected { value: SecurityStructureID },

    /// User has fully prepared a `SecurityStructureOfFactorInstances` but
    /// not made a transaction to apply it to the entity yet.
    FactorInstancesDerived {
        value: SecurityStructureOfFactorInstances,
    },

    /// User has made queued a transaction to apply a `SecurityStructureOfFactorInstances`
    /// but it has not been submitted (confirmed) yet.
    TransactionQueued {
        value: ProvisionalSecurifiedTransactionQueued,
    },
}
