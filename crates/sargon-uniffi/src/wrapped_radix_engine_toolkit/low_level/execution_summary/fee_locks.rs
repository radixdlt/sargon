use crate::prelude::*;
use sargon::FeeLocks as InternalFeeLocks;

/// Information on how much fees were contingent and how much were not.
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct FeeLocks {
    pub lock: Decimal192,
    pub contingent_lock: Decimal192,
}

impl From<InternalFeeLocks> for FeeLocks {
    fn from(value: InternalFeeLocks) -> Self {
        Self {
            lock: value.lock.into(),
            contingent_lock: value.contingent_lock.into(),
        }
    }
}

impl Into<InternalFeeLocks> for FeeLocks {
    fn into(self) -> InternalFeeLocks {
        InternalFeeLocks {
            lock: self.lock.into(),
            contingent_lock: self.contingent_lock.into(),
        }
    }
}