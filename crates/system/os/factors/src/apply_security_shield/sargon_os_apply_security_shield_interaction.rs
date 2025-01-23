use crate::prelude::*;
use radix_connect::DappToWalletInteractionBatchOfTransactions;

#[async_trait::async_trait]
pub trait OsApplySecurityShieldInteraction {
    async fn make_interaction_for_applying_security_shield(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<DappToWalletInteractionBatchOfTransactions>;
}

#[async_trait::async_trait]
impl OsApplySecurityShieldInteraction for SargonOS {
    async fn make_interaction_for_applying_security_shield(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<DappToWalletInteractionBatchOfTransactions> {
        let entities_with_provisional = self
            .apply_security_shield_with_id_to_entities(
                security_shield_id,
                addresses,
            )
            .await?;

        let manifests_for_unsecurified = entities_with_provisional
       .unsecurified_erased()
            .iter()
            .map(|e| {
                let provisional = e.entity.get_provisional().expect("Entity should have a provisional config set since we applied shield above");
                let derived = provisional.as_factor_instances_derived().expect("Should have derived factors");
                let input = TransactionManifestApplySecurityShieldUnsecurifiedInput::new(derived.clone());
                TransactionManifest::apply_security_shield_for_unsecurified_entity(
                    e,
                    input,
                )
        }).collect::<Result<Vec<TransactionManifest>>>()?;

        let manifests_for_securified = entities_with_provisional
        .securified_erased()
             .iter()
             .map(|e| {
                let provisional = e.entity.get_provisional().expect("Entity should have a provisional config set since we applied shield above");
                let _derived = provisional.as_factor_instances_derived().expect("Should have derived factors");
                todo!("Implement TransactionManifest::apply_security_shield_for_securified_entity")
         }).collect::<Result<Vec<TransactionManifest>>>()?;

        Ok(DappToWalletInteractionBatchOfTransactions::new(
            manifests_for_unsecurified
                .iter()
                .chain(manifests_for_securified.iter())
                .cloned()
                .map(UnvalidatedTransactionManifest::from),
        ))
    }
}
