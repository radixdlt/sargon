use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    pub async fn fetch_all_access_controllers_details(
        &self,
    ) -> Result<Vec<AccessControllerStateDetails>> {
        self.wrapped
            .fetch_all_access_controllers_details()
            .await
            .into_iter_result()
    }

    pub async fn is_recovery_proposal_unknown(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<bool> {
        self.wrapped
            .is_recovery_proposal_unknown(entity_address.into_internal())
            .await
            .into_result()
    }
}
