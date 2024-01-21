use crate::prelude::*;

/// The model of a Ledger HQ hardware wallet NanoS, e.g.
/// *Ledger Nano S+*.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum LedgerHardwareWalletModel {
    NanoS,

    #[serde(rename = "nanoS+")]
    NanoSPlus,
    NanoX,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            LedgerHardwareWalletModel::NanoS,
            LedgerHardwareWalletModel::NanoS
        );
        assert_eq!(
            LedgerHardwareWalletModel::NanoX,
            LedgerHardwareWalletModel::NanoX
        );
    }
    #[test]
    fn inequality() {
        assert_ne!(
            LedgerHardwareWalletModel::NanoS,
            LedgerHardwareWalletModel::NanoX
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter(
                [
                    LedgerHardwareWalletModel::NanoS,
                    LedgerHardwareWalletModel::NanoS
                ]
                .into_iter()
            )
            .len(),
            1
        );
    }

    #[test]
    fn ord() {
        assert!(
            LedgerHardwareWalletModel::NanoS < LedgerHardwareWalletModel::NanoX
        );
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(
            &LedgerHardwareWalletModel::NanoS,
            json!("nanoS"),
        );
        assert_json_value_eq_after_roundtrip(
            &LedgerHardwareWalletModel::NanoSPlus,
            json!("nanoS+"),
        );
        assert_json_value_eq_after_roundtrip(
            &LedgerHardwareWalletModel::NanoX,
            json!("nanoX"),
        );
    }
}
