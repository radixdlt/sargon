use crate::prelude::*;

/// For testing purposes
/// We do not support Custodian FactorSource yet, but I wanted to write the
/// heuristics being future proof, so instead of actually assigning any Custodian
/// (which does not exist), we record the calls to assign Custodian using this
/// small struct, so that we can assert correctness of the heuristics.
///
/// When we do add Custodian FactorSource, we can remove this struct and just
/// assert the actual assignment of Custodian factors...
#[derive(Default)]
pub struct AutoBuildOutcomeForTesting {
    pub(super) calls_to_assign_unsupported_factor:
        Vec<CallsToAssignUnsupportedFactor>,
}

/// For testing purposes
/// We do not support Custodian FactorSource yet, but I wanted to write the
/// heuristics being future proof, so instead of actually assigning any Custodian
/// (which does not exist), we record the calls to assign Custodian using this
/// small struct, so that we can assert correctness of the heuristics.
///
/// When we do add Custodian FactorSource, we can remove this struct and just
/// assert the actual assignment of Custodian factors...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CallsToAssignUnsupportedFactor {
    /// The role.
    pub(super) role: RoleKind,

    /// FactorSelector
    pub(super) unsupported: FactorSelector,

    /// The number of factors in the role when `assign_custodian_and_hardware_to_role_if_less_than_limit_before_each_assignment` was called
    pub(super) number_of_factors_for_role: u8,

    /// The value of `limit` parameter passed to `assign_custodian_and_hardware_to_role_if_less_than_limit_before_each_assignment`
    pub(super) limit: u8,
}
