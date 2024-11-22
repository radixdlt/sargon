use std::{
    borrow::Borrow,
    ops::{Add, Index},
};

use crate::prelude::*;

/// A cache of factor instances.
///
/// Keyed under FactorSourceID and then under `IndexAgnosticPath`, each holding
/// an ordered set of Factor Instances, with contiguous derivation entity indices,
/// with lowest indices first in the set and highest last.
///
/// Since an IndexAgnosticPath essentially is the tuple `(NetworkID, DerivationPreset)`,
/// you can think of the implementation to not be:
/// `IndexMap<FactorSourceIDFromHash, IndexMap<IndexAgnosticPath, FactorInstances>>`
/// but actually:
/// `IndexMap<FactorSourceIDFromHash, IndexMap<NetworkID, IndexMap<DerivationPreset, FactorInstances>>>`,
/// in fact it could be, but not sure it is more readable. But for the sake of visualizing
/// the cache we use that structure.
///
/// E.g.:
/// ```ignore
/// [
///     "FactorSourceID<Ledger3>": [
///         "Mainnet": [
///             DerivationPreset::AccountVeci: [
///                 (0', key...),
///                 (1', key...),
///                 ...
///                 (29', key...),
///             ],
///             DerivationPreset::AccountMfa: [
///                 (0^, key...),
///                 (1^, key...),
///                 ...
///                 (29^, key...),
///             ],
///            DerivationPreset::IdentityVeci: [
///                 (0', key...),
///                 ...
///                 (29', key...),
///             ],
///             DerivationPreset::IdentityMfa: [
///                 (0^, key...), ..., (29^, key...),
///             ],
///         ],
///         "Stokenet": [
///             DerivationPreset::AccountVeci: [
///                 (0', key...), ..., (29', key...),
///             ],
///             DerivationPreset::AccountMfa: [
///                 (0^, key...), ..., (29^, key...),
///             ],
///            DerivationPreset::IdentityVeci: [
///                 (0', key...), ... (29', key...),
///             ],
///             DerivationPreset::IdentityMfa: [
///                 (0^, key...), ..., (29^, key...),
///             ],
///         ],
///     ],
///     "FactorSourceID<Arculus5>": [
///         "Mainnet": [
///             DerivationPreset::AccountVeci: [
///                 (0', key...),  ...,  (29', key...),
///             ],
///             DerivationPreset::AccountMfa: [ ... ],
///             DerivationPreset::IdentityVeci: [ ... ],
///             DerivationPreset::IdentityMfa: [ ...  ],
///         ],
///         "Stokenet": [
///             DerivationPreset::AccountVeci: [
///                 (0', key...), ..., (29', key...),
///             ],
///             DerivationPreset::AccountMfa: [ ... ],
///             DerivationPreset::IdentityVeci: [ ... ],
///             DerivationPreset::IdentityMfa: [ ... ],
///         ],
///     ],
/// ]
/// ```
///
/// We use `IndexMap` instead of `HashMap` for future proofing when we serialize,
/// deserialize this cache, we want the JSON values to have stable ordering. Note
/// that the only truly **important** ordering is that of `FactorInstances` values,
/// which are ordered since it is a newtype around `IndexSet<HierarchicalDeterministicFactorInstance>`.
///
///
/// The Serde impl of `IndexAgnosticPath` is this:
/// `1H/618H/1460H/S?"`, where `S` is "Securified" KeySpace, the `?` denotes
/// that we do not know the index. For unsecurified KeySpace "H" is used for hardened
/// and "" (theoretically - not used) for unhardened.
#[derive(Debug, Default)]
pub struct FactorInstancesCache {
    /// PER FactorSource PER IndexAgnosticPath FactorInstances (matching that IndexAgnosticPath)
    map: RwLock<FICStorage>,
}

pub(super) type FICStorage = IndexMap<
    FactorSourceIDFromHash,
    IndexMap<IndexAgnosticPath, FactorInstances>,
>;

impl FactorInstancesCache {
    pub fn with_storage(storage: FICStorage) -> Self {
        Self {
            map: RwLock::new(storage),
        }
    }

    pub fn clone_snapshot(&self) -> Self {
        Self {
            map: RwLock::new(self.map.read().unwrap().clone()),
        }
    }

    pub fn serializable_snapshot(&self) -> FactorInstancesCacheSnapshot {
        FactorInstancesCacheSnapshot::from(self.map.read().unwrap().clone())
    }

