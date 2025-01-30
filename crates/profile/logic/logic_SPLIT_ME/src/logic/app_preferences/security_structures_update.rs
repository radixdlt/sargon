use crate::prelude::*;

pub trait DefaultSecurityStructureUpdating {
    /// Returns the list of IDs of updated SecurityShields - either one or two elements,
    /// depending on if shield identified by `shield_id` already was "default" shield or not.
    fn set_default_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>>;
}

impl DefaultSecurityStructureUpdating for Security {
    fn set_default_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>> {
        let current_default_shield = self.get_default_security_structure();

        let updated_ids = match current_default_shield {
            Some(ref current_default_shield) => {
                vec![current_default_shield.metadata.id, *shield_id]
            }
            None => vec![*shield_id],
        };

        if let Some(current_default_shield) = &current_default_shield {
            self.update_security_structure_remove_flag_default(
                &current_default_shield.metadata.id,
            )?;
        }
        self.update_security_structure_add_flag_default(shield_id)?;

        Ok(updated_ids)
    }
}

impl DefaultSecurityStructureUpdating for AppPreferences {
    fn set_default_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>> {
        self.security.set_default_security_structure(shield_id)
    }
}

impl DefaultSecurityStructureUpdating for Profile {
    fn set_default_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>> {
        self.app_preferences
            .set_default_security_structure(shield_id)
    }
}

pub trait SecurityStructuresUpdating {
    fn update_security_structure_remove_flag_default(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<()>;

    fn update_security_structure_add_flag_default(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<()>;
}

impl SecurityStructuresUpdating for Security {
    /// # Throws
    /// Throws `CommonError:InvalidSecurityStructureID` if the structure identified by `shield_id` does not exist.
    fn update_security_structure_remove_flag_default(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<()> {
        self.security_structures_of_factor_source_ids
            .try_update_with(shield_id, |s| {
                s.metadata.remove_flag(SecurityStructureFlag::Default)
            })
            .map_err(|_| CommonError::InvalidSecurityStructureID {
                bad_value: shield_id.to_string(),
            })
    }

    /// # Throws
    /// Throws `CommonError:InvalidSecurityStructureID` if the structure identified by `shield_id` does not exist.
    fn update_security_structure_add_flag_default(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<()> {
        self.security_structures_of_factor_source_ids
            .try_update_with(shield_id, |s| {
                s.metadata.insert_flag(SecurityStructureFlag::Default)
            })
            .map_err(|_| CommonError::InvalidSecurityStructureID {
                bad_value: shield_id.to_string(),
            })
    }
}
