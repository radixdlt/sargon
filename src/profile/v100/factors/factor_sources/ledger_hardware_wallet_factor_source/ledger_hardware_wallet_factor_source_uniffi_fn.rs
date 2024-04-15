use crate::prelude::*;

#[uniffi::export]
pub fn new_ledger_hardware_wallet_factor_source_sample(
) -> LedgerHardwareWalletFactorSource {
    LedgerHardwareWalletFactorSource::sample()
}

#[uniffi::export]
pub fn new_ledger_hardware_wallet_factor_source_sample_other(
) -> LedgerHardwareWalletFactorSource {
    LedgerHardwareWalletFactorSource::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LedgerHardwareWalletFactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_ledger_hardware_wallet_factor_source_sample(),
                new_ledger_hardware_wallet_factor_source_sample_other(),
                // duplicates should get removed
                new_ledger_hardware_wallet_factor_source_sample(),
                new_ledger_hardware_wallet_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }
}
