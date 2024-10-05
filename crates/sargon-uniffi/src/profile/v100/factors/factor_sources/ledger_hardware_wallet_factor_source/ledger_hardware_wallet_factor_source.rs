use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{hint} : {id}")]
pub struct LedgerHardwareWalletFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic,
    /// that is secured by the Ledger Hardware Wallet device.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a LedgerHardwareWalletFactorSource to help user disambiguate between it and another one.
    pub hint: LedgerHardwareWalletHint,
}

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

#[uniffi::export]
fn new_ledger_hardware_wallet_from_mnemonic_with_passphrase(
    mwp: MnemonicWithPassphrase,
    hint: LedgerHardwareWalletHint,
    common: FactorSourceCommon,
) -> LedgerHardwareWalletFactorSource {
    let id = FactorSourceIDFromHash::new_for_ledger(&mwp);
    LedgerHardwareWalletFactorSource::new(id, common, hint)
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

    #[test]
    fn test_new_ledger_hardware_wallet_from_mnemonic_with_passphrase() {
        assert_eq!(
            new_ledger_hardware_wallet_from_mnemonic_with_passphrase(
                MnemonicWithPassphrase::sample_ledger(),
                LedgerHardwareWalletHint::sample(),
                FactorSourceCommon::sample()
            )
            .factor_source_id(),
            SUT::sample().factor_source_id()
        );
    }
}