    /// Inserts `instances` under `factor_source_id` by splitting them and grouping
    /// them by their `IndexAgnosticPath`.
    ///
    /// Returns `Err` if any of the instances is in fact does NOT have `factor_source_id`,
    /// as its factor source id.
    ///
    /// Returns `bool` indicating if an index was skipped resulting in non-contiguousness, which
    /// we do not use for now. Might be something we enforce or not for certain operations
    /// in the future.
    pub fn insert_for_factor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
        instances: &FactorInstances,
    ) -> Result<bool> {
        let mut skipped_an_index_resulting_in_non_contiguity = false;

        let instances_by_agnostic_path =
            InstancesByAgnosticPath::from(instances.clone());
        instances_by_agnostic_path.validate_from_source(factor_source_id)?;
        let mut binding = self.map.write().unwrap();
        if let Some(existing_for_factor) = binding.get_mut(factor_source_id) {
            for (agnostic_path, instances) in instances_by_agnostic_path {
                let instances = instances.factor_instances();

                if let Some(existing_for_path) =
                    existing_for_factor.get_mut(&agnostic_path)
                {
                    if let Some(fi) = instances
                        .intersection(&existing_for_path.factor_instances())
                        .next()
                    {
                        return Err(
                            CommonError::CacheAlreadyContainsFactorInstance {
                                derivation_path: fi
                                    .derivation_path()
                                    .to_string(),
                            },
                        );
                    }

                    if let Some(last) =
                        existing_for_path.factor_instances().last()
                    {
                        let first_new = instances
                            .first()
                            .unwrap()
                            .derivation_path()
                            .index()
                            .map_to_global_key_space();
                        let last_existing = last
                            .derivation_path()
                            .index()
                            .map_to_global_key_space();
                        if first_new != last_existing + 1 {
                            warn!(
                                "Non-contiguous indices, the index `{}` was skipped!",
                                last.derivation_path().index().map_to_global_key_space() + 1
                            );
                            skipped_an_index_resulting_in_non_contiguity = true;
                        }
                    }
                    existing_for_path.extend(instances);
                } else {
                    existing_for_factor.insert(
                        agnostic_path,
                        FactorInstances::from(instances),
                    );
                }
            }
        } else {
            binding.insert(*factor_source_id, instances_by_agnostic_path.0);
        }

        Ok(skipped_an_index_resulting_in_non_contiguity)
    }

    /// Inserts all instance in `per_factor`.
    pub fn insert_all(
        &self,
        per_factor: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
    ) -> Result<()> {
        for (factor_source_id, instances) in per_factor {
            _ = self.insert_for_factor(factor_source_id, instances)?;
        }
        Ok(())
    }

    /// Returns the max derivation entity index for the given `factor_source_id` and `index_agnostic_path`.
    pub fn max_index_for(
        &self,
        factor_source_id: impl Borrow<FactorSourceIDFromHash>,
        index_agnostic_path: impl Borrow<IndexAgnosticPath>,
    ) -> Option<HDPathComponent> {
        self.get_mono_factor(factor_source_id, index_agnostic_path)
            .unwrap_or_default()
            .factor_instances()
            .into_iter()
            .map(|fi| fi.derivation_path().index())
            .max()
    }

    /// Returns enough instances to satisfy the requested quantity for each factor source,
    /// **OR LESS**, never more, and if less, it means we MUST derive more, and if we
    /// must derive more, this function returns the quantities to derive for each factor source,
    /// for each derivation preset, not only the originally requested one.
    pub fn get_poly_factor_with_quantities(
        &self,
        factor_source_ids: &IndexSet<FactorSourceIDFromHash>,
        originally_requested_quantified_derivation_preset: &QuantifiedDerivationPreset,
        network_id: NetworkID,
    ) -> Result<CachedInstancesWithQuantitiesOutcome> {
        let target_quantity =
            originally_requested_quantified_derivation_preset.quantity;
        let mut pf_instances =
            IndexMap::<FactorSourceIDFromHash, FactorInstances>::new();
        let mut pf_pdp_qty_to_derive = IndexMap::<
            FactorSourceIDFromHash,
            IndexMap<DerivationPreset, usize>,
        >::new();
        let mut is_quantity_satisfied_for_all_factor_sources = true;

        for factor_source_id in factor_source_ids {
            for preset in DerivationPreset::all() {
                let index_agnostic_path =
                    preset.index_agnostic_path_on_network(network_id);
                let for_preset = self
                    .get_mono_factor(factor_source_id, index_agnostic_path)
                    .unwrap_or_default();
                let count_in_cache = for_preset.len();
                if preset
                    == originally_requested_quantified_derivation_preset
                        .derivation_preset
                {
                    let satisfies_requested_quantity =
                        count_in_cache >= target_quantity;
                    if satisfies_requested_quantity {
                        // The instances in the cache can satisfy the requested quantity
                        // for this factor source for this derivation preset
                        pf_instances.append_or_insert_to(
                            factor_source_id,
                            // Only take the first `target_quantity` instances
                            // to be used, the rest are not needed and should
                            // remain in the cache (later we will call delete on
                            // all those instances.)
                            for_preset.split_at(target_quantity).0,
                        );
                    } else {
                        // The instances in the cache cannot satisfy the requested quantity
                        // we must derive more!
                        is_quantity_satisfied_for_all_factor_sources = false;
                        // Since we are deriving more we might as well ensure that the
                        // cache is filled with `CACHE_FILLING_QUANTITY` **AFTER** the
                        // requested quantity is satisfied, meaning we will not only
                        // derive `CACHE_FILLING_QUANTITY - count_in_cache`, instead we
                        // derive the `target_quantity` as well.
                        let quantity_to_derive = CACHE_FILLING_QUANTITY
                            - count_in_cache
                            + target_quantity;
                        pf_pdp_qty_to_derive.append_or_insert_element_to(
                            factor_source_id,
                            (preset, quantity_to_derive),
                        );
                        // insert all instances to be used directly
                        pf_instances.append_or_insert_to(
                            factor_source_id,
                            for_preset.clone(),
                        );
                    }
                } else {
                    // Not originally requested derivation preset, calculate number
                    // of instances to derive IF we are going to derive anyway,
                    // we wanna FILL the cache for those derivation presets as well.
                    if count_in_cache < CACHE_FILLING_QUANTITY {
                        let qty_to_derive =
                            CACHE_FILLING_QUANTITY - count_in_cache;
                        pf_pdp_qty_to_derive.append_or_insert_element_to(
                            factor_source_id,
                            (preset, qty_to_derive),
                        );
                    }
                }
            }
        }
        let outcome = if is_quantity_satisfied_for_all_factor_sources {
            CachedInstancesWithQuantitiesOutcome::Satisfied(pf_instances)
        } else {
            CachedInstancesWithQuantitiesOutcome::NotSatisfied {
                partial_instances: pf_instances,
                quantities_to_derive: pf_pdp_qty_to_derive,
            }
        };
        Ok(outcome)
    }
}

