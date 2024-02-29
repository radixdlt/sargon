use crate::prelude::*;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::Display, uniffi::Enum,
)]
pub enum WalletClientModel {
    #[display("iPhone")]
    Iphone,
    Android,
    Unknown,
}

impl HasSampleValues for WalletClientModel {
    fn sample() -> Self {
        WalletClientModel::Iphone
    }

    fn sample_other() -> Self {
        WalletClientModel::Android
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(WalletClientModel::sample(), WalletClientModel::sample());
        assert_eq!(
            WalletClientModel::sample_other(),
            WalletClientModel::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            WalletClientModel::sample(),
            WalletClientModel::sample_other()
        );
    }

    #[test]
    fn name() {
        assert_eq!(format!("{}", WalletClientModel::Iphone), "iPhone");
        assert_eq!(format!("{}", WalletClientModel::Android), "Android");
        assert_eq!(format!("{}", WalletClientModel::Unknown), "Unknown");
    }
}
