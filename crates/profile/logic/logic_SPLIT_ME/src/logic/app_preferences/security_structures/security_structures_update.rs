use crate::prelude::*;

pub trait ProfileSecurityStructureUpdating {
    /// Returns the list of IDs of updated Security Shields - either one or two elements.
    /// It returns one id if there wasn't any existing main Security Shield.
    /// It returns two ids if there was a previous main Security Shield whose flag was removed.
    fn set_main_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>>;

    /// Renames the Security Shield with the given `shield_id`.
    fn set_security_structure_name(
        &mut self,
        shield_id: &SecurityStructureID,
        name: DisplayName,
    ) -> Result<()>;
}

impl ProfileSecurityStructureUpdating for Security {
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

    fn set_security_structure_name(
        &mut self,
        shield_id: &SecurityStructureID,
        name: DisplayName,
    ) -> Result<()> {
        self.update_security_structure_name(shield_id, name)
    }
}

impl ProfileSecurityStructureUpdating for AppPreferences {
    fn set_main_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>> {
        self.security.set_main_security_structure(shield_id)
    }

    fn set_security_structure_name(
        &mut self,
        security_structure_id: &SecurityStructureID,
        name: DisplayName,
    ) -> Result<()> {
        self.security
            .update_security_structure_name(security_structure_id, name)
    }
}

impl ProfileSecurityStructureUpdating for Profile {
    fn set_main_security_structure(
        &mut self,
        shield_id: &SecurityStructureID,
    ) -> Result<Vec<SecurityStructureID>> {
        self.app_preferences.set_main_security_structure(shield_id)
    }

    fn set_security_structure_name(
        &mut self,
        security_structure_id: &SecurityStructureID,
        name: DisplayName,
    ) -> Result<()> {
        self.app_preferences
            .set_security_structure_name(security_structure_id, name)
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

    fn update_security_structure_name(
        &mut self,
        shield_id: &SecurityStructureID,
        name: DisplayName,
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

    fn update_security_structure_name(
        &mut self,
        security_structure_id: &SecurityStructureID,
        name: DisplayName,
    ) -> Result<()> {
        self.security_structures_of_factor_source_ids
            .try_update_with(security_structure_id, |s| {
                s.metadata.update_name(name)
            })
            .map_err(|_| CommonError::InvalidSecurityStructureID {
                bad_value: security_structure_id.to_string(),
            })
    }
}
