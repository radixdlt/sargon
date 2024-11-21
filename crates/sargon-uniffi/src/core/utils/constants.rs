use crate::prelude::*;

#[uniffi::export]
pub fn constant_entity_name_max_length() -> u64 {
    sargon::ENTITY_NAME_MAX_LENGTH
}

#[uniffi::export]
pub fn constant_min_required_xrd_for_account_deletion() -> f64 {
    sargon::MIN_REQUIRED_XRD_FOR_ACCOUNT_DELETION
}
