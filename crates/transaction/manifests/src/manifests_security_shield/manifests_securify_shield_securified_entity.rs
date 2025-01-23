#![allow(dead_code)]
use profile_supporting_types::AnySecurifiedEntity;

use crate::prelude::*;

pub trait TransactionManifestSecurifySecurifiedEntity:
    Sized + TransactionManifestSetRolaKey
{
    fn apply_security_shield_for_securified_entity(
        securified_entity: AnySecurifiedEntity,
        input: TransactionManifestApplySecurityShieldSecurifiedInput,
    ) -> Result<Self>;
}

impl TransactionManifestSecurifySecurifiedEntity for TransactionManifest {
    fn apply_security_shield_for_securified_entity(
        _securified_entity: AnySecurifiedEntity,
        _input: TransactionManifestApplySecurityShieldSecurifiedInput,
    ) -> Result<Self> {
        // let TransactionManifestApplySecurityShieldSecurifiedInput {
        //     security_structure_of_factor_instances,
        //     apply_shield_manifest_kind,
        // } = input.clone();

        // let builder = ManifestBuilder::new();
        // let access_controller_address = securified_entity
        //     .securified_entity_control
        //     .access_controller_address;

        // let should_start_recovery =
        //     apply_shield_manifest_kind.should_confirm_recovery();
        // if should_start_recovery {
        //     //  builder.call_method(access_controller_address, , arguments)
        //     todo!();
        // }
        todo!("implement me")
    }
}

impl TransactionManifestApplySecurityShieldKindSelector {
    fn should_confirm_recovery_with_explicit(&self) -> bool {
        match self {
            Self::PrimaryAndRecoveryWithExplicitConfirmation => true,
            Self::PrimaryAndRecoveryWithTimedAutoConfirm => false,
            Self::PrimaryAndExplicitConfirmation => true,
            Self::PrimaryWithTimedAutoConfirm => false,
            Self::RecoveryAndExplicitConfirmation => true,
            Self::RecoveryWithTimedAutoConfirm => false,
        }
    }
    fn should_confirm_recovery_with_time(&self) -> bool {
        match self {
            Self::PrimaryAndRecoveryWithExplicitConfirmation => false,
            Self::PrimaryAndRecoveryWithTimedAutoConfirm => true,
            Self::PrimaryAndExplicitConfirmation => false,
            Self::PrimaryWithTimedAutoConfirm => true,
            Self::RecoveryAndExplicitConfirmation => false,
            Self::RecoveryWithTimedAutoConfirm => true,
        }
    }
    fn should_confirm_recovery(&self) -> bool {
        self.should_confirm_recovery_with_explicit()
            || self.should_confirm_recovery_with_time()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionManifestApplySecurityShieldKindSelector {
    /// (Primary Recovery Confirmation)
    PrimaryAndRecoveryWithExplicitConfirmation,

    /// (Primary Recovery Time)
    PrimaryAndRecoveryWithTimedAutoConfirm,

    /// (Primary Confirmation)
    PrimaryAndExplicitConfirmation,

    /// (Primary Time) ‼️ REQUIRES "Dugong" ‼️
    PrimaryWithTimedAutoConfirm,

    /// (Recovery Confirmation)
    RecoveryAndExplicitConfirmation,

    /// (Recovery Time)
    RecoveryWithTimedAutoConfirm,
}

#[derive(Debug, Clone)]
pub struct TransactionManifestApplySecurityShieldAnyInput {
    pub security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
    pub apply_shield_manifest_kind:
        Option<TransactionManifestApplySecurityShieldKindSelector>,
}
impl TransactionManifestApplySecurityShieldAnyInput {
    fn new(
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        apply_shield_manifest_kind: impl Into<
            Option<TransactionManifestApplySecurityShieldKindSelector>,
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
        apply_shield_manifest_kind: TransactionManifestApplySecurityShieldKindSelector,
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

#[derive(Debug, Clone)]
pub struct TransactionManifestApplySecurityShieldSecurifiedInput {
    pub security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
    pub apply_shield_manifest_kind:
        TransactionManifestApplySecurityShieldKindSelector,
}

impl TransactionManifestApplySecurityShieldSecurifiedInput {
    pub fn new(
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        apply_shield_manifest_kind: TransactionManifestApplySecurityShieldKindSelector,
    ) -> Self {
        Self {
            security_structure_of_factor_instances,
            apply_shield_manifest_kind,
        }
    }
}
