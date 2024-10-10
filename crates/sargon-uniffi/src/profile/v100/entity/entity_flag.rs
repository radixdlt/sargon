use crate::prelude::*;
use sargon::EntityFlag as InternalEntityFlag;

/// Flags used to mark state of an Account or Persona such as whether
/// user has marked it as deleted or not.
#[derive(
    Clone,
    
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum EntityFlag {
    /// The entity is marked as deleted by user. Entity should still be kept in Profile
    DeletedByUser,

    /// Just a temporary placeholder value used by Sample Values.
    PlaceholderSampleValueFlag,
}

impl From<InternalEntityFlag> for EntityFlag {
    fn from(value: InternalEntityFlag) -> Self {
        match value {
            InternalEntityFlag::DeletedByUser => Self::DeletedByUser,
            InternalEntityFlag::PlaceholderSampleValueFlag => Self::PlaceholderSampleValueFlag,
        }
    }
}

impl Into<InternalEntityFlag> for EntityFlag {
    fn into(self) -> InternalEntityFlag {
        match self {
            EntityFlag::DeletedByUser => InternalEntityFlag::DeletedByUser,
            EntityFlag::PlaceholderSampleValueFlag => InternalEntityFlag::PlaceholderSampleValueFlag,
        }
    }
}

#[uniffi::export]
pub fn new_entity_flag_sample() -> EntityFlag {
    InternalEntityFlag::sample().into()
}

#[uniffi::export]
pub fn new_entity_flag_sample_other() -> EntityFlag {
    InternalEntityFlag::sample_other().into()
}