#[derive(Debug, PartialEq, Eq, enum_as_inner::EnumAsInner)]
pub enum CachedInstancesWithQuantitiesOutcome {
    Satisfied(IndexMap<FactorSourceIDFromHash, FactorInstances>),
    NotSatisfied {
        partial_instances: IndexMap<FactorSourceIDFromHash, FactorInstances>,
        quantities_to_derive:
            IndexMap<FactorSourceIDFromHash, IndexMap<DerivationPreset, usize>>,
    },
}

impl FactorInstancesCache {
    pub fn get_mono_factor(
        &self,
        factor_source_id: impl Borrow<FactorSourceIDFromHash>,
        index_agnostic_path: impl Borrow<IndexAgnosticPath>,
    ) -> Option<FactorInstances> {
        let binding = self.map.read().unwrap();
        let for_factor = binding.get(factor_source_id.borrow())?;
        let instances = for_factor.get(index_agnostic_path.borrow())?;
        Some(instances.clone())
    }

    pub fn delete(
        &self,
        pf_instances: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
    ) {
        for (factor_source_id, instances_to_delete) in pf_instances {
            if instances_to_delete.is_empty() {
                continue;
            }
            let mut binding = self.map.write().unwrap();
            let existing_for_factor = binding
                .get_mut(factor_source_id)
                .expect("expected to delete factors");

            let instances_to_delete_by_path =
                InstancesByAgnosticPath::from(instances_to_delete.clone());
            for (index_agnostic_path, instances_to_delete) in
                instances_to_delete_by_path
            {
                let instances_to_delete = IndexSet::<
                    HierarchicalDeterministicFactorInstance,
                >::from_iter(
                    instances_to_delete.into_iter()
                );

                let existing_for_path = existing_for_factor
                    .get(&index_agnostic_path)
                    .expect("expected to delete")
                    .factor_instances();

                if !existing_for_path.is_superset(&instances_to_delete) {
                    panic!("Programmer error! Some of the factors to delete were not in cache!");
                }
                let to_keep = existing_for_path
                    .symmetric_difference(&instances_to_delete)
                    .cloned()
                    .collect::<FactorInstances>();

                // replace
                existing_for_factor.insert(index_agnostic_path, to_keep);
            }
        }

        self.prune();
    }

