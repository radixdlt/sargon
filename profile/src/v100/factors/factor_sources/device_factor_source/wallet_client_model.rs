use std::fmt::Display;

use crate::HasPlaceholder;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum WalletClientModel {
    Iphone,
    Android,
    Unknown,
}
impl WalletClientModel {
    pub fn name(&self) -> String {
        match self {
            Self::Iphone => "iPhone",
            Self::Android => "Android",
            Self::Unknown => "Unknown",
        }
        .to_string()
    }
}
impl Display for WalletClientModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", &self.name())
    }
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
        assert_eq!(WalletClientModel::Iphone.name(), "iPhone");
        assert_eq!(WalletClientModel::Android.name(), "Android");
        assert_eq!(WalletClientModel::Unknown.name(), "Unknown");
    }
}
