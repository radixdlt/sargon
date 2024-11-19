use crate::prelude::*;

/// Just an alias for when `HDFactorInstanceTransactionSigning` is used to create a new Account.
pub type HDFactorInstanceIdentityCreation =
    HDFactorInstanceTransactionSigning<IdentityPath>;

impl HasSampleValues for HDFactorInstanceIdentityCreation {
    fn sample() -> Self {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            IdentityPath::sample().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        Self::new(hd_fi).unwrap()
    }

    fn sample_other() -> Self {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            IdentityPath::sample_other().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        Self::new(hd_fi).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HDFactorInstanceIdentityCreation;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}