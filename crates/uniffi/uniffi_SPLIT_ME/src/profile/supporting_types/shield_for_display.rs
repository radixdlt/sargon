use crate::prelude::*;
use sargon::ShieldForDisplay as InternalShieldForDisplay;

decl_vec_samples_for!(ShieldsForDisplay, ShieldForDisplay);

/// A minimal version of a Security Structure meant for display purposes within wallet
#[derive(Clone, PartialEq, Hash, Eq, Debug, uniffi::Record)]
pub struct ShieldForDisplay {
    pub metadata: SecurityStructureMetadata,
    pub number_of_linked_accounts: u32,
    pub number_of_linked_hidden_accounts: u32,
    pub number_of_linked_personas: u32,
    pub number_of_linked_hidden_personas: u32,
}

impl ShieldForDisplay {
    pub fn into_internal(&self) -> InternalShieldForDisplay {
        self.clone().into()
    }
}

impl From<InternalShieldForDisplay> for ShieldForDisplay {
    fn from(internal: InternalShieldForDisplay) -> Self {
        Self {
            metadata: SecurityStructureMetadata::from(internal.metadata),
            number_of_linked_accounts: internal.number_of_linked_accounts
                as u32,
            number_of_linked_hidden_accounts: internal
                .number_of_linked_hidden_accounts
                as u32,
            number_of_linked_personas: internal.number_of_linked_personas
                as u32,
            number_of_linked_hidden_personas: internal
                .number_of_linked_hidden_personas
                as u32,
        }
    }
}

impl From<ShieldForDisplay> for InternalShieldForDisplay {
    fn from(internal: ShieldForDisplay) -> Self {
        Self {
            metadata: internal.metadata.into(),
            number_of_linked_accounts: internal.number_of_linked_accounts
                as usize,
            number_of_linked_hidden_accounts: internal
                .number_of_linked_hidden_accounts
                as usize,
            number_of_linked_personas: internal.number_of_linked_personas
                as usize,
            number_of_linked_hidden_personas: internal
                .number_of_linked_hidden_personas
                as usize,
        }
    }
}

#[uniffi::export]
pub fn new_shield_for_display_sample() -> ShieldForDisplay {
    InternalShieldForDisplay::sample().into()
}

#[uniffi::export]
pub fn new_shield_for_display_sample_other() -> ShieldForDisplay {
    InternalShieldForDisplay::sample_other().into()
}
