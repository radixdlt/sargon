use crate::prelude::*;

/// A discriminator of an `HDPathComponent`.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Debug,
    derive_more::Display,
)]
pub enum KeySpace {
    #[debug("{}", if *is_hardened { UnsecurifiedHardened::NON_CANONICAL_SUFFIX } else { "" })]
    #[display("{}", if *is_hardened { UnsecurifiedHardened::CANONICAL_SUFFIX } else { "" })]
    Unsecurified { is_hardened: bool },

    #[debug("{}", SecurifiedU30::NON_CANONICAL_SUFFIX)]
    #[display("{}", SecurifiedU30::CANONICAL_SUFFIX)]
    Securified,
}

impl FromStr for KeySpace {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        if s == Self::Securified.to_string() {
            Ok(Self::Securified)
        } else if s == (Self::Unsecurified { is_hardened: false }).to_string() {
            Ok(Self::Unsecurified { is_hardened: false })
        } else if s == (Self::Unsecurified { is_hardened: true }).to_string() {
            Ok(Self::Unsecurified { is_hardened: true })
        } else {
            Err(CommonError::Unknown)
        }
    }
}

impl KeySpace {
    pub fn is_securified(&self) -> bool {
        matches!(self, Self::Securified)
    }
    pub fn is_unsecurified(&self) -> bool {
        matches!(self, Self::Unsecurified { is_hardened: _ },)
    }
    pub fn is_unsecurified_hardened(&self) -> bool {
        matches!(self, Self::Unsecurified { is_hardened: true },)
    }
    pub fn is_unsecurified_unhardened(&self) -> bool {
        matches!(self, Self::Unsecurified { is_hardened: false },)
    }
}

impl HasSampleValues for KeySpace {
    fn sample() -> Self {
        KeySpace::Securified
    }

    fn sample_other() -> Self {
        KeySpace::Unsecurified { is_hardened: false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = KeySpace;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    pub fn is_securified() {
        assert!(SUT::Securified.is_securified());
        assert!(!SUT::Unsecurified { is_hardened: true }.is_securified());
        assert!(!SUT::Unsecurified { is_hardened: false }.is_securified());
    }

    #[test]
    pub fn is_unsecurified() {
        assert!(!SUT::Securified.is_unsecurified());
        assert!(SUT::Unsecurified { is_hardened: true }.is_unsecurified());
        assert!(SUT::Unsecurified { is_hardened: false }.is_unsecurified());
    }

    #[test]
    pub fn is_unsecurified_unhardened() {
        assert!(!SUT::Securified.is_unsecurified_unhardened());
        assert!(SUT::Unsecurified { is_hardened: false }
            .is_unsecurified_unhardened());
        assert!(!SUT::Unsecurified { is_hardened: true }
            .is_unsecurified_unhardened());
    }

    #[test]
    pub fn is_unsecurified_hardened() {
        assert!(!SUT::Securified.is_unsecurified_hardened());
        assert!(
            SUT::Unsecurified { is_hardened: true }.is_unsecurified_hardened()
        );
        assert!(!SUT::Unsecurified { is_hardened: false }
            .is_unsecurified_hardened());
    }
}
