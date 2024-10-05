use crate::prelude::*;
use sargon::DeviceFactorSource as InternalDeviceFactorSource;

/// A factor source representing the host device which SargonOS runs on, typically
/// an iPhone or Android device.
///
/// This is the initial factor source of
/// all new Accounts and Personas. Users authenticate signing by authorizing
/// the host to access a mnemonic stored in secure storage on
/// the device.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{hint} {id}")]
pub struct DeviceFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a DeviceFactorSource to help user disambiguate between it and another one.
    pub hint: DeviceFactorSourceHint,
}

impl From<InternalDeviceFactorSource> for DeviceFactorSource {
    fn from(value: InternalDeviceFactorSource) -> Self {
        unimplemented!()
    }
}

impl Into<InternalDeviceFactorSource> for DeviceFactorSource {
    fn into(self) -> InternalDeviceFactorSource {
        unimplemented!()
    }
}