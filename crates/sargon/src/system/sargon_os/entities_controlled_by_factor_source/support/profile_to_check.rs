use crate::prelude::*;

/// The Profile to which we want to check the entities controlled by a factor source.
#[derive(Clone, Debug, PartialEq)]
pub enum ProfileToCheck {
    /// We should check against the current Profile.
    Current,

    /// We should check against a specific Profile.
    /// Useful when we are in the Import Mnenmonics flow.
    Specific(Profile),
}
