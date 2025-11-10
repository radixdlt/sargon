use crate::prelude::*;

impl SargonOS {
    pub async fn make_stop_timed_recovery_manifest(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        let profile_snapshot = self.profile()?;
        let entity = profile_snapshot.entity_by_address(address)?;
        let ac_address = entity
            .security_state()
            .as_securified()
            .expect("Entity must be securified")
            .access_controller_address;
        let ac_state_details = self
            .access_controller_state_repository_client
            .get_cached_access_controller_details(&ac_address)
            .await?;
        let recovery_proposal = ac_state_details
            .state
            .recovery_role_recovery_attempt
            .expect("The recovery proposal must be present")
            .recovery_proposal;

        Ok(TransactionManifest::stop_timed_recovery(
            ac_address,
            recovery_proposal,
        ))
    }
}
