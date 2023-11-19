const BIP32_HARDENED: u32 = 2147483648;

pub type HDPathValue = u32;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HDPathComponent(HDPathValue);

impl HDPathComponent {
    pub(crate) fn value(&self) -> HDPathValue {
        if self.is_hardened() {
            self.0 - BIP32_HARDENED
        } else {
            self.0
        }
    }

    pub(crate) fn is_hardened(&self) -> bool {
        self.0 >= BIP32_HARDENED
    }

    pub(crate) fn from_value(value: HDPathValue) -> Self {
        Self(value)
    }

    pub(crate) fn harden(value: HDPathValue) -> Self {
        assert!(value < BIP32_HARDENED);
        Self(value + BIP32_HARDENED)
    }
}

impl ToString for HDPathComponent {
    fn to_string(&self) -> String {
        let h_or_empty = if self.is_hardened() { "H" } else { "" };
        format!("{}{}", self.value(), h_or_empty)
    }
}
