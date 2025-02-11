use sargon::OsSecurityStructuresQuerying;

use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Returns the entities linked to a given `SecurityStructureID`, either on the current `Profile` or a specific one.
    pub async fn entities_linked_to_security_structure(
        &self,
        shield_id: SecurityStructureID,
        profile_to_check: ProfileToCheck,
    ) -> Result<EntitiesLinkedToSecurityStructure> {
        self.wrapped
            .entities_linked_to_security_structure(
                shield_id.into_internal(),
                profile_to_check.into_internal(),
            )
            .await
            .into_result()
    }
}
