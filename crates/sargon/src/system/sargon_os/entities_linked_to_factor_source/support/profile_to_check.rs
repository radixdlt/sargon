use crate::prelude::*;

/// The Profile to which we want to check the entities linked to a factor source.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum ProfileToCheck {
    /// We should check against the current Profile.
    Current,

    /// We should check against a specific Profile.
    /// Useful when we are in the Import Mnenmonics flow.
    Specific(Profile),
}

impl HasSampleValues for ProfileToCheck {
    fn sample() -> Self {
        Self::Current
    }

    fn sample_other() -> Self {
        Self::Specific(Profile::sample_other())
    }
}
