use crate::prelude::*;
/// A virtual hierarchical deterministic `FactorInstance`
#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct HierarchicalDeterministicFactorInstance {
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: HierarchicalDeterministicPublicKey,
}

#[uniffi::export]
pub fn new_hierarchical_deterministic_factor_instance_sample(
) -> HierarchicalDeterministicFactorInstance {
    HierarchicalDeterministicFactorInstance::sample()
}

#[uniffi::export]
pub fn new_hierarchical_deterministic_factor_instance_sample_other(
) -> HierarchicalDeterministicFactorInstance {
    HierarchicalDeterministicFactorInstance::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HierarchicalDeterministicFactorInstance;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_hierarchical_deterministic_factor_instance_sample(),
                new_hierarchical_deterministic_factor_instance_sample_other(),
                // duplicates should get removed
                new_hierarchical_deterministic_factor_instance_sample(),
                new_hierarchical_deterministic_factor_instance_sample_other(),
            ])
            .len(),
            2
        );
    }
}
