pub use crate::prelude::*;
use sargon::Instant as InternalInstant;

/// Represents a Unix timestamp, capturing the seconds since the unix epoch.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct Instant {
    pub seconds_since_unix_epoch: i64,
}
