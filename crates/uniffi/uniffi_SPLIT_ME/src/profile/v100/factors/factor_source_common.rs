use crate::prelude::*;
use sargon::FactorSourceCommon as InternalFactorSourceCommon;

/// Common properties shared between FactorSources of different kinds, describing
/// its state, when added, and supported cryptographic parameters.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct FactorSourceCommon {
    /// Cryptographic parameters a certain FactorSource supports, e.g. Elliptic Curves.
    ///
    /// Since Radix Wallet App version 1.3.0, it is possible to add crypto
    /// parameters to a FactorSource, e.g. when a user with a DeviceFactorSource
    /// with babylon crypto parameters, lets call it `B`, with mnemonic `M` adds
    /// `M` again but as an "Olympia" factor source, then the olympia crypto
    /// parameters are added to `B`.
    pub crypto_parameters: FactorSourceCryptoParameters,

    /// When this factor source for originally added by the user.
    pub added_on: Timestamp,

    /// Date of last usage of this factor source
    ///
    /// This is the only mutable property, it is mutable
    /// since we will update it every time this FactorSource
    /// is used.
    pub last_used_on: Timestamp,

    /// Flags which describe a certain state a FactorSource might be in, e.g. `Main` (BDFS).
    pub flags: Vec<FactorSourceFlag>,
}

#[uniffi::export]
pub fn new_factor_source_common_sample() -> FactorSourceCommon {
    InternalFactorSourceCommon::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_common_sample_other() -> FactorSourceCommon {
    InternalFactorSourceCommon::sample_other().into()
}

#[uniffi::export]
pub fn new_factor_source_common_olympia() -> FactorSourceCommon {
    InternalFactorSourceCommon::new_olympia().into()
}

#[uniffi::export]
pub fn new_factor_source_common_babylon() -> FactorSourceCommon {
    InternalFactorSourceCommon::new_babylon().into()
}

#[uniffi::export]
pub fn new_factor_source_common_bdfs() -> FactorSourceCommon {
    InternalFactorSourceCommon::new_bdfs().into()
}

decl_conversion_tests_for!(FactorSourceCommon);
