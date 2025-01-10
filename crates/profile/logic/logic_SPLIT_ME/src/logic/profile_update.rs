use time_utils::now;

use crate::prelude::*;

pub trait ProfileDiagnosticsFactorInstances {
    // TODO: Sometimes later it would be nice to remove this method
    // and only use `diagnostics_for_factor_instances_valid_with_handler` and then
    // send a handler from SargonOS which has access to some new driver which
    // can use Swift Issue Reporting API:
    // https://github.com/pointfreeco/swift-issue-reporting
    // which will cause execution to halt with a runtime issue, which will be great
    // for debugging and finding issues!
    // Maybe android host can raise an exception..?
    fn diagnostics_for_factor_instances_valid(&self) {
        self.diagnostics_for_factor_instances_valid_with_handler(|_| {});
    }

    fn diagnostics_for_factor_instances_valid_with_handler(
        &self,
        on_duplicate: impl FnMut(DuplicateInstances),
    );
}

impl ProfileDiagnosticsFactorInstances for Profile {
    fn diagnostics_for_factor_instances_valid_with_handler(
        &self,
        mut on_duplicate: impl FnMut(DuplicateInstances),
    ) {
        let Some(duplicate_instances) = self.check_for_duplicated_instances()
        else {
            return;
        };

        error!("Duplicated FactorInstances found {:?}", duplicate_instances);
        on_duplicate(duplicate_instances);
    }
}

pub trait ProfileAllEntitiesOnAllNetworks {
    /// Returns ALL entities on ALL network, both account and persona, mixed.
    /// Including hidden/deleted entities.
    fn all_entities_on_all_networks(&self) -> IndexSet<AccountOrPersona>;

    /// Returns ALL FactorInstances for ALL Personas and Accounts on ALL networks as keys
    /// and their factor instances as values.
    fn instances_of_each_entity_on_all_networks(
        &self,
    ) -> IndexMap<AccountOrPersona, IndexSet<FactorInstance>> {
        self.all_entities_on_all_networks()
            .into_iter()
            .map(|e| (e.clone(), e.unique_all_factor_instances()))
            .collect()
    }
}

impl ProfileAllEntitiesOnAllNetworks for Profile {
    /// Returns ALL entities on ALL network, both account and persona, mixed.
    /// Including hidden/deleted entities.
    fn all_entities_on_all_networks(&self) -> IndexSet<AccountOrPersona> {
        self.networks
            .iter()
            .flat_map(|n| {
                let mut entities = IndexSet::<AccountOrPersona>::new();
                entities.extend(n.accounts.erased());
                entities.extend(n.personas.erased());
                entities
            })
            .collect::<IndexSet<_>>()
    }
}

pub trait ProfileAssertNewFactorInstancesNotUsed:
    ProfileAllEntitiesOnAllNetworks
{
    fn find_all_duplicate_instances_matching_against(
        &self,
        against: IndexMap<AccountOrPersona, IndexSet<FactorInstance>>,
    ) -> IdentifiedVecOf<DuplicateInstances>;

    /// Checks ALL FactorInstances for ALL Personas and Accounts on ALL networks,
    /// returns `Some(DuplicateInstances)`` if the same
    /// FactorInstances is used between any entity.
    fn check_for_duplicated_instances(&self) -> Option<DuplicateInstances> {
        let whole_profile = self.instances_of_each_entity_on_all_networks();
        self.find_all_duplicate_instances_matching_against(whole_profile)
            .into_iter()
            .next()
    }

    fn assert_new_factor_instances_not_already_used_erased(
        &self,
        entities: impl IntoIterator<Item = AccountOrPersona>,
    ) -> Result<()> {
        let instances_of_new_entities = entities
            .into_iter()
            .map(|e| (e.clone(), e.unique_all_factor_instances()))
            .collect::<IndexMap<AccountOrPersona, IndexSet<_>>>();

        let Some(duplicate_instances) = self
            .find_all_duplicate_instances_matching_against(
                instances_of_new_entities,
            )
            .into_iter()
            .next()
        else {
            return Ok(());
        };

        Err(duplicate_instances.into_error())
    }

    /// Like `check_for_duplicated_instances` but does not check all entities in profile against
    /// all entities in profile, instead checks `instances_of_new_entities` against all entities
    /// in profile. Also this is throwing.
    fn assert_new_factor_instances_not_already_used<
        E: Into<AccountOrPersona>
            + Clone
            + std::fmt::Debug
            + std::cmp::Eq
            + Identifiable,
    >(
        &self,
        entities: impl IntoIterator<Item = E>,
    ) -> Result<()> {
        let entities = entities
            .into_iter()
            .map(Into::<AccountOrPersona>::into)
            .collect::<IdentifiedVecOf<AccountOrPersona>>();

        self.assert_new_factor_instances_not_already_used_erased(entities)
    }
}

