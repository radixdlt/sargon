use crate::prelude::*;
use sargon::ProfileToCheck as InternalProfileToCheck;

/// The Profile to which we want to check the entities linked to a factor source.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
#[allow(clippy::large_enum_variant)]
pub enum ProfileToCheck {
    /// We should check against the current Profile.
    Current,

    /// We should check against a specific Profile.
    /// Useful when we are in the Import Mnenmonics flow.
    Specific(Profile),
}
