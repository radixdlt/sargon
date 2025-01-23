#![allow(dead_code)]
use profile_supporting_types::AnySecurifiedEntity;

use crate::prelude::*;

pub trait TransactionManifestSecurifySecurifiedEntity:
    TransactionManifestSetRolaKey
{
    fn apply_security_shield_for_securified_entity(
        securified_entity: AnySecurifiedEntity,
        input: TransactionManifestApplySecurityShieldSecurifiedInput,
    ) -> Result<TransactionManifest>;

    fn _update_shield_exercising_recovery_and_explicit_confirmation(
        builder: ScryptoTransactionManifestBuilder,
        securified_entity: &AnySecurifiedEntity,
        input: &TransactionManifestApplySecurityShieldSecurifiedInput,
    ) -> Result<ScryptoTransactionManifestBuilder> {
        todo!()
    }
}

impl TransactionManifestSecurifySecurifiedEntity for TransactionManifest {
    fn apply_security_shield_for_securified_entity(
        securified_entity: AnySecurifiedEntity,
        input: TransactionManifestApplySecurityShieldSecurifiedInput,
    ) -> Result<Self> {
        let TransactionManifestApplySecurityShieldSecurifiedInput {
            security_structure_of_factor_instances,
            apply_shield_manifest_kind: kind,
        } = input.clone();

        let entity_address = securified_entity.entity.address();

        let mut builder = ScryptoTransactionManifestBuilder::new();

        use TransactionManifestApplySecurityShieldKindSelector::*;
        builder = match kind {
            PrimaryAndRecoveryWithExplicitConfirmation => Self::_update_shield_exercising_recovery_and_explicit_confirmation(builder, &securified_entity, &input)?,
            PrimaryAndRecoveryWithTimedAutoConfirm => todo!(),
            PrimaryAndExplicitConfirmation => todo!(),
            PrimaryWithTimedAutoConfirm => todo!(),
            RecoveryAndExplicitConfirmation => todo!(),
            RecoveryWithTimedAutoConfirm => todo!(),
        };

        // Set Rola Key
        let should_set_rola_key = security_structure_of_factor_instances
            .authentication_signing_factor_instance
            != securified_entity
                .current_authentication_signing_factor_instance();
        if should_set_rola_key {
            if kind.can_set_rola_key() {
                builder = TransactionManifest::set_rola_key(
                    builder,
                    &security_structure_of_factor_instances
                        .authentication_signing_factor_instance,
                    &entity_address,
                );
            } else {
                return Err(CommonError::Unknown); // TODO: new error variant
            }
        }

        let manifest = TransactionManifest::sargon_built(
            builder,
            securified_entity.network_id(),
        );

        // N.B.
        // We do NOT top of XRD vault of AccessController - yet!
        // Host will need to call the function:
        // `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account`
        // after user has selected account to pay in wallet GUI.
        // (and as usual also call `modify_manifest_lock_fee`)

        Ok(manifest)
    }
}

impl TransactionManifestApplySecurityShieldKindSelector {
    fn can_exercise_primary_role(&self) -> bool {
        match self {
            Self::PrimaryAndRecoveryWithExplicitConfirmation => true,
            Self::PrimaryAndRecoveryWithTimedAutoConfirm => true,
            Self::PrimaryAndExplicitConfirmation => true,
            Self::PrimaryWithTimedAutoConfirm => true,
            Self::RecoveryAndExplicitConfirmation => false,
            Self::RecoveryWithTimedAutoConfirm => false,
        }
    }

    fn can_set_rola_key(&self) -> bool {
        self.can_exercise_primary_role()
    }

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
