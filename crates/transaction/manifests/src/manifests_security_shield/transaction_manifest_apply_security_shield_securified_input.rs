use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TransactionManifestApplySecurityShieldSecurifiedInput {
    pub security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
    pub apply_shield_manifest_kind: TransactionManifestApplySecurityShieldKind,
}

impl TransactionManifestApplySecurityShieldSecurifiedInput {
    pub fn new(
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        apply_shield_manifest_kind: TransactionManifestApplySecurityShieldKind,
    ) -> Self {
        Self {
            security_structure_of_factor_instances,
            apply_shield_manifest_kind,
        }
    }
}
