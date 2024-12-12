use crate::prelude::*;
use sargon::FactorSourceAccessibility as InternalFactorSourceAccessibility;

/// An enum representing the accessibility of a factor source.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum FactorSourceAccessibility {
    Device(DeviceFactorSourceAccessibility),

    Ledger(LedgerHardwareWalletFactorSource),
}
