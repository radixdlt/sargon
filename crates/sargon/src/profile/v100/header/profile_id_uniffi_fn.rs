use crate::prelude::*;

uniffi::custom_newtype!(ProfileID, Uuid);

#[uniffi::export]
pub fn new_profile_id_sample() -> ProfileID {
    ProfileID::sample()
}

#[uniffi::export]
pub fn new_profile_id_sample_other() -> ProfileID {
    ProfileID::sample_other()
}
