use crate::prelude::*;

impl GeneralRoleWithHierarchicalDeterministicFactorInstances {
    /// Securified { Single Threshold only }
    pub(crate) fn m2<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        Self::single_threshold(fi(FactorSourceIDFromHash::sample_at(0)))
    }

    /// Securified { Single Override only }
    pub(crate) fn m3<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        Self::single_override(fi(FactorSourceIDFromHash::sample_at(1)))
    }

    /// Securified { Threshold factors only #3 }
    pub(crate) fn m4<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::threshold_only(
            [F::sample_at(0), F::sample_at(3), F::sample_at(5)].map(fi),
            2,
        )
    }

    /// Securified { Override factors only #2 }
    pub(crate) fn m5<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::override_only([F::sample_at(1), F::sample_at(4)].map(&fi))
    }

    /// Securified { Threshold #3 and Override factors #2  }
    pub(crate) fn m6<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::new(
            [F::sample_at(0), F::sample_at(3), F::sample_at(5)].map(&fi),
            2,
            [F::sample_at(1), F::sample_at(4)].map(&fi),
        )
    }

    /// Securified { Threshold only # 5/5 }
    pub(crate) fn m7<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::threshold_only(
            [
                F::sample_at(2),
                F::sample_at(6),
                F::sample_at(7),
                F::sample_at(8),
                F::sample_at(9),
            ]
            .map(&fi),
            5,
        )
    }
    /// Securified { Threshold 1/1 and Override factors #1  }
    pub(crate) fn m8<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::new([F::sample_at(1)].map(&fi), 1, [F::sample_at(8)].map(&fi))
    }
}
