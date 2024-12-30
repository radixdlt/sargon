use crate::prelude::*;

pub type MatrixOfFactorInstances = AbstractMatrixBuilt<FactorInstance>;

impl MatrixOfFactorInstances {
    pub fn timed_recovery_delay_in_minutes(&self) -> u32 {
        let timed_recovery_in_days =
            self.number_of_days_until_auto_confirm as u32;
        MINUTES_PER_DAY * timed_recovery_in_days
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
}

impl MatrixOfFactorInstances {
    fn sample_from_matrix_of_sources(
        matrix_of_sources: MatrixOfFactorSources,
        entity_kind: CAP26EntityKind,
    ) -> Self {
        // let mut consuming_instances =
        //     MnemonicWithPassphrase::derive_instances_for_factor_sources(
        //         NetworkID::Mainnet,
        //         1,
        //         [if entity_kind == CAP26EntityKind::Account {
        //             DerivationPreset::AccountMfa
        //         } else {
        //             DerivationPreset::IdentityMfa
        //         }],
        //         matrix_of_sources.all_factors().into_iter().cloned(),
        //     );

        // Self::fulfilling_matrix_of_factor_sources_with_instances(
        //     &mut consuming_instances,
        //     matrix_of_sources.clone(),
        // )
        // .unwrap()
        todo!() // FIXME
    }
}

impl HasSampleValues for MatrixOfFactorInstances {
    /// Account
    fn sample() -> Self {
        // Self::sample_from_matrix_of_sources(
        //     MatrixOfFactorSources::sample(),
        //     CAP26EntityKind::Account,
        // )
        todo!()
    }

    /// Persona
    fn sample_other() -> Self {
        // Self::sample_from_matrix_of_sources(
        //     MatrixOfFactorSources::sample_other(),
        //     CAP26EntityKind::Identity,
        // )
        todo!()
    }
}

impl SecurityStructureOfFactorInstances {
    pub fn fulfilling_structure_of_factor_sources_with_instances(
        consuming_instances: &mut IndexMap<
            FactorSourceIDFromHash,
            FactorInstances,
        >,
        security_structure_of_factor_sources: &SecurityStructureOfFactorSources,
    ) -> Result<Self, CommonError> {
        let matrix_of_factors = MatrixOfFactorInstances::fulfilling_matrix_of_factor_sources_with_instances(
        consuming_instances,
        security_structure_of_factor_sources.matrix_of_factors.clone(),
      )?;

        let authentication_signing = if let Some(existing) = consuming_instances
            .get_mut(
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
                matrix_of_factor_sources.number_of_days_until_auto_confirm,
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
        assert_eq!(
            sut.timed_recovery_delay_in_minutes(),
            SUT::DEFAULT_NUMBER_OF_DAYS_UNTIL_AUTO_CONFIRM as u32 * 24 * 60
        );
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
                PrimaryRoleWithFactorInstances::unbuilt_with_factors(0, [], []),
                RecoveryRoleWithFactorInstances::unbuilt_with_factors(
                    0,
                    [],
                    [],
                ),
                ConfirmationRoleWithFactorInstances::unbuilt_with_factors(
                    0,
                    [],
                    [],
                ),
                1,
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
                "threshold": 2,
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
                "threshold": 0,
                "thresholdFactors": [],
                "overrideFactors": [
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
                ]
              },
              "confirmationRole": {
                "threshold": 0,
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
              "numberOfDaysUntilAutoConfirm": 14
            }
            "#,
        );
    }
}
