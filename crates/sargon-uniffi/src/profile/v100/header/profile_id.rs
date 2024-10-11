use crate::prelude::*;
use sargon::ProfileID as InternalProfileID;

/// A stable and globally unique identifier of a Profile.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ProfileID {
    value: Uuid,
}

delegate_display_debug_into!(ProfileID, InternalProfileID);

impl From<InternalProfileID> for ProfileID {
    fn from(value: InternalProfileID) -> Self {
        Self { value: value.0 }
    }
}

impl Into<InternalProfileID> for ProfileID {
    fn into(self) -> InternalProfileID {
        InternalProfileID(self.value)
    }
}

#[uniffi::export]
pub fn new_profile_id_sample() -> ProfileID {
    InternalProfileID::sample().into()
}

#[uniffi::export]
pub fn new_profile_id_sample_other() -> ProfileID {
    InternalProfileID::sample_other().into()
}
