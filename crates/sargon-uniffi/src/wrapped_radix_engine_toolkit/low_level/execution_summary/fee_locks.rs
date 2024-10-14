use crate::prelude::*;
use sargon::FeeLocks as InternalFeeLocks;

/// Information on how much fees were contingent and how much were not.
#[derive(Clone, PartialEq, Eq, InternalConversionV2, uniffi::Record)]
pub struct FeeLocks {
    pub lock: Decimal192,
    pub contingent_lock: Decimal192,
}
