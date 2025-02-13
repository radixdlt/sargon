use crate::prelude::*;
use sargon::DetailedManifestClassKind as InternalDetailedManifestClassKind;

/// A discriminator type for the `DetailedManifestClass` enum.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum DetailedManifestClassKind {
    General,
    Transfer,
    ValidatorClaim,
    ValidatorStake,
    ValidatorUnstake,
    AccountDepositSettingsUpdate,
    PoolContribution,
    PoolRedemption,
    DeleteAccounts,
    SecurifyEntity,
}

delegate_display_debug_into!(
    DetailedManifestClassKind,
    InternalDetailedManifestClassKind
);
