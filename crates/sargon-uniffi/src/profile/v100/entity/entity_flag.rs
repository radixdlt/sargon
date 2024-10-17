use crate::prelude::*;
use sargon::EntityFlag as InternalEntityFlag;

decl_vec_samples_for!(EntityFlags, EntityFlag);

/// Flags used to mark state of an Account or Persona such as whether
/// user has marked it as deleted or not.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum EntityFlag {
    /// The entity is marked as deleted by user. Entity should still be kept in Profile
    DeletedByUser,

    /// Just a temporary placeholder value used by Sample Values.
    PlaceholderSampleValueFlag,
}

#[uniffi::export]
pub fn new_entity_flag_sample() -> EntityFlag {
    InternalEntityFlag::sample().into()
}

#[uniffi::export]
pub fn new_entity_flag_sample_other() -> EntityFlag {
    InternalEntityFlag::sample_other().into()
}
