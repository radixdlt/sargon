use crate::prelude::*;

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
