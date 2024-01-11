use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display, uniffi::Enum)]
pub enum WalletClientModel {
    Iphone,
    Android,
    Unknown,
}

impl HasPlaceholder for WalletClientModel {
    fn placeholder() -> Self {
        WalletClientModel::Iphone
    }

    fn placeholder_other() -> Self {
        WalletClientModel::Android
    }
}

#[cfg(test)]
mod tests {
    use crate::HasPlaceholder;

    use super::WalletClientModel;
    #[test]
    fn equality() {
        assert_eq!(
            WalletClientModel::placeholder(),
            WalletClientModel::placeholder()
        );
        assert_eq!(
            WalletClientModel::placeholder_other(),
            WalletClientModel::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            WalletClientModel::placeholder(),
            WalletClientModel::placeholder_other()
        );
    }

    #[test]
    fn name() {
        assert_eq!(WalletClientModel::Iphone.name(), "Ihhone");
        assert_eq!(WalletClientModel::Android.name(), "Android");
        assert_eq!(WalletClientModel::Unknown.name(), "Unknown");
    }
}
