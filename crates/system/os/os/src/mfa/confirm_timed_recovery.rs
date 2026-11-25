use crate::prelude::*;

impl SargonOS {
    pub async fn make_confirm_timed_recovery_manifest(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        let profile_snapshot = self.profile()?;
        let entity = profile_snapshot.entity_by_address(address)?;
        let securified = AnySecurifiedEntity::new(entity.clone())?;
        // The manifest confirms the provisional config's rules.
        if securified
            .securified_entity_control
            .provisional_securified_config
            .is_none()
        {
            return Err(CommonError::EntityHasNoProvisionalSecurityConfigSet);
        }

        Ok(TransactionManifest::confirm_timed_recovery(securified))
    }
}
