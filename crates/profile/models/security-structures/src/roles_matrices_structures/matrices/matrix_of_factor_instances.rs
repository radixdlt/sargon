use crate::prelude::*;

pub type MatrixOfFactorInstances = AbstractMatrixBuilt<FactorInstance>;

impl MatrixOfFactorInstances {
    pub fn timed_recovery_delay_in_minutes(&self) -> u32 {
        self.time_until_delayed_confirmation_is_callable
            .in_minutes()
    }
}

impl HasFactorInstances for MatrixOfFactorInstances {
    fn unique_tx_signing_factor_instances(&self) -> IndexSet<FactorInstance> {
        let mut set = IndexSet::new();
        set.extend(self.primary_role.all_factors().into_iter().cloned());
        set.extend(self.recovery_role.all_factors().into_iter().cloned());
        set.extend(self.confirmation_role.all_factors().into_iter().cloned());
        set
    }
}

pub trait HasFactorInstances {
    fn unique_tx_signing_factor_instances(&self) -> IndexSet<FactorInstance>;

    /// Override this method for types which has an authentication signing factor
    /// instance, e.g. `SecurityStructureOfFactorInstances`.
    fn unique_all_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.unique_tx_signing_factor_instances()
    }

    fn assert_has_entity_kind(
        &self,
        entity_kind_of_entity: CAP26EntityKind,
    ) -> Result<()> {
        let entity_kind_of_factor_instances =
            self.entity_kind_of_all_factors()?;

        if entity_kind_of_entity != entity_kind_of_factor_instances {
            return Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: entity_kind_of_entity.to_string(), entity_kind_of_factor_instances: entity_kind_of_factor_instances.to_string() });
        }

        Ok(())
    }

    fn entity_kind_of_all_factors(&self) -> Result<CAP26EntityKind> {
        let index_agnostic_path =
            self.index_agnostic_path_of_all_tx_signing_factor_instances()?;
        Ok(index_agnostic_path.entity_kind)
    }

    fn index_agnostic_path_of_all_tx_signing_factor_instances(
        &self,
    ) -> Result<IndexAgnosticPath> {
        let factors = self
            .unique_tx_signing_factor_instances()
            .into_iter()
            .filter_map(|f| f.try_as_hd_factor_instances().ok())
            .collect_vec();

        if factors.is_empty() {
            return Err(CommonError::NoTransactionSigningFactorInstance);
        }

        let index_agnostic_path =
            factors.first().unwrap().derivation_path().agnostic();

        if factors
            .iter()
            .any(|f| f.get_entity_kind() != index_agnostic_path.entity_kind)
        {
            return Err(CommonError::WrongEntityKindOfInFactorInstancesPath);
        }

        if factors
            .iter()
            .any(|f| f.get_key_kind() != CAP26KeyKind::TransactionSigning)
        {
            return Err(
                CommonError::WrongKeyKindOfTransactionSigningFactorInstance,
            );
        }

        Ok(index_agnostic_path)
    }

    /// Returns whether the entity is linked to the given factor source.
    fn is_linked_to_factor_source(&self, factor_source: FactorSource) -> bool {
        self.unique_all_factor_instances().iter().any(|factor| {
            factor.factor_source_id == factor_source.factor_source_id()
        })
    }
}

impl MatrixOfFactorInstances {
    fn sample_from_matrix_of_sources(
        network_id: NetworkID,
        matrix_of_sources: MatrixOfFactorSources,
        entity_kind: CAP26EntityKind,
    ) -> Self {
        let mut consuming_instances =
            MnemonicWithPassphrase::derive_instances_for_factor_sources(
                network_id,
                1,
                [if entity_kind == CAP26EntityKind::Account {
                    DerivationPreset::AccountMfa
                } else {
                    DerivationPreset::IdentityMfa
                }],
                matrix_of_sources.all_factors().into_iter().cloned(),
            );

        Self::fulfilling_matrix_of_factor_sources_with_instances(
            &mut consuming_instances,
            matrix_of_sources.clone(),
        )
        .unwrap()
    }
}

trait InstancesDeriving {
    fn derive_instances_for_factor_sources(
        network_id: NetworkID,
        quantity_per_factor: usize,
        derivation_presets: impl IntoIterator<Item = DerivationPreset>,
        sources: impl IntoIterator<Item = FactorSource>,
    ) -> IndexMap<FactorSourceIDFromHash, FactorInstances>;
}

