use crate::prelude::*;

impl GeneralRoleWithHierarchicalDeterministicFactorInstances {
    /// Securified { Single Threshold only }
    pub fn r2<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        Self::single_threshold(fi(FactorSourceIDFromHash::sample_at(0)))
    }

    /// Securified { Single Override only }
    pub fn r3<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        Self::single_override(fi(FactorSourceIDFromHash::sample_at(1)))
    }

    /// Securified { Threshold factors only #3 }
    pub fn r4<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::threshold_factors_only(
            [F::sample_at(0), F::sample_at(3), F::sample_at(5)].map(fi),
            2,
        )
        .unwrap()
    }

    /// Securified { Override factors only #2 }
    pub fn r5<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::override_only([F::sample_at(1), F::sample_at(4)].map(&fi))
            .unwrap()
    }

    /// Securified { Threshold #3 and Override factors #2  }
    pub fn r6<F>(fi: F) -> Self
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
        .unwrap()
    }

    /// Securified { Threshold only # 5/5 }
    pub fn r7<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::threshold_factors_only(
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
        .unwrap()
    }
    /// Securified { Threshold 1/1 and Override factors #1  }
    pub fn r8<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::new([F::sample_at(1)].map(&fi), 1, [F::sample_at(8)].map(&fi))
            .unwrap()
    }
}
