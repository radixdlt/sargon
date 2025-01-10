use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsShieldApplying {
    async fn apply_security_shield_to_entities(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl OsShieldApplying for SargonOS {
    async fn apply_security_shield_to_entities(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<()> {
        todo!()
    }
}