impl InstancesDeriving for MnemonicWithPassphrase {
    fn derive_instances_for_factor_sources(
        network_id: NetworkID,
        quantity_per_factor: usize,
        derivation_presets: impl IntoIterator<Item = DerivationPreset>,
        sources: impl IntoIterator<Item = FactorSource>,
    ) -> IndexMap<FactorSourceIDFromHash, FactorInstances> {
        let derivation_presets =
            derivation_presets.into_iter().collect::<Vec<_>>();

        let next_index_assigner =
            NextDerivationEntityIndexWithEphemeralOffsets::default();

        sources
            .into_iter()
            .map(|fs| {
                let fsid = fs.id_from_hash();
                let mwp = fsid.sample_associated_mnemonic();

                let paths = derivation_presets
                    .clone()
                    .into_iter()
                    .map(|dp| (dp, quantity_per_factor))
                    .collect::<IndexMap<DerivationPreset, usize>>();

                let paths = paths
                    .into_iter()
                    .flat_map(|(derivation_preset, qty)| {
                        // `qty` many paths
                        (0..qty)
                            .map(|_| {
                                let index_agnostic_path = derivation_preset
                                    .index_agnostic_path_on_network(network_id);

                                next_index_assigner
                                    .reserve(fsid, index_agnostic_path)
                                    .map(|index| {
                                        DerivationPath::from_index_agnostic_path_and_component(index_agnostic_path, index)
                                    })
                                    .unwrap()
                            })
                            .collect::<IndexSet<DerivationPath>>()
                    })
                    .collect::<IndexSet<DerivationPath>>();

                let instances = mwp
                    .derive_public_keys(paths)
                    .into_iter()
                    .map(|public_key| {
                        HierarchicalDeterministicFactorInstance::new(
                            fsid, public_key,
                        )
                    })
                    .collect::<FactorInstances>();

                (fsid, instances)
            })
            .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>()
    }
}

impl HasSampleValues for MatrixOfFactorInstances {
    /// Account
    fn sample() -> Self {
        Self::sample_from_matrix_of_sources(
            NetworkID::Mainnet,
            MatrixOfFactorSources::sample(),
            CAP26EntityKind::Account,
        )
    }

    /// Persona
    fn sample_other() -> Self {
        Self::sample_from_matrix_of_sources(
            NetworkID::Mainnet,
            MatrixOfFactorSources::sample_other(),
            CAP26EntityKind::Identity,
        )
    }
}

impl MatrixOfFactorInstances {
    pub fn sample_sim() -> Self {
        Self::sample_from_matrix_of_sources(
            NetworkID::Simulator,
            MatrixOfFactorSources::sample(),
            CAP26EntityKind::Account,
        )
    }

    pub fn sample_other_sim() -> Self {
        Self::sample_from_matrix_of_sources(
            NetworkID::Simulator,
            MatrixOfFactorSources::sample_other(),
            CAP26EntityKind::Account,
        )
    }
}

impl SecurityStructureOfFactorInstances {
    pub fn fulfilling_structure_of_factor_sources_with_instances(
        consuming_instances: &mut IndexMap<
            FactorSourceIDFromHash,
            FactorInstances,
        >,
        existing_rola_key: Option<HierarchicalDeterministicFactorInstance>,
        security_structure_of_factor_sources: &SecurityStructureOfFactorSources,
    ) -> Result<Self, CommonError> {
        let matrix_of_factors = MatrixOfFactorInstances::fulfilling_matrix_of_factor_sources_with_instances(
        consuming_instances,
        security_structure_of_factor_sources.matrix_of_factors.clone(),
      )?;

        let authentication_signing = match existing_rola_key {
            Some(existing) => Ok(existing),
            None => {
                if let Some(existing) = consuming_instances.get_mut(
                    &security_structure_of_factor_sources
                        .authentication_signing_factor
                        .id_from_hash(),
                ) {
                    let instance = existing.first_authentication_signing().ok_or(
    CommonError::MissingRolaKeyForSecurityStructureOfFactorInstances,
    )?;

                    let _ = existing.shift_remove(&instance); // don't forget to consume it!
                    Ok(instance)
                } else {
                    Err(CommonError::MissingRolaKeyForSecurityStructureOfFactorInstances)
                }
            }
        }?;

        Self::new(
            security_structure_of_factor_sources.id(),
            matrix_of_factors,
            authentication_signing,
        )
    }
}

