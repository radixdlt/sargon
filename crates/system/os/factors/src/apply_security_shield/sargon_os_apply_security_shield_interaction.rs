use crate::prelude::*;
use radix_connect::{
    BatchOfTransactionsApplyingSecurityShield,
    DappToWalletInteractionBatchOfTransactions,
};

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
                TransactionManifest::apply_security_shield_for_unsecurified_entity(
                    e.clone(),
                    derived.clone()
                ).map(|manifest| {
                    BatchOfTransactionsApplyingSecurityShield::new(derived.security_structure_id, e.address(), [UnvalidatedTransactionManifest::from(manifest)])
                })
        }).collect::<Result<Vec<BatchOfTransactionsApplyingSecurityShield>>>()?;

        let manifests_for_securified = entities_with_provisional
        .securified_erased()
             .iter()
             .map(|e| {
                let provisional = e.entity.get_provisional().expect("Entity should have a provisional config set since we applied shield above");
                let derived = provisional.as_factor_instances_derived().expect("Should have derived factors");

                let manifests = RolesExercisableInTransactionManifestCombination::all()
                .into_iter()
                .filter_map(|combination| {
                    TransactionManifest::apply_security_shield_for_securified_entity(
                        e.clone(),
                        derived.clone(),
                        combination
                    )
                })
                .map(UnvalidatedTransactionManifest::from)
                .collect::<Vec<_>>();

                BatchOfTransactionsApplyingSecurityShield::new(derived.security_structure_id, e.address(), manifests)
         }).collect::<Vec<BatchOfTransactionsApplyingSecurityShield>>();

        Ok(DappToWalletInteractionBatchOfTransactions::new(
            manifests_for_unsecurified
                .iter()
                .chain(manifests_for_securified.iter())
                .cloned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use prelude::fixture_interaction;

    use super::*;

    #[actix_rt::test]
    async fn test_make_interaction_for_applying_security_shield() {
        // ARRANGE
        let (os, shield_id, addresses_of_mixed_entities_sec_unsec) = {
            let os = SargonOS::fast_boot_bdfs_and_interactor(
                MnemonicWithPassphrase::sample_device_other(),
                None,
                false,
            )
            .await;
            let shield = add_unsafe_shield_with_matrix_with_fixed_metadata(
                &os,
                SecurityStructureMetadata::sample(),
            )
            .await
            .unwrap();
            let shield_id = shield.id();
            let network = NetworkID::Mainnet;
            let account = os
                .create_and_save_new_account_with_main_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();
            let persona = os
                .create_and_save_new_persona_with_main_bdfs(
                    network,
                    DisplayName::sample_other(),
                    None,
                )
                .await
                .unwrap();

            os.apply_security_shield_with_id_to_entities(
                shield_id,
                IndexSet::from_iter([
                    AddressOfAccountOrPersona::from(account.address()),
                    AddressOfAccountOrPersona::from(persona.address()),
                ]),
            )
            .await
            .unwrap();

            // Dummy impl of securifying entities
            let (securified_account, securified_persona) = {
                let mut account =
                    os.account_by_address(account.address()).unwrap();
                let mut persona =
                    os.persona_by_address(persona.address()).unwrap();

                let mut account_security_structure_of_instances = account
                    .get_provisional()
                    .unwrap()
                    .as_factor_instances_derived()
                    .unwrap()
                    .clone();

                // Here we ensure that we test that we reuse the existing ROLA key for the persona below, but not for this account, i.e. the existing ROLA key of this account will mismatch that of the shield.
                account_security_structure_of_instances
                    .authentication_signing_factor_instance =
                    HierarchicalDeterministicFactorInstance::sample_auth_signing();
                assert_ne!(
                    FactorSourceID::from(
                        account_security_structure_of_instances
                            .authentication_signing_factor_instance
                            .factor_source_id
                    ),
                    shield.authentication_signing_factor
                );

                let account_secured_control = SecuredEntityControl::new(
                    account
                        .clone()
                        .security_state()
                        .as_unsecured()
                        .unwrap()
                        .transaction_signing
                        .clone(),
                    AccessControllerAddress::sample_mainnet(),
                    account_security_structure_of_instances,
                )
                .unwrap();
                account
                    .set_security_state(EntitySecurityState::Securified {
                        value: account_secured_control,
                    })
                    .unwrap();
                os.update_account(account.clone()).await.unwrap();

                let persona_security_structure_of_instances = persona
                    .get_provisional()
                    .unwrap()
                    .as_factor_instances_derived()
                    .unwrap()
                    .clone();
                let persona_secured_control = SecuredEntityControl::new(
                    persona
                        .clone()
                        .security_state()
                        .as_unsecured()
                        .unwrap()
                        .transaction_signing
                        .clone(),
                    AccessControllerAddress::sample_mainnet_other(),
                    persona_security_structure_of_instances,
                )
                .unwrap();
                persona
                    .set_security_state(EntitySecurityState::Securified {
                        value: persona_secured_control,
                    })
                    .unwrap();
                os.update_persona(persona.clone()).await.unwrap();

                (account, persona)
            };

            let account = os
                .create_and_save_new_account_with_main_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();
            let persona = os
                .create_and_save_new_persona_with_main_bdfs(
                    network,
                    DisplayName::sample_other(),
                    None,
                )
                .await
                .unwrap();

            (
                os,
                shield_id,
                IndexSet::from_iter([
                    AddressOfAccountOrPersona::from(account.address()),
                    persona.address().into(),
                    securified_account.address().into(),
                    securified_persona.address().into(),
                ]),
            )
        };

        // ACT
        let interaction = {
            os.make_interaction_for_applying_security_shield(
                shield_id,
                addresses_of_mixed_entities_sec_unsec.clone(),
            )
            .await
            .unwrap()
        };

        let fixture_json =
            fixture_interaction!("wallet_interaction_batch_of_transactions");
        assert_eq_after_json_roundtrip(&interaction, fixture_json);
    }
}
