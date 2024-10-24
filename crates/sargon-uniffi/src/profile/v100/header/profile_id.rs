use crate::prelude::*;
use sargon::ProfileID as InternalProfileID;

uniffi::custom_newtype!(ProfileID, Uuid);

/// A stable and globally unique identifier of a Profile.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion)]
pub struct ProfileID(Uuid);

delegate_display_debug_into!(ProfileID, InternalProfileID);

#[uniffi::export]
pub fn new_profile_id_sample() -> ProfileID {
    InternalProfileID::sample().into()
}

#[uniffi::export]
pub fn new_profile_id_sample_other() -> ProfileID {
    InternalProfileID::sample_other().into()
}
