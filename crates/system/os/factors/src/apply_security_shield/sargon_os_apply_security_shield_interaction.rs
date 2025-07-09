use std::sync::{Mutex, OnceLock};

use crate::prelude::*;
use addresses::address_union;
use radix_connect::DappToWalletInteractionBatchOfTransactions;

#[async_trait::async_trait]
pub trait OsApplySecurityShieldInteraction {
    /// For every entity in the given set of addresses, we use FactorInstancesProvider
    /// to create `SecurityStructureOfFactorInstances` based on the security shield
    /// loaded by the given `security_shield_id`. We update Profile to set
    /// a provisional security config with the derived factors.
    ///
    /// We then create one TransactionManifest for each entity, if the entity
    /// is unsecurified it will be a Manifest which calls "securify" on the
    /// component and creates an AccessController for it. If the entity is already
    /// securified, we will create a Manifest which uses default `RolesExercisableInTransactionManifestCombination`
    /// to update the security config.
    ///
    /// These manifests are not fully valid yet - we need to set a payer of XRD
    /// vault top up and TX fee - which uses different logic depending on if
    /// the entity applying the shield is securified or not. If it is securified
    /// we will need to lock fee against the XRD vault of the AccessController -
    /// if the entity is unsecurified we will need to lock fee against the account
    /// address if it is an account - if it is a Persona another account - securified
    /// or not - will need to be used to pay TX fee.
    ///
    /// Host should present these manifests to the user for approval, and then
    /// display necessary input UI components for payer and fee locking - and then
    /// host need to tell Sargon to create many more manifest for each securified
    /// entity, `RolesExercisableInTransactionManifestCombination::all() - 1` more
    /// manifests need to be created and set payer and fee locking for each of them.
    ///
    /// When user slides to sign - Sargon will try to batch sign each kind of manifest
    /// based on `RolesExercisableInTransactionManifestCombination` for the securified
    /// entities - and for the unsecurified entities the Primary role will always
    /// be used.
    async fn make_interaction_for_applying_security_shield(
        &self,
        security_shield_id: SecurityStructureID,
        addresses: IndexSet<AddressOfAccountOrPersona>,
    ) -> Result<DappToWalletInteractionBatchOfTransactions>;
}

address_union!(
    enum EntityApplyingShieldAddress: accessController, account, identity
);

impl From<AccountOrPersona> for EntityApplyingShieldAddress {
    fn from(value: AccountOrPersona) -> Self {
        match value.security_state() {
            EntitySecurityState::Securified { value } => {
                Self::AccessController(value.access_controller_address())
            }
            EntitySecurityState::Unsecured { .. } => match value {
                AccountOrPersona::AccountEntity(account) => {
                    Self::Account(account.address)
                }
                AccountOrPersona::PersonaEntity(persona) => {
                    Self::Identity(persona.address)
                }
            },
        }
    }
}

fn hacky_tmp_entities_applying_shield(
) -> &'static Mutex<IndexMap<EntityApplyingShieldAddress, TransactionManifest>>
{
    static ARRAY: OnceLock<
        Mutex<IndexMap<EntityApplyingShieldAddress, TransactionManifest>>,
    > = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(IndexMap::new()))
}

/// Called by `make_interaction_for_applying_security_shield` to set the entities
fn hacky_tmp_set_entities_applying_shield(
    entities: IndexMap<EntityApplyingShieldAddress, TransactionManifest>,
) {
    *hacky_tmp_entities_applying_shield().lock().unwrap() = entities;
}

pub fn hacky_tmp_get_entities_applying_shield(
) -> IndexMap<EntityApplyingShieldAddress, TransactionManifest> {
    hacky_tmp_entities_applying_shield().lock().unwrap().clone()
}

impl EntityApplyingShieldAddress {
    fn from_unsecurified_entity(entity: &AnyUnsecurifiedEntity) -> Self {
        match &entity.entity {
            AccountOrPersona::AccountEntity(ref account) => {
                Self::Account(account.address())
            }
            AccountOrPersona::PersonaEntity(ref persona) => {
                Self::Identity(persona.address())
            }
        }
    }
}

// TODO: when https://github.com/radixdlt/sargon/pull/373 and follow up PRs are merged and we can get those addresses from the manifest using RETs analysis
// is merge remove this and use static analysis using RET to get this.
fn __hacky_tmp_using_local_global_state_extract_address_of_entity_updating_shield(
    manifest: &TransactionManifest,
) -> Result<EntityApplyingShieldAddress> {
    let lookup = hacky_tmp_get_entities_applying_shield();
    let address = lookup.iter().find_map(|(address, m)| {
        if m == manifest {
            Some(*address)
        } else {
            None
        }
    });
    address.ok_or(CommonError::Unknown)
}

// TODO: when https://github.com/radixdlt/sargon/pull/373 and follow up PRs are merged
// impl this
fn _extract_address_of_entity_updating_shield(
    _manifest: &TransactionManifest,
) -> Result<EntityApplyingShieldAddress> {
    todo!("cannot be implemented yet, awaiting #132 RET PR")
}

// TODO: when https://github.com/radixdlt/sargon/pull/373 and follow up PRs are merged
// is merge remove this and use static analysis using RET to get this.
pub fn extract_address_of_entity_updating_shield(
    manifest: &TransactionManifest,
) -> Result<EntityApplyingShieldAddress> {
    __hacky_tmp_using_local_global_state_extract_address_of_entity_updating_shield(manifest)
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

        let mut manifests_for_entity =
            IndexMap::<EntityApplyingShieldAddress, TransactionManifest>::new();

        let manifests_for_unsecurified = entities_with_provisional
       .unsecurified_erased()
            .iter()
            .map(|e| {
                let provisional = e.entity.get_provisional().expect("Entity should have a provisional config set since we applied shield above");
                let derived = provisional.as_factor_instances_derived().expect("Should have derived factors");
                TransactionManifest::apply_security_shield_for_unsecurified_entity(
                    e.clone(),
                    derived.clone()
                ).inspect(|m| {
                    manifests_for_entity.insert(
                        EntityApplyingShieldAddress::from_unsecurified_entity(&e),
                        m.clone()
                    );
                }).map(UnvalidatedTransactionManifest::from)
        }).collect::<Result<Vec<UnvalidatedTransactionManifest>>>()?;

        let manifests_for_securified = entities_with_provisional
        .securified_erased()
             .iter()
             .map(|e| {
                let provisional = e.entity.get_provisional().expect("Entity should have a provisional config set since we applied shield above");
                let derived = provisional.as_factor_instances_derived().expect("Should have derived factors");
                let manifest = TransactionManifest::apply_security_shield_for_securified_entity(
                    e.clone(),
                    derived.clone(),
                    RolesExercisableInTransactionManifestCombination::manifest_end_user_gets_to_preview()
                );
                manifests_for_entity.insert(
                    EntityApplyingShieldAddress::AccessController(
                        e.securified_entity_control.access_controller_address()
                    ),
                    manifest.clone()
                );
                UnvalidatedTransactionManifest::from(manifest)
         }).collect_vec();

        // TODO: when https://github.com/radixdlt/sargon/pull/373 and follow up PRs are merged
        // is merge remove this and use static analysis using RET to get this.
        hacky_tmp_set_entities_applying_shield(manifests_for_entity);

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
                true
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
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();
            let persona = os
                .create_and_save_new_persona_with_bdfs(
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
                .create_and_save_new_account_with_bdfs(
                    network,
                    DisplayName::sample(),
                )
                .await
                .unwrap();
            let persona = os
                .create_and_save_new_persona_with_bdfs(
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