impl MatrixOfFactorInstances {
    /// Maps `MatrixOfFactorSources -> MatrixOfFactorInstances` by
    /// "assigning" FactorInstances to each MatrixOfFactorInstances from
    /// `consuming_instances`.
    ///
    /// NOTE:
    /// **One FactorInstance might be used multiple times in the MatrixOfFactorInstances,
    /// e.g. ones in the PrimaryRole(WithFactorInstances) and again in RecoveryRole(WithFactorInstances) or
    /// in RecoveryRole(WithFactorInstances)**.
    ///
    /// However, the same FactorInstance is NEVER used in two different MatrixOfFactorInstances.
    ///
    ///
    fn fulfilling_matrix_of_factor_sources_with_instances(
        consuming_instances: &mut IndexMap<
            FactorSourceIDFromHash,
            FactorInstances,
        >,
        matrix_of_factor_sources: MatrixOfFactorSources,
    ) -> Result<Self, CommonError> {
        let instances = &consuming_instances.clone();

        let primary_role =
            PrimaryRoleWithFactorInstances::fulfilling_role_of_factor_sources_with_factor_instances(
                instances,
                &matrix_of_factor_sources,
            )?;
        let recovery_role =
            RecoveryRoleWithFactorInstances::fulfilling_role_of_factor_sources_with_factor_instances(
                instances,
                &matrix_of_factor_sources,
            )?;
        let confirmation_role =
            ConfirmationRoleWithFactorInstances::fulfilling_role_of_factor_sources_with_factor_instances(
                instances,
                &matrix_of_factor_sources,
            )?;

        let matrix = unsafe {
            Self::unbuilt_with_roles_and_days(
                primary_role,
                recovery_role,
                confirmation_role,
                matrix_of_factor_sources
                    .time_until_delayed_confirmation_is_callable,
            )
        };

        // Now that we have assigned instances, **possibly the SAME INSTANCE to multiple roles**,
        // lets delete them from the `consuming_instances` map.
        for instance in matrix.all_factors() {
            let fsid =
                &FactorSourceIDFromHash::try_from(instance.factor_source_id)
                    .unwrap();
            let existing = consuming_instances.get_mut(fsid).unwrap();

            let to_remove = HierarchicalDeterministicFactorInstance::try_from(
                instance.clone(),
            )
            .unwrap();

            // We remove at the beginning of the list first.
            existing.shift_remove(&to_remove);

            if existing.is_empty() {
                // not needed per se, but feels prudent to "prune".
                consuming_instances.shift_remove_entry(fsid);
            }
        }

        Ok(matrix)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MatrixOfFactorInstances;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn timed_recovery_delay_in_minutes() {
        let sut = SUT::sample();
        assert_eq!(sut.timed_recovery_delay_in_minutes(), 14_u32 * 24 * 60);
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(
            SUT::sample().unique_tx_signing_factor_instances(),
            SUT::sample_other().unique_tx_signing_factor_instances()
        );
    }

    #[test]
    fn err_if_no_instance_found_for_factor_source() {
        assert!(matches!(
            SUT::fulfilling_matrix_of_factor_sources_with_instances(
                &mut IndexMap::new(),
                MatrixOfFactorSources::sample()
            ),
            Err(CommonError::MissingFactorMappingInstancesIntoRole)
        ));
    }

    #[test]
    fn empty_is_err() {
        let invalid = unsafe {
            SUT::unbuilt_with_roles_and_days(
                PrimaryRoleWithFactorInstances::unbuilt_with_factors(
                    Threshold::All,
                    [],
                    [],
                ),
                RecoveryRoleWithFactorInstances::unbuilt_with_factors(
                    Threshold::zero(),
                    [],
                    [],
                ),
                ConfirmationRoleWithFactorInstances::unbuilt_with_factors(
                    Threshold::zero(),
                    [],
                    [],
                ),
                TimePeriod::with_days(1),
            )
        };
        let res =
            invalid.index_agnostic_path_of_all_tx_signing_factor_instances();
        assert!(matches!(
            res,
            Err(CommonError::NoTransactionSigningFactorInstance)
        ));
    }

    #[test]
    fn err_if_empty_instance_found_for_factor_source() {
        assert!(matches!(
            SUT::fulfilling_matrix_of_factor_sources_with_instances(
                &mut IndexMap::kv(
                    FactorSource::sample_device_babylon().id_from_hash(),
                    FactorInstances::from_iter([])
                ),
                MatrixOfFactorSources::sample()
            ),
            Err(CommonError::MissingFactorMappingInstancesIntoRole)
        ));
    }

    #[test]
    fn assert_json_sample() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "primaryRole": {
                "threshold": "all",
                "thresholdFactors": [
                  {
                    "factorSourceID": {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "device",
                        "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                      }
                    },
                    "badge": {
                      "discriminator": "virtualSource",
                      "virtualSource": {
                        "discriminator": "hierarchicalDeterministicPublicKey",
                        "hierarchicalDeterministicPublicKey": {
                          "publicKey": {
                            "curve": "curve25519",
                            "compressedData": "427969814e15d74c3ff4d9971465cb709d210c8a7627af9466bdaa67bd0929b7"
                          },
                          "derivationPath": {
                            "scheme": "cap26",
                            "path": "m/44H/1022H/1H/525H/1460H/0S"
                          }
                        }
                      }
                    }
                  },
                  {
                    "factorSourceID": {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "ledgerHQHardwareWallet",
                        "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                      }
                    },
                    "badge": {
                      "discriminator": "virtualSource",
                      "virtualSource": {
                        "discriminator": "hierarchicalDeterministicPublicKey",
                        "hierarchicalDeterministicPublicKey": {
                          "publicKey": {
                            "curve": "curve25519",
                            "compressedData": "92cd6838cd4e7b0523ed93d498e093f71139ffd5d632578189b39a26005be56b"
                          },
                          "derivationPath": {
                            "scheme": "cap26",
                            "path": "m/44H/1022H/1H/525H/1460H/0S"
                          }
                        }
                      }
                    }
                  }
                ],
                "overrideFactors": []
              },
              "recoveryRole": {
                "threshold": "all",
                "thresholdFactors": [],
                "overrideFactors": [
                  {
                    "factorSourceID": {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "arculusCard",
                        "body": "12f36554769cd96614776e6dbd5629825b8e87366eec5e515de32bb1ea153820"
                      }
                    },
                    "badge": {
                      "discriminator": "virtualSource",
                      "virtualSource": {
                        "discriminator": "hierarchicalDeterministicPublicKey",
                        "hierarchicalDeterministicPublicKey": {
                          "publicKey": {
                            "curve": "curve25519",
                            "compressedData": "999bc2b17d012c3ce49da85b880029be5f9a9093247821f746ba73b6fc20e406"
                          },
                          "derivationPath": {
                            "scheme": "cap26",
                            "path": "m/44H/1022H/1H/525H/1460H/0S"
                          }
                        }
                      }
                    }
                  }
                ]
              },
              "confirmationRole": {
                "threshold": "all",
                "thresholdFactors": [],
                "overrideFactors": [
                  {
                    "factorSourceID": {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "password",
                        "body": "181ab662e19fac3ad9f08d5c673b286d4a5ed9cd3762356dc9831dc42427c1b9"
                      }
                    },
                    "badge": {
                      "discriminator": "virtualSource",
                      "virtualSource": {
                        "discriminator": "hierarchicalDeterministicPublicKey",
                        "hierarchicalDeterministicPublicKey": {
                          "publicKey": {
                            "curve": "curve25519",
                            "compressedData": "4af49eb56b1af579aaf03f1760ec526f56e2297651f7a067f4b362f685417a81"
                          },
                          "derivationPath": {
                            "scheme": "cap26",
                            "path": "m/44H/1022H/1H/525H/1460H/0S"
                          }
                        }
                      }
                    }
                  }
                ]
              },
              "timeUntilDelayedConfirmationIsCallable": {
            	"value": 2,
            	"unit": "weeks"
              }
            }
            "#,
        );
    }
}
