use crate::prelude::*;

use sargon::OsShieldApplying;

// ==================
// Apply Shield to entities
// ==================
#[uniffi::export]
impl SargonOS {
    pub async fn apply_security_shield_to_entities(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: Vec<AddressOfAccountOrPersona>,
    ) -> Result<()> {
        self.wrapped
            .apply_security_shield_to_entities(
                security_shield_id.into_internal(),
                addresses.into_internal(),
            )
            .await
            .into_result()
    }
}
