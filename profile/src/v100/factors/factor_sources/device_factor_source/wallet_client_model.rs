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
    use crate::prelude::*;
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
        assert_eq!(format!("{}", WalletClientModel::Iphone), "Iphone");
        assert_eq!(format!("{}", WalletClientModel::Android), "Android");
        assert_eq!(format!("{}", WalletClientModel::Unknown), "Unknown");
    }
}
