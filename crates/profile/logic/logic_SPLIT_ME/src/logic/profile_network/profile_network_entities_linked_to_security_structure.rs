use crate::prelude::*;

pub trait ProfileNetworkQueryEntitiesLinkedToSecurityStructure {
    fn entities_linked_to_security_structure(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<EntitiesLinkedToSecurityStructure>;
}

impl ProfileNetworkQueryEntitiesLinkedToSecurityStructure for ProfileNetwork {
    /// Returns the entities linked to a given `SecurityStructure` on the current `ProfileNetwork`.
    fn entities_linked_to_security_structure(
        &self,
        shield_id: SecurityStructureID,
    ) -> Result<EntitiesLinkedToSecurityStructure> {
        let accounts = self
            .accounts_non_hidden()
            .iter()
            .filter(|a| a.is_linked_to_security_structure(shield_id))
            .collect::<Accounts>();
        let hidden_accounts = self
            .accounts_hidden()
            .iter()
            .filter(|a| a.is_linked_to_security_structure(shield_id))
            .collect::<Accounts>();
        let personas = self
            .personas_non_hidden()
            .iter()
            .filter(|p| p.is_linked_to_security_structure(shield_id))
            .collect::<Personas>();
        let hidden_personas = self
            .personas_hidden()
            .iter()
            .filter(|p| p.is_linked_to_security_structure(shield_id))
            .collect::<Personas>();

        Ok(EntitiesLinkedToSecurityStructure::new(
            accounts,
            hidden_accounts,
            personas,
            hidden_personas,
        ))
    }
}
