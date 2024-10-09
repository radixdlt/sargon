use crate::prelude::*;
use sargon::LedgerHardwareWalletModel as InternalLedgerHardwareWalletModel;

/// The model of a Ledger HQ hardware wallet NanoS, e.g.
/// *Ledger Nano S+*.
#[derive(
    Clone,
    
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum LedgerHardwareWalletModel {
    NanoS,
    NanoSPlus,
    NanoX,
}

impl From<InternalLedgerHardwareWalletModel> for LedgerHardwareWalletModel {
    fn from(value: InternalLedgerHardwareWalletModel) -> Self {
        match value {
            InternalLedgerHardwareWalletModel::NanoS => LedgerHardwareWalletModel::NanoS,
            InternalLedgerHardwareWalletModel::NanoSPlus => LedgerHardwareWalletModel::NanoSPlus,
            InternalLedgerHardwareWalletModel::NanoX => LedgerHardwareWalletModel::NanoX,
        }
    }
}

impl Into<InternalLedgerHardwareWalletModel> for LedgerHardwareWalletModel {
    fn into(self) -> InternalLedgerHardwareWalletModel {
        match self {
            LedgerHardwareWalletModel::NanoS => InternalLedgerHardwareWalletModel::NanoS,
            LedgerHardwareWalletModel::NanoSPlus => InternalLedgerHardwareWalletModel::NanoSPlus,
            LedgerHardwareWalletModel::NanoX => InternalLedgerHardwareWalletModel::NanoX,
        }
    }
}

#[uniffi::export]
pub fn ledger_hw_wallet_model_to_string(
    model: LedgerHardwareWalletModel,
) -> String {
    model.into_internal().to_string()
}

#[uniffi::export]
pub fn new_ledger_hw_wallet_model_from_string(
    string: String,
) -> Result<LedgerHardwareWalletModel> {
    InternalLedgerHardwareWalletModel::from_str(&string).map_result()
}

#[uniffi::export]
pub fn new_ledger_hw_wallet_model_sample() -> LedgerHardwareWalletModel {
    InternalLedgerHardwareWalletModel::sample().into()
}

#[uniffi::export]
pub fn new_ledger_hw_wallet_model_sample_other() -> LedgerHardwareWalletModel {
    InternalLedgerHardwareWalletModel::sample_other().into()
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
