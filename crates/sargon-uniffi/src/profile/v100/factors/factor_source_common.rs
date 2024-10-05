use crate::prelude::*;

/// Flags which describe a certain state a FactorSource might be in, e.g. `Main` (BDFS).
pub type FactorSourceFlags = IdentifiedVecOf<FactorSourceFlag>;
impl Identifiable for FactorSourceFlag {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

/// Common properties shared between FactorSources of different kinds, describing
/// its state, when added, and supported cryptographic parameters.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, uniffi::Record,
)]
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
    pub flags: FactorSourceFlags,
}

#[uniffi::export]
pub fn new_factor_source_common_sample() -> FactorSourceCommon {
    FactorSourceCommon::sample()
}

#[uniffi::export]
pub fn new_factor_source_common_sample_other() -> FactorSourceCommon {
    FactorSourceCommon::sample_other()
}

#[uniffi::export]
pub fn new_factor_source_common_olympia() -> FactorSourceCommon {
    FactorSourceCommon::new_olympia()
}

#[uniffi::export]
pub fn new_factor_source_common_babylon() -> FactorSourceCommon {
    FactorSourceCommon::new_babylon()
}

#[uniffi::export]
pub fn new_factor_source_common_bdfs(is_main: bool) -> FactorSourceCommon {
    FactorSourceCommon::new_bdfs(is_main)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceCommon;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_common_sample(),
                new_factor_source_common_sample_other(),
                // duplicates should get removed
                new_factor_source_common_sample(),
                new_factor_source_common_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn inequality_of_presets() {
        assert_ne!(
            new_factor_source_common_babylon(),
            new_factor_source_common_olympia()
        );
        assert_ne!(
            new_factor_source_common_bdfs(false),
            new_factor_source_common_olympia()
        );
    }
}
