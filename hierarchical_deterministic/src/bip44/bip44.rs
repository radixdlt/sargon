use crate::bip32::HDPathComponent;

impl HDPathComponent {
    pub fn bip44_purpose() -> Self {
        Self::harden(44)
    }

    /// The `cointype` of Radix DLT: `1022H`, as defined in SLIP44, see
    /// merged PR: https://github.com/satoshilabs/slips/pull/1137
    pub fn bip44_cointype() -> Self {
        Self::harden(1022)
    }
}
