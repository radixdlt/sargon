use crate::prelude::*;
use sargon::OnLedgerSettings as InternalOnLedgerSettings;

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

impl From<InternalOnLedgerSettings> for OnLedgerSettings {
    fn from(value: InternalOnLedgerSettings) -> Self {
        Self {
            third_party_deposits: value.third_party_deposits.into(),
        }
    }
}

impl Into<InternalOnLedgerSettings> for OnLedgerSettings {
    fn into(self) -> InternalOnLedgerSettings {
        InternalOnLedgerSettings {
            third_party_deposits: self.third_party_deposits.into(),
        }
    }
}

#[uniffi::export]
pub fn new_on_ledger_settings_sample() -> OnLedgerSettings {
    InternalOnLedgerSettings::sample().into()
}

#[uniffi::export]
pub fn new_on_ledger_settings_sample_other() -> OnLedgerSettings {
    InternalOnLedgerSettings::sample_other().into()
}

#[uniffi::export]
pub fn new_on_ledger_settings_default() -> OnLedgerSettings {
    InternalOnLedgerSettings::default().into()
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
