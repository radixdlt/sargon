use crate::prelude::*;
use std::{borrow::Borrow, sync::RwLock};
use sargon::ProfileState as InternalProfileState;

#[derive(
    Debug, Clone, PartialEq, EnumAsInner,  uniffi::Enum,
)]
#[allow(clippy::large_enum_variant)]
pub enum ProfileState {
    /// When no profile exists in secure storage when OS is booted.
    None,

    /// When the profile snapshot retrieved from secure storage failed to convert into a
    /// valid Profile.
    Incompatible(CommonError),

    /// When a valid 'Profile' exists. This can either happen when the os boots, or a profile is
    /// restored, or the user creates a new profile.
    Loaded(Profile),
}

impl From<InternalProfileState> for ProfileState {
    fn from(value: InternalProfileState) -> Self {
        unimplemented!()
    }
}