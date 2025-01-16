#[uniffi::export]
pub fn constant_display_name_max_length() -> u64 {
    sargon::DisplayName::MAX_LEN as u64
}

#[uniffi::export]
pub fn constant_entity_name_max_length() -> u64 {
    constant_display_name_max_length()
}

#[uniffi::export]
pub fn constant_min_required_xrd_for_account_deletion() -> f64 {
    sargon::MIN_REQUIRED_XRD_FOR_ACCOUNT_DELETION
}

#[uniffi::export]
pub fn constant_max_recovery_confirmation_fallback_period_units() -> u16 {
    sargon::SecurityShieldBuilder::MAX_RECOVERY_CONFIRMATION_FALLBACK_PERIOD_UNITS
}
