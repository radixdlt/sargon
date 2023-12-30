const BIP32_HARDENED: u32 = 2147483648;

pub type HDPathValue = u32;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, uniffi::Record)]
pub struct HDPathComponent {
    pub value: HDPathValue,
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

    pub(crate) fn from_value(value: HDPathValue) -> Self {
        Self { value }
    }

    pub(crate) fn harden(value: HDPathValue) -> Self {
        assert!(value < BIP32_HARDENED);
        Self {
            value: value + BIP32_HARDENED,
        }
    }
}

impl ToString for HDPathComponent {
    fn to_string(&self) -> String {
        let h_or_empty = if self.is_hardened() { "H" } else { "" };
        format!("{}{}", self.index(), h_or_empty)
    }
}

#[cfg(test)]
mod tests {
    use super::HDPathComponent;

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
        assert_eq!(HDPathComponent::from_value(3).index(), 3)
    }

    #[test]
    fn non_hardened_value_is_not_hardened() {
        assert!(!HDPathComponent::from_value(3).is_hardened())
    }

    #[test]
    fn hardened_to_string() {
        assert_eq!(HDPathComponent::harden(5).to_string(), "5H")
    }

    #[test]
    fn non_hardened_to_string() {
        assert_eq!(HDPathComponent::from_value(7).to_string(), "7")
    }
}
