use crate::prelude::*;
use sargon::OsMarkAsSecurified;

#[uniffi::export]
impl SargonOS {
    /// Marks the entities as securified by finding the `AccessControllerAddress` on ledger
    /// and updates the profile.
    async fn mark_entities_as_securified(
        &self,
        entity_addresses: Vec<AddressOfAccountOrPersona>,
    ) -> Result<()> {
        self.wrapped
            .mark_entities_as_securified(IndexSet::from_iter(
                entity_addresses.iter().map(|e| e.into_internal()),
            ))
            .await
            .into_result()
    }
}