impl ProfileAssertNewFactorInstancesNotUsed for Profile {
    /// Returns a list of `DuplicateInstances` where the same `FactorInstance` is used between
    /// entities in this profile, matched against `against`.
    fn find_all_duplicate_instances_matching_against(
        &self,
        against: IndexMap<AccountOrPersona, IndexSet<FactorInstance>>,
    ) -> IdentifiedVecOf<DuplicateInstances> {
        let mut instances_per_entity =
            self.instances_of_each_entity_on_all_networks();

        let mut duplicates = IdentifiedVecOf::<DuplicateInstances>::new();

        let mut check =
            |entity: AccountOrPersona, to_check: IndexSet<FactorInstance>| {
                for (e, existing) in instances_per_entity.iter() {
                    // We don't want to compare an entity against itself
                    if e.address() == entity.address() {
                        continue;
                    }
                    let intersection = existing
                        .intersection(&to_check)
                        .collect::<IndexSet<_>>();

                    intersection.into_iter().for_each(|duplicate| {
                        let duplicate = DuplicateInstances {
                            entity1: e.clone(),
                            entity2: entity.clone(),
                            factor_instance: (*duplicate).clone(),
                        };
                        duplicates.insert(duplicate);
                    });
                }
                instances_per_entity.insert(entity.clone(), to_check);
            };

        for (entity, instances) in against {
            check(entity, instances)
        }

        duplicates
    }
}

pub trait ProfileEntitiesUpdating {
    fn update_entities_erased(
        &mut self,
        updated_entities: IdentifiedVecOf<AccountOrPersona>,
    ) -> Result<()>;

    fn update_entities<E: IsEntity>(
        &mut self,
        updated_entities: IdentifiedVecOf<E>,
    ) -> Result<()> {
        self.update_entities_erased(
            updated_entities.into_iter().map(Into::into).collect(),
        )
    }

    /// Returns a clone of the updated account if found, else None.
    fn update_account<F>(
        &mut self,
        address: &AccountAddress,
        mutate: F,
    ) -> Option<Account>
    where
        F: FnMut(&mut Account);

    /// Returns a clone of the updated persona if found, else None.
    fn update_persona<F>(
        &mut self,
        address: &IdentityAddress,
        mutate: F,
    ) -> Option<Persona>
    where
        F: FnMut(&mut Persona);
}

impl ProfileEntitiesUpdating for Profile {
    fn update_entities_erased(
        &mut self,
        updated_entities: IdentifiedVecOf<AccountOrPersona>,
    ) -> Result<()> {
        self.networks.update_entities_erased(updated_entities)
    }

    /// Returns a clone of the updated account if found, else None.
    fn update_account<F>(
        &mut self,
        address: &AccountAddress,
        mutate: F,
    ) -> Option<Account>
    where
        F: FnMut(&mut Account),
    {
        self.networks.update_account(address, mutate)
    }

    /// Returns a clone of the updated persona if found, else None.
    fn update_persona<F>(
        &mut self,
        address: &IdentityAddress,
        mutate: F,
    ) -> Option<Persona>
    where
        F: FnMut(&mut Persona),
    {
        self.networks.update_persona(address, mutate)
    }
}

pub trait ProfileFactorSourceUpdating {
    fn update_any_factor_source<F>(
        &mut self,
        factor_source_id: &FactorSourceID,
        mutate: F,
    ) -> Result<()>
    where
        F: FnMut(&mut FactorSource);

    fn update_factor_source<S, M>(
        &mut self,
        factor_source_id: &FactorSourceID,
        mutate: M,
    ) -> Result<bool>
    where
        S: IsFactorSource,
        M: FnMut(S) -> Result<S>;

    fn update_any_factor_source_common<F>(
        &mut self,
        factor_source_id: &FactorSourceID,
        mut mutate: F,
    ) -> Result<()>
    where
        F: FnMut(&mut FactorSourceCommon),
    {
        self.update_any_factor_source(factor_source_id, |fs| {
            let mut common = fs.common_properties();
            mutate(&mut common);
            fs.set_common_properties(common);
        })
    }

    fn update_last_used_of_factor_source(
        &mut self,
        id: &FactorSourceID,
    ) -> Result<()> {
        self.update_any_factor_source_common(id, |common| {
            common.last_used_on = now();
        })
    }

    fn update_factor_source_remove_flag_main(
        &mut self,
        id: &FactorSourceID,
    ) -> Result<()> {
        self.update_any_factor_source_common(id, |common| {
            common.flags.remove_id(&FactorSourceFlag::Main);
        })
    }

    fn update_factor_source_add_flag_main(
        &mut self,
        id: &FactorSourceID,
    ) -> Result<()> {
        self.update_any_factor_source_common(id, |common| {
            common.flags.insert(FactorSourceFlag::Main);
        })
    }
}

