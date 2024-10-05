use crate::prelude::*;

/// Account settings that user has set on the account component
/// On-Ledger, that is set via a transaction mutating the state
/// on the network.
///
/// This settings include third-party deposits, controlling who
/// can send which assets to this account.
///
/// These settings SHOULD be kept in sync between local state
/// (in Profile) and On-Ledger.
#[derive(
    Default,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct OnLedgerSettings {
    /// Controls the ability of third-parties to deposit into this account
    pub third_party_deposits: ThirdPartyDeposits,
}

#[uniffi::export]
pub fn new_on_ledger_settings_sample() -> OnLedgerSettings {
    OnLedgerSettings::sample()
}

#[uniffi::export]
pub fn new_on_ledger_settings_sample_other() -> OnLedgerSettings {
    OnLedgerSettings::sample_other()
}

#[uniffi::export]
pub fn new_on_ledger_settings_default() -> OnLedgerSettings {
    OnLedgerSettings::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OnLedgerSettings;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_on_ledger_settings_sample(),
                new_on_ledger_settings_sample_other(),
                new_on_ledger_settings_default(),
                // duplicates should get removed
                new_on_ledger_settings_sample(),
                new_on_ledger_settings_sample_other(),
                new_on_ledger_settings_default(),
            ])
            .len(),
            3
        );
    }
}
