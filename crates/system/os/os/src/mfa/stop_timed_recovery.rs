use crate::prelude::*;

impl SargonOS {
    pub async fn make_stop_timed_recovery_manifest(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        let profile_snapshot = self.profile()?;
        let entity = profile_snapshot.entity_by_address(address)?;

        Ok(TransactionManifest::stop_timed_recovery(
            AnySecurifiedEntity::new(entity.clone())?,
        ))
    }
}
