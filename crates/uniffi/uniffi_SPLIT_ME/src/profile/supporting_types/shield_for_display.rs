use crate::prelude::*;
use sargon::ShieldForDisplay as InternalShieldForDisplay;

/// A minimal version of a Security Structure meant for display purposes within wallet
#[derive(Clone, PartialEq, Hash, Eq, InternalConversion, uniffi::Record)]
pub struct ShieldForDisplay {
    pub metadata: SecurityStructureMetadata,
    pub number_of_linked_accounts: usize,
    pub number_of_linked_hidden_accounts: usize,
    pub number_of_linked_personas: usize,
    pub number_of_linked_hidden_personas: usize,
}

#[uniffi::export]
pub fn new_shield_for_display_sample() -> ShieldForDisplay {
    InternalShieldForDisplay::sample().into()
}

#[uniffi::export]
pub fn new_shield_for_display_sample_other() -> ShieldForDisplay {
    InternalShieldForDisplay::sample_other().into()
}
