const BIP32_HARDENED: u32 = 2147483648;

pub type HDPathValue = u32;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.bip32_string())]
#[debug("{}", self.bip32_string())]
pub struct HDPathComponent {
    pub value: HDPathValue,
}

impl From<HDPathValue> for HDPathComponent {
    fn from(value: HDPathValue) -> Self {
        Self { value }
    }
}

impl HDPathComponent {
    pub(crate) fn index(&self) -> HDPathValue {
        if self.is_hardened() {
            self.value - BIP32_HARDENED
        } else {
            self.value
        }
    }

    pub(crate) fn is_hardened(&self) -> bool {
        self.value >= BIP32_HARDENED
    }

    pub(crate) fn non_hardened(value: HDPathValue) -> Self {
        assert!(
            value < BIP32_HARDENED,
            "Passed value was hardened, expected it to not be."
        );
        Self { value }
    }

    pub(crate) fn harden(value: HDPathValue) -> Self {
        assert!(
            value < BIP32_HARDENED,
            "Passed value was already hardened, expected it to not be."
        );
        Self {
            value: value + BIP32_HARDENED,
        }
    }
}

impl HDPathComponent {
    fn bip32_string(&self) -> String {
        let h_or_empty = if self.is_hardened() { "H" } else { "" };
        format!("{}{}", self.index(), h_or_empty)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::BIP32_HARDENED;

    #[test]
    fn harden() {
        assert!(HDPathComponent::harden(0).is_hardened())
    }

    #[test]
    fn hardened_value() {
        assert_eq!(HDPathComponent::harden(3).index(), 3)
    }

    #[test]
    fn non_hardened_value() {
        assert_eq!(HDPathComponent::non_hardened(3).index(), 3)
    }

    #[test]
    fn non_hardened_value_is_not_hardened() {
        assert!(!HDPathComponent::non_hardened(3).is_hardened())
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", HDPathComponent::harden(5)), "5H");
        assert_eq!(format!("{}", HDPathComponent::non_hardened(5)), "5");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", HDPathComponent::harden(5)), "5H");
        assert_eq!(format!("{:?}", HDPathComponent::non_hardened(5)), "5");
    }

    #[should_panic(
        expected = "Passed value was hardened, expected it to not be."
    )]
    #[test]
    fn non_hardened_already_hardened_panics() {
        _ = HDPathComponent::non_hardened(7 + BIP32_HARDENED);
    }

    #[should_panic(
        expected = "Passed value was already hardened, expected it to not be."
    )]
    #[test]
    fn harden_already_hardened_panics() {
        _ = HDPathComponent::harden(7 + BIP32_HARDENED);
    }

    #[test]
    fn from_hdpath_value() {
        assert_eq!(
            <HDPathComponent as From<HDPathValue>>::from(5).to_string(),
            "5"
        );
        assert_eq!(
            <HDPathComponent as From<HDPathValue>>::from(2147483650)
                .to_string(),
            "2H"
        );
    }
}
