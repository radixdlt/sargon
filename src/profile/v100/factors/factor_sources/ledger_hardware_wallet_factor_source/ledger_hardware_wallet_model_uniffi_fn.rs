use crate::prelude::*;

#[uniffi::export]
pub fn ledger_hw_wallet_model_to_string(
    model: LedgerHardwareWalletModel,
) -> String {
    model.to_string()
}

#[uniffi::export]
pub fn new_ledger_hw_wallet_model_from_string(
    string: String,
) -> Result<LedgerHardwareWalletModel> {
    LedgerHardwareWalletModel::from_str(&string)
}

#[uniffi::export]
pub fn new_ledger_hw_wallet_model_sample() -> LedgerHardwareWalletModel {
    LedgerHardwareWalletModel::sample()
}

#[uniffi::export]
pub fn new_ledger_hw_wallet_model_sample_other() -> LedgerHardwareWalletModel {
    LedgerHardwareWalletModel::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LedgerHardwareWalletModel;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_ledger_hw_wallet_model_sample(),
                new_ledger_hw_wallet_model_sample_other(),
                // duplicates should get removed
                new_ledger_hw_wallet_model_sample(),
                new_ledger_hw_wallet_model_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn string_roundtrip() {
        let sut = SUT::sample();
        let str = ledger_hw_wallet_model_to_string(sut);
        let from_str = new_ledger_hw_wallet_model_from_string(str).unwrap();
        assert_eq!(sut, from_str);
    }
}
