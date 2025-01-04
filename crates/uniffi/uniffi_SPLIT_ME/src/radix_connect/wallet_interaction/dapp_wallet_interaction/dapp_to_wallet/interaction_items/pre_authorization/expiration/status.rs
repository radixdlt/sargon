use crate::prelude::*;
use sargon::DappToWalletInteractionSubintentExpirationStatus as InternalDappToWalletInteractionSubintentExpirationStatus;

/// An enum that represents the expiration status of a subintent at a given time.
///
/// Useful for determining if a subintent is still valid at the moment the Host apps
/// receive the corresponding request.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum DappToWalletInteractionSubintentExpirationStatus {
    /// The subintent hasn't expired yet
    Valid,

    /// The subintent is too close to its expiration. Although it hasn't expired yet, the Host apps
    /// shouldn't allow the user dealing with it.
    ExpirationTooClose,

    /// The subintent has already expired.
    Expired,
}
