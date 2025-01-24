use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TransactionManifestApplySecurityShieldAnyInput {
    pub security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
    pub apply_shield_manifest_kind:
        Option<TransactionManifestApplySecurityShieldKind>,
}
impl TransactionManifestApplySecurityShieldAnyInput {
    fn new(
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        apply_shield_manifest_kind: impl Into<
            Option<TransactionManifestApplySecurityShieldKind>,
        >,
    ) -> Self {
        Self {
            security_structure_of_factor_instances,
            apply_shield_manifest_kind: apply_shield_manifest_kind.into(),
        }
    }
    pub fn for_securified(
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        apply_shield_manifest_kind: TransactionManifestApplySecurityShieldKind,
    ) -> Self {
        Self::new(
            security_structure_of_factor_instances,
            apply_shield_manifest_kind,
        )
    }
    pub fn as_securified(
        &self,
    ) -> Result<TransactionManifestApplySecurityShieldSecurifiedInput> {
        let apply_shield_manifest_kind = self
            .apply_shield_manifest_kind
            .clone()
            .ok_or(CommonError::Unknown)?; // TODO: replace with proper error
        Ok(TransactionManifestApplySecurityShieldSecurifiedInput {
            security_structure_of_factor_instances: self
                .security_structure_of_factor_instances
                .clone(),
            apply_shield_manifest_kind,
        })
    }
    pub fn for_unsecurified(
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
    ) -> Self {
        Self::new(security_structure_of_factor_instances, None)
    }

    pub fn as_unsecurified(
        &self,
    ) -> Result<TransactionManifestApplySecurityShieldUnsecurifiedInput> {
        Ok(TransactionManifestApplySecurityShieldUnsecurifiedInput {
            security_structure_of_factor_instances: self
                .security_structure_of_factor_instances
                .clone(),
        })
    }
}
