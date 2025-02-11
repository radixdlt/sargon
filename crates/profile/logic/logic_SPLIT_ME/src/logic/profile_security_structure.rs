use crate::prelude::*;

pub trait ProfileHasAnyMainSecurityStructure {
    /// Returns true if the profile has any main security structure.
    fn has_any_main_security_structure(&self) -> bool;
}

impl ProfileHasAnyMainSecurityStructure for Security {
    /// Returns true if the profile has any main security structure.
    fn has_any_main_security_structure(&self) -> bool {
        self.security_structures_of_factor_source_ids
            .iter()
            .any(|s| s.is_main())
    }
}

impl ProfileHasAnyMainSecurityStructure for AppPreferences {
    /// Returns true if the profile has any main security structure.
    fn has_any_main_security_structure(&self) -> bool {
        self.security.has_any_main_security_structure()
    }
}

impl ProfileHasAnyMainSecurityStructure for Profile {
    /// Returns true if the profile has any main security structure.
    fn has_any_main_security_structure(&self) -> bool {
        self.app_preferences.has_any_main_security_structure()
    }
}
