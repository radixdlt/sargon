use crate::prelude::*;
use sargon::DerivationPurpose as InternalDerivationPurpose;

/// The purpose that initiated an interaction with the host to derive keys.
/// The orchestrator behind this operation is the `KeysCollector`.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, InternalConversion, uniffi::Enum,
)]
pub enum DerivationPurpose {
    /// When the create account flow, initiates keys collection
    /// for account VECIs
    CreatingNewAccount,

    /// When the create persona flow, initiates keys collection
    /// for identity VECIs
    CreatingNewPersona,

    /// When applying a security shield to an account, initiates keys collection
    /// for account MFA
    SecurifyingAccount,

    /// When applying a security shield to a persona, initiates keys collection
    /// for identity MFA
    SecurifyingPersona,

    /// When applying a security shield to accounts and personas mixed, initiates keys collection
    /// for account MFA
    SecurifyingAccountsAndPersonas,

    /// When adding a new factor source, initiates keys collection
    /// for collecting various factor instances.
    PreDerivingKeys,

    /// When deriving accounts for recovery
    AccountRecovery,
}
