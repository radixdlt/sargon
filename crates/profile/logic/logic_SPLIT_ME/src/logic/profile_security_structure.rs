use crate::prelude::*;

pub trait ProfileHasAnyMainSecurityStructure {
    /// Returns true if the profile has any main security structure.
    fn has_any_main_security_structure(&self) -> bool;
}

impl ProfileHasAnyMainSecurityStructure for Profile {
    /// Returns true if the profile has any main security structure.
    fn has_any_main_security_structure(&self) -> bool {
        self.app_preferences
            .security
            .security_structures_of_factor_source_ids
            .iter()
            .any(|s| s.is_main())
    }
}
