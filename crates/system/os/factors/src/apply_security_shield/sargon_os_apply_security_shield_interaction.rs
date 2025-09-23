use std::sync::{Mutex, OnceLock};

use crate::prelude::*;
use addresses::address_union;

#[async_trait::async_trait]
pub trait OsApplySecurityShieldInteraction {
    async fn make_setup_security_shield_manifest(
        &self,
        security_structure: SecurityStructureOfFactorSources,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest>;
}

#[async_trait::async_trait]
impl OsApplySecurityShieldInteraction for SargonOS {
    async fn make_setup_security_shield_manifest(
        &self,
        security_structure: SecurityStructureOfFactorSources,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        let profile_snapshot = self.profile()?;
        let entity = profile_snapshot.entity_by_address(address)?;
        let key_derivation_interactors = self.keys_derivation_interactor();

        let factor_sources_to_use = security_structure
            .clone()
            .all_factors()
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<IndexSet<FactorSource>>();

        let network_id = address.network_id();

        let index_assigner =
            NextDerivationEntityIndexProfileAnalyzingAssigner::new(
                network_id,
                Some(Arc::new(profile_snapshot)),
            );

        // 1. Create the necessary derivation paths to be used to derive the instances
        let mut per_factor_paths =
            IndexMap::<FactorSourceIDFromHash, IndexSet<DerivationPath>>::new();

        // 1.1 Create ROLA derivation path
        let rola_factor =
            security_structure.clone().authentication_signing_factor;
        let rola_idx_agnostic_path = DerivationPreset::rola_entity_kind(entity.get_entity_kind())
            .index_agnostic_path_on_network(network_id);
        let default_index_rola_index = HDPathComponent::from_local_key_space(
            0u32,
            rola_idx_agnostic_path.key_space,
        )?;
        let rola_derivation_path = index_assigner
            .next(rola_factor.id_from_hash(), rola_idx_agnostic_path)
            .map(|index| {
                DerivationPath::from_index_agnostic_path_and_component(
                    rola_idx_agnostic_path,
                    index.unwrap_or(default_index_rola_index),
                )
            })?;

        per_factor_paths.append_or_insert_element_to(
            rola_factor.id_from_hash(),
            rola_derivation_path,
        );

        // 1.2 Create the matrix configuration derivation paths
        let matrix_factors = security_structure.matrix_of_factors.all_factors();
        let mfa_idx_agnostic_path =
            DerivationPreset::mfa_entity_kind(entity.get_entity_kind())
                .index_agnostic_path_on_network(network_id);
        let default_index_mfa_index = HDPathComponent::from_local_key_space(
            0u32,
            mfa_idx_agnostic_path.key_space,
        )?;
        let matrix_paths = matrix_factors
        .into_iter()
        .map(|factor| {
            let path = index_assigner.next(
                factor.id_from_hash(),
                mfa_idx_agnostic_path
            )
            .map(|index| {
                DerivationPath::from_index_agnostic_path_and_component(
                    mfa_idx_agnostic_path,
                    index.unwrap_or(default_index_mfa_index.clone()),
                 )
            })?;
            Ok((factor.id_from_hash(), path))
        })
        .collect::<Result<IndexMap<FactorSourceIDFromHash, DerivationPath>>>()?;

        for (id, path) in matrix_paths {
            per_factor_paths.append_or_insert_element_to(id, path);
        }

        // 2. Setup keys collector and derive the keys
        let collector = KeysCollector::new(
            factor_sources_to_use,
            per_factor_paths.clone(),
            key_derivation_interactors,
            DerivationPurpose::SecurifyingAccount,
        )?;

        let keys_output = collector.collect_keys().await;

        let mut instances = keys_output
            .factors_by_source
            .into_iter()
            .map(|(id, factors)| {
                let instances = FactorInstances::from(factors);
                (id, instances)
            })
            .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>();

        // 3. Populate the security structure with the instances
        let security_structure_of_factor_instances = SecurityStructureOfFactorInstances::fulfilling_structure_of_factor_sources_with_instances(
            &mut instances,
            None,
            &security_structure
        )?;

        // 4. Set the security structure as provisional, this will be extracted on transaction analysis
        let mut entity = entity;
        entity.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: security_structure_of_factor_instances.clone(),
            },
        );

        self.update_entities_erased(vec![entity.clone()].into())
            .await?;
        for factor_source_id in instances.keys() {
            self.update_last_used_of_factor_source(factor_source_id.clone())
                .await?
        }

        // 5. Create manifest
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                AnyUnsecurifiedEntity::with_unsecured_entity_control(
                    entity.clone(),
                    entity.entity_security_state().into_unsecured().unwrap(),
                ),
                security_structure_of_factor_instances,
            )
    }
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
