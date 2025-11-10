use crate::prelude::*;
use sargon::OsCommitProvisionalSecurityState;
use sargon::OsMarkAsSecurified;

#[uniffi::export]
impl SargonOS {
    async fn commit_provisional_security_state(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<()> {
        self.wrapped
            .commit_provisional_security_state(entity_address.into_internal())
            .await
            .into_result()
    }

    async fn remove_provisional_security_state(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<()> {
        self.wrapped
            .remove_provisional_security_state(entity_address.into_internal())
            .await
            .into_result()
    }
}
