use crate::prelude::*;

/// A discriminator of an `HDPathComponent`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum KeySpace {
    Unsecurified { is_hardened: bool },
    Securified,
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

    type Sut = KeySpace;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    pub fn is_securified() {
        assert!(Sut::Securified.is_securified());
        assert!(!Sut::Unsecurified { is_hardened: true }.is_securified());
        assert!(!Sut::Unsecurified { is_hardened: false }.is_securified());
    }

    #[test]
    pub fn is_unsecurified() {
        assert!(!Sut::Securified.is_unsecurified());
        assert!(Sut::Unsecurified { is_hardened: true }.is_unsecurified());
        assert!(Sut::Unsecurified { is_hardened: false }.is_unsecurified());
    }

    #[test]
    pub fn is_unsecurified_unhardened() {
        assert!(!Sut::Securified.is_unsecurified_unhardened());
        assert!(Sut::Unsecurified { is_hardened: false }
            .is_unsecurified_unhardened());
        assert!(!Sut::Unsecurified { is_hardened: true }
            .is_unsecurified_unhardened());
    }

    #[test]
    pub fn is_unsecurified_hardened() {
        assert!(!Sut::Securified.is_unsecurified_hardened());
        assert!(
            Sut::Unsecurified { is_hardened: true }.is_unsecurified_hardened()
        );
        assert!(!Sut::Unsecurified { is_hardened: false }
            .is_unsecurified_hardened());
    }
}