    /// "Prunes" the cache from empty collections
    fn prune(&self) {
        let ids = self.factor_source_ids();
        for factor_source_id in ids.iter() {
            let mut binding = self.map.write().unwrap();

            let inner_map = binding.get_mut(factor_source_id).unwrap();
            if inner_map.is_empty() {
                // empty map, prune it!
                binding.shift_remove(factor_source_id);
                continue;
            }
            // see if pruning of instances inside of values `inner_map` is needed
            let inner_ids = inner_map
                .keys()
                .cloned()
                .collect::<IndexSet<IndexAgnosticPath>>();
            for inner_id in inner_ids.iter() {
                if inner_map.get(inner_id).unwrap().is_empty() {
                    // FactorInstances empty, prune it!
                    inner_map.shift_remove(inner_id);
                }
            }
        }
    }

    fn factor_source_ids(&self) -> IndexSet<FactorSourceIDFromHash> {
        self.map.read().unwrap().keys().cloned().collect()
    }

    /// Reads out the instance of `factor_source_id` without mutating the cache.
    pub fn peek_all_instances_of_factor_source(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Option<IndexMap<IndexAgnosticPath, FactorInstances>> {
        self.map.read().unwrap().get(&factor_source_id).cloned()
    }

    pub fn total_number_of_factor_instances(&self) -> usize {
        self.map
            .read()
            .unwrap()
            .values()
            .map(|x| {
                x.values()
                    .map(|y| y.len())
                    .reduce(Add::add)
                    .unwrap_or_default()
            })
            .reduce(Add::add)
            .unwrap_or_default()
    }
}

#[cfg(test)]
impl FactorInstancesCache {
    /// Queries the cache to see if the cache is full for factor_source_id for
    /// each DerivationPreset
    pub fn is_full(
        &self,
        network_id: NetworkID,
        factor_source_id: FactorSourceIDFromHash,
    ) -> bool {
        DerivationPreset::all()
            .into_iter()
            .map(|preset| {
                self.get_poly_factor_with_quantities(
                    &IndexSet::just(factor_source_id),
                    &QuantifiedDerivationPreset::new(
                        preset,
                        CACHE_FILLING_QUANTITY,
                    ),
                    network_id,
                )
            })
            .all(|outcome| {
                matches!(
                    outcome,
                    Ok(CachedInstancesWithQuantitiesOutcome::Satisfied(_))
                )
            })
    }

    pub fn assert_is_full(
        &self,
        network_id: NetworkID,
        factor_source_id: FactorSourceIDFromHash,
    ) {
        assert!(self.is_full(network_id, factor_source_id));
    }
}

#[cfg(test)]
mod tests {

    use crate::factor_instances_provider::next_index_assigner;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorInstancesCache;

