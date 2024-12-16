use crate::prelude::*;
use sargon::FactorSourceIntegrity as InternalFactorSourceIntegrity;

/// An enum representing the integrity of a factor source.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum FactorSourceIntegrity {
    Device(DeviceFactorSourceIntegrity),

    Ledger(LedgerHardwareWalletFactorSource),
}
