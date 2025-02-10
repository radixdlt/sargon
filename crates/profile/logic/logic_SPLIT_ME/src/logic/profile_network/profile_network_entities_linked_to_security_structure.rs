use crate::prelude::*;

pub trait ProfileNetworkQueryEntitiesLinkedToSecurityStructure {
    fn entities_linked_to_security_structure(
        &self,
        metadata: SecurityStructureMetadata,
    ) -> Result<EntitiesLinkedToSecurityStructure>;
}

impl ProfileNetworkQueryEntitiesLinkedToSecurityStructure for ProfileNetwork {
    /// Returns the entities linked to a given `SecurityStructure` on the current `ProfileNetwork`.
    fn entities_linked_to_security_structure(
        &self,
        metadata: SecurityStructureMetadata,
    ) -> Result<EntitiesLinkedToSecurityStructure> {
        let accounts = self
            .accounts_non_hidden()
            .iter()
            .filter(|a| a.is_linked_to_security_structure(metadata.id))
            .collect::<Accounts>();
        let hidden_accounts = self
            .accounts_hidden()
            .iter()
            .filter(|a| a.is_linked_to_security_structure(metadata.id))
            .collect::<Accounts>();
        let personas = self
            .personas_non_hidden()
            .iter()
            .filter(|p| p.is_linked_to_security_structure(metadata.id))
            .collect::<Personas>();
        let hidden_personas = self
            .personas_hidden()
            .iter()
            .filter(|p| p.is_linked_to_security_structure(metadata.id))
            .collect::<Personas>();

        Ok(EntitiesLinkedToSecurityStructure::new(
            metadata,
            accounts,
            hidden_accounts,
            personas,
            hidden_personas,
        ))
    }
}
