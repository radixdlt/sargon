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

impl From<PrivateHierarchicalDeterministicFactorInstance> for HierarchicalDeterministicPrivateKey {
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

impl HasPlaceholder for PrivateHierarchicalDeterministicFactorInstance {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new(
            HierarchicalDeterministicPrivateKey::placeholder(),
            FactorSourceID::placeholder(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::new(
            HierarchicalDeterministicPrivateKey::placeholder_other(),
            FactorSourceID::placeholder_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::HasPlaceholder;
    use crate::{Derivation, HierarchicalDeterministicPrivateKey};

    use crate::v100::factors::factor_instance::factor_instance::FactorInstance;

    use super::PrivateHierarchicalDeterministicFactorInstance;

    #[test]
    fn equality() {
        assert_eq!(
            PrivateHierarchicalDeterministicFactorInstance::placeholder(),
            PrivateHierarchicalDeterministicFactorInstance::placeholder()
        );
        assert_eq!(
            PrivateHierarchicalDeterministicFactorInstance::placeholder_other(),
            PrivateHierarchicalDeterministicFactorInstance::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            PrivateHierarchicalDeterministicFactorInstance::placeholder(),
            PrivateHierarchicalDeterministicFactorInstance::placeholder_other()
        );
    }

    #[test]
    fn new() {
        let sut = PrivateHierarchicalDeterministicFactorInstance::placeholder();
        assert_eq!(
            sut.private_key.derivation_path.to_string(),
            "m/44H/1022H/1H/525H/1460H/0H"
        );
        assert_eq!(
            sut.private_key.private_key.to_hex(),
            "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003"
        );
    }

    #[test]
    fn into_hierarchical_deterministic_private_key() {
        let sut = PrivateHierarchicalDeterministicFactorInstance::placeholder();
        let key: HierarchicalDeterministicPrivateKey = sut.into();
        assert_eq!(
            key.public_key(),
            HierarchicalDeterministicPrivateKey::placeholder().public_key()
        );
    }

    #[test]
    fn into_factor_instance() {
        let sut = PrivateHierarchicalDeterministicFactorInstance::placeholder();
        let key: FactorInstance = sut.into();
        assert_eq!(
            key.factor_source_id,
            PrivateHierarchicalDeterministicFactorInstance::placeholder().factor_source_id
        );
    }
}