    #[test]
    fn non_contiguous_indices() {
        let sut = SUT::default();
        let fsid = FactorSourceIDFromHash::sample_at(0);

        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(0u32).unwrap(),
            ),
        );
        assert!(!sut
            .insert_for_factor(&fsid, &FactorInstances::from_iter([fi0]))
            .unwrap());
        let fi2 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(2u32).unwrap(),
            ),
        );
        assert!(sut
            .insert_for_factor(&fsid, &FactorInstances::from_iter([fi2]))
            .unwrap());
    }

    #[test]
    fn non_contiguous_indices_securified() {
        let sut = SUT::default();
        let fsid = FactorSourceIDFromHash::sample_at(0);

        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Securified(SecurifiedU30::ZERO),
        );
        assert!(!sut
            .insert_for_factor(&fsid, &FactorInstances::from_iter([fi0]))
            .unwrap());
        let fi2 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Securified(SecurifiedU30::TWO),
        );
        assert!(sut
            .insert_for_factor(&fsid, &FactorInstances::from_iter([fi2]))
            .unwrap());
    }

    #[test]
    fn insert_all_factor_source_id_discrepancy_is_err() {
        let sut = SUT::default();
        assert!(sut
            .insert_all(&IndexMap::kv(
                FactorSourceIDFromHash::sample_passphrase_other(),
                FactorInstances::sample()
            ))
            .is_err())
    }

    #[test]
    fn test_json_of_inner_collection() {
        let element = HierarchicalDeterministicFactorInstance::sample();
        let instances = FactorInstances::from_iter([element.clone()]);
        type Inner = IndexMap<IndexAgnosticPath, FactorInstances>;
        let index_agnostic_path = IndexAgnosticPath::new(
            NetworkID::Mainnet,
            CAP26EntityKind::Account,
            CAP26KeyKind::TransactionSigning,
            KeySpace::Unsecurified { is_hardened: true },
        );
        assert_json_value_eq_after_roundtrip(
            &index_agnostic_path,
            json!("1H/525H/1460H/H?"),
        );
        let inner = Inner::kv(index_agnostic_path, instances);

        assert_eq_after_json_roundtrip(
            &inner,
            r#"
            {
                "1H/525H/1460H/H?": [
                    {
                        "badge": {
			        		"virtualSource": {
			        			"hierarchicalDeterministicPublicKey": {
			        				"publicKey": {
			        					"curve": "curve25519",
			        					"compressedData":   "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
			        				},
			        				"derivationPath": {
			        					"scheme": "cap26",
			        					"path": "m/44H/1022H/1H/525H/1460H/0H"
			        				}
			        			},
			        			"discriminator": "hierarchicalDeterministicPublicKey"
			        		},
			        		"discriminator": "virtualSource"
			        	},
			        	"factorSourceID": {
			        		"fromHash": {
			        			"kind": "device",
			        			"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
			        		},
			        		"discriminator": "fromHash"
			        	}
			        }
                ]
            }
            "#,
        );

        let storage = FICStorage::kv(element.factor_source_id, inner);
        let cache_snapshot =
            FactorInstancesCacheSnapshot::from(storage.clone());

        assert_eq_after_json_roundtrip(
            &cache_snapshot,
            r#"
            {
                "device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a": {
                    "1H/525H/1460H/H?": [
                        {
			            	"publicKey": {
			            		"curve": "curve25519",
			            		"compressedData":    "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
			            	},
			            	"derivationPath": {
			            		"scheme": "cap26",
			            		"path": "m/44H/1022H/1H/525H/1460H/0H"
			            	}
			            }
                    ]
                }
            }
            "#,
        );
    }

    #[test]
    fn json() {
        let sut = SUT::default();
        let fsid = FactorSourceIDFromHash::sample_at(0);

        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(0u32).unwrap(),
            ),
        );
        assert!(!sut
            .insert_for_factor(&fsid, &FactorInstances::from_iter([fi0]))
            .unwrap());
        let fi2 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(2u32).unwrap(),
            ),
        );

        assert!(sut
            .insert_for_factor(&fsid, &FactorInstances::from_iter([fi2]))
            .unwrap());

        let fsid = FactorSourceIDFromHash::sample_at(1);

        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Securified(SecurifiedU30::ZERO),
        );
        assert!(!sut
            .insert_for_factor(&fsid, &FactorInstances::from_iter([fi0]))
            .unwrap());
        let fi2 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Securified(SecurifiedU30::TWO),
        );

        assert!(sut
            .insert_for_factor(&fsid, &FactorInstances::from_iter([fi2]))
            .unwrap());

        let serializable = sut.serializable_snapshot();
        let json = serde_json::to_value(&serializable).unwrap();
        let and_back: FactorInstancesCacheSnapshot =
            serde_json::from_value(json).unwrap();
        assert_eq!(serializable, and_back);

        assert_eq_after_json_roundtrip(
            &serializable,
            r#"
            {
              "device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a": {
                "1H/525H/1460H/H?": [
                  {
                    "publicKey": {
                      "curve": "curve25519",
                      "compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
                    },
                    "derivationPath": {
                      "scheme": "cap26",
                      "path": "m/44H/1022H/1H/525H/1460H/0H"
                    }
                  },
                  {
                    "publicKey": {
                      "curve": "curve25519",
                      "compressedData": "543ece3944963391882cf59e8e5bd5f9a0d5669c7e5b9f5a32fc7e7925464d8f"
                    },
                    "derivationPath": {
                      "scheme": "cap26",
                      "path": "m/44H/1022H/1H/525H/1460H/2H"
                    }
                  }
                ]
              },
              "ledgerHQHardwareWallet:ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b": {
                "1H/525H/1460H/S?": [
                  {
                    "publicKey": {
                      "curve": "curve25519",
                      "compressedData": "92cd6838cd4e7b0523ed93d498e093f71139ffd5d632578189b39a26005be56b"
                    },
                    "derivationPath": {
                      "scheme": "cap26",
                      "path": "m/44H/1022H/1H/525H/1460H/0S"
                    }
                  },
                  {
                    "publicKey": {
                      "curve": "curve25519",
                      "compressedData": "d5c14836f71268aca2df8c560244747277b8ad268a619b18f1c6dbfb6a93f37f"
                    },
                    "derivationPath": {
                      "scheme": "cap26",
                      "path": "m/44H/1022H/1H/525H/1460H/2S"
                    }
                  }
                ]
              }
            }
            "#,
        );
    }

    #[test]
    fn factor_source_discrepancy() {
        let sut = SUT::default();
        let fs0 = FactorSourceIDFromHash::sample_at(0);
        let fs1 = FactorSourceIDFromHash::sample_at(1);
        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fs0,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(0u32).unwrap(),
            ),
        );
        assert!(sut
            .insert_for_factor(
                &fs1, // this is a lie
                &FactorInstances::from_iter([fi0])
            )
            .is_err());
    }

    #[test]
    #[should_panic]
    fn delete_panics_for_unknown() {
        let sut = SUT::default();
        let instances = FactorInstances::sample();
        assert_eq!(instances.len(), 2);
        let factor_source_ids = instances
            .clone()
            .into_iter()
            .map(|fi| fi.factor_source_id())
            .collect::<IndexSet<_>>();
        assert_eq!(factor_source_ids.len(), 1);
        let fsid = factor_source_ids.into_iter().next().unwrap();
        sut.insert_for_factor(
            &fsid,
            &instances
                .clone()
                .into_iter()
                .take(1)
                .collect::<FactorInstances>(),
        )
        .unwrap();

        sut.delete(&IndexMap::kv(fsid, instances));
    }

    #[test]
    fn delete() {
        let sut = SUT::default();

        let factor_source_ids = FactorSource::sample_all()
            .into_iter()
            .map(|f| f.id_from_hash())
            .collect::<IndexSet<_>>();

        let n = 30;
        let mut to_delete =
            IndexMap::<FactorSourceIDFromHash, FactorInstances>::new();
        let mut to_remain =
            IndexMap::<FactorSourceIDFromHash, FactorInstances>::new();
        for factor_source_id in factor_source_ids.clone() {
            let fsid = factor_source_id;
            let instances = (0..n)
                .map(|i| {
                    let fi =
                        HierarchicalDeterministicFactorInstance::new_for_entity(
                            fsid,
                            CAP26EntityKind::Account,
                            Hardened::Unsecurified(
                                UnsecurifiedHardened::try_from(i).unwrap(),
                            ),
                        );

                    if i < 10 {
                        to_delete.append_or_insert_to(
                            &fsid,
                            IndexSet::just(fi.clone()),
                        );
                    } else {
                        to_remain.append_or_insert_to(
                            &fsid,
                            IndexSet::just(fi.clone()),
                        );
                    }
                    fi
                })
                .collect::<IndexSet<_>>();

            sut.insert_for_factor(&fsid, &FactorInstances::from(instances))
                .unwrap();
        }

        sut.delete(&to_delete);

        let path = &IndexAgnosticPath::new(
            NetworkID::Mainnet,
            CAP26EntityKind::Account,
            CAP26KeyKind::TransactionSigning,
            KeySpace::Unsecurified { is_hardened: true },
        );
        for (f, instances) in to_remain {
            assert_eq!(sut.get_mono_factor(f, path).unwrap(), instances)
        }
    }

    #[test]
    fn throws_if_same_is_added() {
        let sut = SUT::default();
        let fsid = FactorSourceIDFromHash::sample_at(0);
        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(0u32).unwrap(),
            ),
        );
        let fi1 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(1u32).unwrap(),
            ),
        );
        assert!(!sut
            .insert_for_factor(
                &fsid,
                &FactorInstances::from_iter([fi0.clone(), fi1])
            )
            .unwrap());

        assert_eq!(
            sut.insert_for_factor(
                &fsid,
                &FactorInstances::from_iter([fi0.clone()])
            )
            .err()
            .unwrap(),
            CommonError::CacheAlreadyContainsFactorInstance {
                derivation_path: fi0.derivation_path().to_string()
            }
        );
    }
}