use crate::prelude::*;
use sargon::ProfileState as InternalProfileState;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
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
