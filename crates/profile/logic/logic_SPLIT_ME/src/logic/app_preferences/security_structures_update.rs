use crate::prelude::*;

pub trait MainSecurityStructureUpdating {
    /// Returns the list of IDs of updated Security Shields - either one or two elements.
    /// It returns one id if there wasn't any existing main Security Shield.
    /// It returns two ids if there was a previous main Security Shield whose flag was removed.
    fn set_main_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>>;
}

impl MainSecurityStructureUpdating for Security {
    /// Returns the list of IDs of updated Security Shields - either one or two elements.
    /// It returns one id if there wasn't any existing main Security Shield.
    /// It returns two ids if there was a previous main Security Shield whose flag was removed.
    fn set_main_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>> {
        let current_main_shield = self.get_main_security_structure();

        let updated_ids = match current_main_shield {
            Some(ref current_main_shield) => {
                vec![current_main_shield.metadata.id, *shield_id]
            }
            None => vec![*shield_id],
        };

        self.update_security_structure_add_flag_main(shield_id)?;

        if let Some(current_main_shield) = &current_main_shield {
            self.update_security_structure_remove_flag_main(
                &current_main_shield.metadata.id,
            )?;
        }

        Ok(updated_ids)
    }
}

impl MainSecurityStructureUpdating for AppPreferences {
    fn set_main_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>> {
        self.security.set_main_security_structure(shield_id)
    }
}

impl MainSecurityStructureUpdating for Profile {
    fn set_main_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>> {
        self.app_preferences.set_main_security_structure(shield_id)
    }
}

pub trait SecurityStructuresUpdating {
    fn update_security_structure_remove_flag_main(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<()>;

    fn update_security_structure_add_flag_main(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<()>;
}

impl SecurityStructuresUpdating for Security {
    /// # Throws
    /// Throws `CommonError:InvalidSecurityStructureID` if the structure identified by `shield_id` does not exist.
    fn update_security_structure_remove_flag_main(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<()> {
        self.security_structures_of_factor_source_ids
            .try_update_with(shield_id, |s| {
                s.metadata.remove_flag(SecurityStructureFlag::Main)
            })
            .map_err(|_| CommonError::InvalidSecurityStructureID {
                bad_value: shield_id.to_string(),
            })
    }

    /// # Throws
    /// Throws `CommonError:InvalidSecurityStructureID` if the structure identified by `shield_id` does not exist.
    fn update_security_structure_add_flag_main(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<()> {
        self.security_structures_of_factor_source_ids
            .try_update_with(shield_id, |s| {
                s.metadata.insert_flag(SecurityStructureFlag::Main)
            })
            .map_err(|_| CommonError::InvalidSecurityStructureID {
                bad_value: shield_id.to_string(),
            })
    }
}