impl ProfileFactorSourceUpdating for Profile {
    fn update_factor_source<S, M>(
        &mut self,
        factor_source_id: &FactorSourceID,
        mut mutate: M,
    ) -> Result<bool>
    where
        S: IsFactorSource,
        M: FnMut(S) -> Result<S>,
    {
        self.factor_sources
            .maybe_update_with(factor_source_id, |f| {
                S::try_from(f.clone())
                    .map_err(|_| CommonError::CastFactorSourceWrongKind {
                        expected: S::kind().to_string(),
                        found: f.factor_source_kind().to_string(),
                    })
                    .and_then(|element| {
                        mutate(element).map(|modified| modified.into())
                    })
            })
    }

    fn update_any_factor_source<F>(
        &mut self,
        factor_source_id: &FactorSourceID,
        mutate: F,
    ) -> Result<()>
    where
        F: FnMut(&mut FactorSource),
    {
        self.factor_sources
            .try_update_with(factor_source_id, mutate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn update_factor_source_not_update_when_factor_source_not_found() {
        let mut sut = SUT::sample();
        let wrong_id: &FactorSourceID =
            &LedgerHardwareWalletFactorSource::sample_other().id.into();

        assert_eq!(
            sut.update_factor_source(
                wrong_id,
                |lfs: LedgerHardwareWalletFactorSource| { Ok(lfs) }
            ),
            Ok(false)
        );
    }

    #[test]
    fn change_supported_curve_of_factor_source() {
        let mut sut = SUT::sample();
        let id: &FactorSourceID = &DeviceFactorSource::sample().id.into();
        assert!(sut.factor_sources.contains_id(FactorSourceID::from(
            DeviceFactorSource::sample().id
        )));

        assert_eq!(
            sut.factor_sources
                .get_id(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common
                .crypto_parameters
                .supported_curves
                .items(),
            [SLIP10Curve::Curve25519]
        );

        assert_eq!(
            sut.update_factor_source(id, |mut dfs: DeviceFactorSource| {
                dfs.common.crypto_parameters =
                    FactorSourceCryptoParameters::babylon_olympia_compatible();
                Ok(dfs)
            }),
            Ok(true)
        );

        // test failure
        assert_eq!(
            sut.update_factor_source(id, |_: DeviceFactorSource| {
                Err(CommonError::UpdateFactorSourceMutateFailed)
            }),
            Err(CommonError::UpdateFactorSourceMutateFailed)
        );

        assert_eq!(
            sut.factor_sources
                .get_id(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common
                .crypto_parameters
                .supported_curves
                .items(),
            [SLIP10Curve::Curve25519, SLIP10Curve::Secp256k1]
        );
    }

    #[test]
    fn add_supported_curve_to_factor_source_failure_cast_wrong_factor_source_kind(
    ) {
        let mut sut = SUT::sample();
        let id: &FactorSourceID = &DeviceFactorSource::sample().id.into();

        assert!(sut.factor_sources.contains_id(FactorSourceID::from(
            DeviceFactorSource::sample().id
        )));

        assert_eq!(
            sut.factor_sources
                .get_id(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common
                .crypto_parameters
                .supported_curves
                .items(),
            [SLIP10Curve::Curve25519]
        );

        assert_eq!(
            sut.update_factor_source(
                id,
                |mut lfs: LedgerHardwareWalletFactorSource| {
                    lfs.common.crypto_parameters =
                    FactorSourceCryptoParameters::babylon_olympia_compatible();
                    Ok(lfs)
                }
            ),
            Err(CommonError::CastFactorSourceWrongKind {
                expected: FactorSourceKind::LedgerHQHardwareWallet.to_string(),
                found: FactorSourceKind::Device.to_string()
            })
        );

        // Remains unchanged
        assert_eq!(
            sut.factor_sources
                .get_id(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common
                .crypto_parameters
                .supported_curves
                .items(),
            [SLIP10Curve::Curve25519]
        );
    }

    #[test]
    fn update_name_of_accounts() {
        let mut sut = SUT::sample();
        let account = sut
            .networks
            .get_id(NetworkID::Mainnet)
            .unwrap()
            .accounts
            .get_at_index(0)
            .unwrap()
            .clone();

        assert_eq!(account.display_name.value(), "Alice");
        assert!(sut
            .update_account(&account.address, |a| a.display_name =
                DisplayName::new("Bob").unwrap())
            .is_some());

        assert_eq!(
            sut.networks
                .get_id(NetworkID::Mainnet)
                .unwrap()
                .accounts
                .get_at_index(0)
                .unwrap()
                .display_name
                .value(),
            "Bob"
        );
    }

    #[test]
    fn update_name_of_persona() {
        let mut sut = SUT::sample();
        let persona = sut
            .networks
            .get_id(NetworkID::Mainnet)
            .unwrap()
            .personas
            .get_at_index(0)
            .unwrap()
            .clone();

        assert_eq!(persona.display_name.value(), "Satoshi");
        assert!(sut
            .update_persona(&persona.address, |a| a.display_name =
                DisplayName::new("Batman").unwrap())
            .is_some());

        assert_eq!(
            sut.networks
                .get_id(NetworkID::Mainnet)
                .unwrap()
                .personas
                .get_at_index(0)
                .unwrap()
                .display_name
                .value(),
            "Batman"
        );
    }
}
