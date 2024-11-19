use crate::prelude::*;

impl GeneralRoleWithHierarchicalDeterministicFactorInstances {
    /// Primary Role
    /// Securified { Single Threshold only }
    pub(crate) fn r2<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        Self::single_threshold(
            RoleKind::Primary,
            fi(FactorSourceIDFromHash::sample_at(0)),
        )
    }

    /// Primary Role
    /// Securified { Single Override only }
    pub(crate) fn r3<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        Self::single_override(
            RoleKind::Primary,
            fi(FactorSourceIDFromHash::sample_at(1)),
        )
    }

    /// Primary Role
    /// Securified { Threshold factors only #3 }
    pub(crate) fn r4<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::with_factors_and_role(
            RoleKind::Primary,
            [F::sample_at(0), F::sample_at(3), F::sample_at(5)].map(fi),
            2,
            [],
            false
        )
        .unwrap()
    }

    /// Primary Role
    /// Securified { Override factors only #2 }
    pub(crate) fn r5<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::with_factors_and_role(
            RoleKind::Primary,
            [],
            0,
            [F::sample_at(1), F::sample_at(4)].map(&fi),
            false
        )
        .unwrap()
    }

    /// Primary Role
    /// Securified { Threshold #3 and Override factors #2  }
    pub(crate) fn r6<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::with_factors_and_role(
            RoleKind::Primary,
            [F::sample_at(0), F::sample_at(3), F::sample_at(5)].map(&fi),
            2,
            [F::sample_at(1), F::sample_at(4)].map(&fi),
            false
        )
        .unwrap()
    }

    /// Primary Role
    /// Securified { Threshold only # 5/5 }
    pub(crate) fn r7<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::with_factors_and_role(
            RoleKind::Primary,
            [
                F::sample_at(2),
                F::sample_at(6),
                F::sample_at(7),
                F::sample_at(8),
                F::sample_at(9),
            ]
            .map(&fi),
            5,
            [],
            false
        )
        .unwrap()
    }

    /// Primary Role
    /// Securified { Threshold 1/1 and Override factors #1  }
    pub(crate) fn r8<F>(fi: F) -> Self
    where
        F: Fn(
            FactorSourceIDFromHash,
        ) -> HierarchicalDeterministicFactorInstance,
    {
        type F = FactorSourceIDFromHash;
        Self::with_factors_and_role(
            RoleKind::Primary,
            [F::sample_at(1)].map(&fi),
            1,
            [F::sample_at(8)].map(&fi),
            false
        )
        .unwrap()
    }
}
