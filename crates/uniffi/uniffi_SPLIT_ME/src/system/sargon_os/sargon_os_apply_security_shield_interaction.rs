use crate::prelude::*;
use sargon::OsApplySecurityShieldInteraction;

#[uniffi::export]
impl SargonOS {
    pub async fn make_interaction_for_applying_security_shield(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: Vec<AddressOfAccountOrPersona>,
    ) -> Result<DappToWalletInteractionBatchOfTransactions> {
        self.wrapped
            .make_interaction_for_applying_security_shield(
                security_shield_id.into(),
                IndexSet::from_iter(
                    addresses.iter().map(|a| a.into_internal()),
                ),
            )
            .await
            .into_result()
    }
}
