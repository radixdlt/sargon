use crate::prelude::*;

impl SargonOS {
    pub async fn make_update_security_shield_manifest(
        &self,
        security_structure: SecurityStructureOfFactorSources,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        let profile_snapshot = self.profile()?;
        let entity = profile_snapshot.entity_by_address(address)?;

        match entity.security_state() {
            EntitySecurityState::Unsecured { value: _ } => {
                self.make_setup_security_shield_manifest(
                    security_structure,
                    address,
                )
                .await
            }
            EntitySecurityState::Securified { value: sec_control } => {
                self.make_transaction_manifest_for_securified_entity(
                    security_structure,
                    entity,
                    sec_control.security_structure,
                )
                .await
            }
        }
    }
}
