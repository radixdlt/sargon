use crate::prelude::*;

/// An ephemeral (never persisted) HD FactorInstance which contains
/// the private key, with the ID of its creating FactorSource.
#[derive(Debug, PartialEq, Eq)]
pub struct PrivateHierarchicalDeterministicFactorInstance {
    /// The HD Private Key.
    pub private_key: HierarchicalDeterministicPrivateKey,

    /// The ID of the FactorSource creating the `PrivateKey`.
    pub factor_source_id: FactorSourceID,
}

impl From<PrivateHierarchicalDeterministicFactorInstance>
    for HierarchicalDeterministicPrivateKey
{
    fn from(value: PrivateHierarchicalDeterministicFactorInstance) -> Self {
        value.private_key
    }
}

impl From<PrivateHierarchicalDeterministicFactorInstance> for FactorInstance {
    fn from(value: PrivateHierarchicalDeterministicFactorInstance) -> Self {
        FactorInstance::with_hierarchical_deterministic_public_key(
            value.factor_source_id,
            value.private_key.public_key(),
        )
    }
}

impl PrivateHierarchicalDeterministicFactorInstance {
    /// Instantiates a new `PrivateHierarchicalDeterministicFactorInstance` from the HD PrivateKey
    /// with a FactorSourceID.
    pub fn new(
        private_key: HierarchicalDeterministicPrivateKey,
        factor_source_id: FactorSourceID,
    ) -> Self {
        Self {
            private_key,
            factor_source_id,
        }
    }
}

impl HasSampleValues for PrivateHierarchicalDeterministicFactorInstance {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(
            HierarchicalDeterministicPrivateKey::sample(),
            FactorSourceID::sample(),
        )
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::new(
            HierarchicalDeterministicPrivateKey::sample_other(),
            FactorSourceID::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(
            PrivateHierarchicalDeterministicFactorInstance::sample(),
            PrivateHierarchicalDeterministicFactorInstance::sample()
        );
        assert_eq!(
            PrivateHierarchicalDeterministicFactorInstance::sample_other(),
            PrivateHierarchicalDeterministicFactorInstance::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            PrivateHierarchicalDeterministicFactorInstance::sample(),
            PrivateHierarchicalDeterministicFactorInstance::sample_other()
        );
    }

    #[test]
    fn new() {
        let sut = PrivateHierarchicalDeterministicFactorInstance::sample();
        assert_eq!(
            sut.private_key.derivation_path.to_string(),
            "m/44H/1022H/1H/525H/1460H/0H"
        );
        assert_eq!(
            sut.private_key.private_key.to_hex(),
            "88ec4649da764965f862510dbe53d551a3fc2da49e1ef1f383d9d17006773bee"
        );
    }

    #[test]
    fn into_hierarchical_deterministic_private_key() {
        let sut = PrivateHierarchicalDeterministicFactorInstance::sample();
        let key: HierarchicalDeterministicPrivateKey = sut.into();
        assert_eq!(
            key.public_key(),
            HierarchicalDeterministicPrivateKey::sample().public_key()
        );
    }

    #[test]
    fn into_factor_instance() {
        let sut = PrivateHierarchicalDeterministicFactorInstance::sample();
        let key: FactorInstance = sut.into();
        assert_eq!(
            key.factor_source_id,
            PrivateHierarchicalDeterministicFactorInstance::sample()
                .factor_source_id
        );
    }
}
