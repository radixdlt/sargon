use crate::prelude::*;
use sargon::{
    ProvisionalSecurifiedConfig as InternalProvisionalSecurifiedConfig,
    ProvisionalSecurifiedTransactionQueued as InternalProvisionalSecurifiedTransactionQueued,
};

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ProvisionalSecurifiedTransactionQueued {
    pub factor_instances: SecurityStructureOfFactorInstances,
    pub txid: TransactionIntentHash,
}

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
