use crate::prelude::*;
use sargon::OsApplySecurityShieldInteraction;

#[uniffi::export]
impl SargonOS {
    pub async fn make_setup_security_shield_manifest(
        &self,
        security_structure: SecurityStructureOfFactorSources,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        self.wrapped
            .make_setup_security_shield_manifest(
                security_structure.into(),
                address.into(),
            )
            .await
            .into_result()
    }
}
