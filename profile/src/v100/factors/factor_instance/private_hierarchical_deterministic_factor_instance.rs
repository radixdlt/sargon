use derive_getters::Getters;
use hierarchical_deterministic::derivation::hierarchical_deterministic_private_key::HierarchicalDeterministicPrivateKey;

use crate::v100::factors::factor_source_id::FactorSourceID;

use super::factor_instance::FactorInstance;

/// An ephemeral (never persisted) HD FactorInstance which contains
/// the private key, with the ID of its creating FactorSource.
#[derive(Getters)]
pub struct PrivateHierarchicalDeterministicFactorInstance {
    /// The HD Private Key.
    private_key: HierarchicalDeterministicPrivateKey,
    /// The ID of the FactorSource creating the `PrivateKey`.
    factor_source_id: FactorSourceID,
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

#[cfg(any(test, feature = "placeholder"))]
impl PrivateHierarchicalDeterministicFactorInstance {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::new(
            HierarchicalDeterministicPrivateKey::placeholder(),
            FactorSourceID::placeholder(),
        )
    }
}

#[cfg(test)]
mod tests {
    use hierarchical_deterministic::derivation::{
        derivation::Derivation,
        hierarchical_deterministic_private_key::HierarchicalDeterministicPrivateKey,
    };

    use crate::v100::factors::factor_instance::factor_instance::FactorInstance;

    use super::PrivateHierarchicalDeterministicFactorInstance;

    #[test]
    fn new() {
        let sut = PrivateHierarchicalDeterministicFactorInstance::placeholder();
        assert_eq!(
            sut.private_key().derivation_path().to_string(),
            "m/44H/1022H/1H/525H/1460H/0H"
        );
        assert_eq!(
            sut.private_key().private_key().to_hex(),
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
            key.factor_source_id(),
            &PrivateHierarchicalDeterministicFactorInstance::placeholder().factor_source_id
        );
    }
}
