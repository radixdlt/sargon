use crate::prelude::*;
use sargon::KeysCollectionReason as InternalKeysCollectionReason;

/// The reason that `KeysCollector` interacted with the host in order to
/// derive keys.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, InternalConversion, uniffi::Enum,
)]
pub enum KeysCollectionReason {
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

    /// When adding a new factor source, initiates keys collection
    /// for collecting various factor instances.
    PreDerivingKeys,
}
