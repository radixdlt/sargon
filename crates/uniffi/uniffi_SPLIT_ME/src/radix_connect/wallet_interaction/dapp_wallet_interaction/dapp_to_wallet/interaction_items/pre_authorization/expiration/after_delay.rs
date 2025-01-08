use crate::prelude::*;
use sargon::DappToWalletInteractionSubintentExpireAfterDelay as InternalDappToWalletInteractionSubintentExpireAfterDelay;

/// Suggests that the subintent's expiry timestamp is set to `current_time + expire_after_seconds`
/// at the last moment, right before the intent is fixed for signing.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionSubintentExpireAfterDelay {
    /// The time (in seconds) after the subintent is signed that it will expire.
    pub expire_after_seconds: u64,
}
