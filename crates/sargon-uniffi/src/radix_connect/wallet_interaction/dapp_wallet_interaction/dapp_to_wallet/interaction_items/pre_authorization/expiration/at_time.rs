use crate::prelude::*;
use sargon::DappToWalletInteractionSubintentExpireAtTime as InternalDappToWalletInteractionSubintentExpireAtTime;

/// The subintent expires at a specific fixed timestamp
#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionSubintentExpireAtTime {
    /// The unix timestamp in seconds when the subintent expires.
    pub unix_timestamp_seconds: u64,
}
