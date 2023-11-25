use serde::{Deserialize, Serialize};

use crate::v100::factors::{
    factor_source_common::FactorSourceCommon, factor_source_id_from_hash::FactorSourceIDFromHash,
};

use super::device_factor_source_hint::DeviceFactorSourceHint;

/// A factor source representing the device that the Radix Wallet is running on
/// typically an iPhone or Android device. This is the initial factor source of
/// all new Accounts and Personas an users authenticate signing by authorizing
/// the client (Wallet App) to access a mnemonic stored in secure storage on
/// the device.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
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
