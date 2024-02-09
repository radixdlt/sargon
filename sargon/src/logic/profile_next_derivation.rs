use crate::prelude::*;

impl Profile {
    #[cfg(not(tarpaulin_include))] // false negative
    pub fn factor_source_by_id<F>(&self, id: &FactorSourceID) -> Result<F>
    where
        F: IsFactorSource,
    {
        self.factor_sources
            .get(id)
            .ok_or(CommonError::ProfileDoesNotContainFactorSourceWithID(
                id.clone(),
            ))
            .and_then(|f| {
                f.clone().try_into().map_err(|_| {
                    CommonError::CastFactorSourceWrongKind {
                        expected: <F as IsFactorSource>::kind(),
                        found: f.factor_source_kind(),
                    }
                })
            })
    }

    pub fn device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<DeviceFactorSource> {
        self.factor_source_by_id(&id.clone().into())
    }

    pub fn bdfs(&self) -> DeviceFactorSource {
        let device_factor_source = self
            .factor_sources
            .clone()
            .into_iter()
            .filter_map(|f| f.as_device().cloned())
            .collect_vec();

        let explicit_main = device_factor_source
            .clone()
            .into_iter()
            .filter(|x| x.is_main_bdfs())
            .collect_vec()
            .first()
            .cloned();

        let implicit_main = device_factor_source
            .into_iter()
            .filter(|x| x.common.supports_babylon())
            .collect_vec()
            .first()
            .expect(
                "A Profile should always contain Babylon DeviceFactorSource",
            )
            .clone();

        explicit_main.unwrap_or(implicit_main).clone()
    }

    fn next_derivation_index_for_entity_for_factor_source(
        &self,
        kind: EntityKind,
        network_id: NetworkID,
        factor_source_id: FactorSourceIDFromHash,
    ) -> HDPathValue {
        match kind {
            EntityKind::Persona => panic!("Personas are not supported yet"),
            EntityKind::Accounts => {}
        };
        let index = self
            .networks
            .get(&network_id)
            .map(|n| {
                n.accounts
                    .items()
                    .into_iter()
                    .filter(|a| match &a.security_state {
                        EntitySecurityState::Unsecured { value } => {
                            value.transaction_signing.factor_source_id
                                == factor_source_id
                        }
                    })
                    .collect_vec()
                    .len()
            })
            .unwrap_or(0);

        index as u32
    }

    pub fn next_derivation_index_for_entity(
        &self,
        kind: EntityKind,
        network_id: NetworkID,
    ) -> HDPathValue {
        self.next_derivation_index_for_entity_for_factor_source(
            kind,
            network_id,
            self.bdfs().id,
        )
    }
}

impl Profile {
    pub fn placeholder_no_device_factor_source() -> Self {
        let networks = ProfileNetworks::placeholder();
        let mut header = Header::placeholder();
        header.content_hint = networks.content_hint();
        Self::with(
            header,
            FactorSources::from_iter([FactorSource::placeholder_ledger()]),
            AppPreferences::placeholder(),
            networks,
        )
    }

    pub fn placeholder_no_babylon_device_factor_source() -> Self {
        let networks = ProfileNetworks::placeholder();
        let mut header = Header::placeholder();
        header.content_hint = networks.content_hint();
        Self::with(
            header,
            FactorSources::from_iter([
                DeviceFactorSource::placeholder_olympia().into(),
            ]),
            AppPreferences::placeholder(),
            networks,
        )
    }

    pub fn placeholder_no_factor_source_explicitly_marked_as_main() -> Self {
        let mut profile = Profile::placeholder();

        let main_factors = profile
            .factor_sources
            .iter()
            .filter_map(|f| f.as_device().cloned())
            .filter(|df| df.common.flags.contains(&FactorSourceFlag::Main))
            .map(|f| f.factor_source_id())
            .collect_vec();

        main_factors.iter().for_each(|id| {
            _ = profile.factor_sources.update_with(id, |f| {
                _ = f
                    .as_device_mut()
                    .unwrap()
                    .common
                    .flags
                    .remove(&FactorSourceFlag::Main)
            })
        });

        profile
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn factor_source_by_id_success_device() {
        let profile = Profile::placeholder();
        let dfs = DeviceFactorSource::placeholder_babylon();
        assert_eq!(
            profile.factor_source_by_id::<DeviceFactorSource>(
                &dfs.factor_source_id()
            ),
            Ok(dfs)
        );
    }

    #[test]
    fn factor_source_by_id_success_ledger() {
        let profile = Profile::placeholder();
        let lfs = LedgerHardwareWalletFactorSource::placeholder();
        assert_eq!(
            profile.factor_source_by_id::<LedgerHardwareWalletFactorSource>(
                &lfs.factor_source_id()
            ),
            Ok(lfs)
        );
    }

    #[test]
    fn factor_source_by_id_fail_wrong_kind() {
        let profile = Profile::placeholder();
        let dfs = DeviceFactorSource::placeholder_babylon();
        assert_eq!(
            profile.factor_source_by_id::<LedgerHardwareWalletFactorSource>(
                &dfs.factor_source_id()
            ),
            Err(CommonError::CastFactorSourceWrongKind {
                expected: FactorSourceKind::LedgerHQHardwareWallet,
                found: FactorSourceKind::Device,
            })
        );
    }

    #[test]
    fn factor_source_by_id_fail_unknown_id() {
        let profile = Profile::placeholder();
        let lfs = LedgerHardwareWalletFactorSource::placeholder_other();
        assert_eq!(
            profile.factor_source_by_id::<LedgerHardwareWalletFactorSource>(
                &lfs.factor_source_id()
            ),
            Err(CommonError::ProfileDoesNotContainFactorSourceWithID(
                lfs.factor_source_id()
            ))
        );
    }

    #[test]
    fn device_factor_source_by_id_success_device() {
        let profile = Profile::placeholder();
        let dfs = DeviceFactorSource::placeholder_babylon();
        assert_eq!(profile.device_factor_source_by_id(&dfs.id), Ok(dfs));
    }

    #[test]
    fn device_factor_source_by_id_fail_unknown_id() {
        let profile = Profile::placeholder();

        let id = FactorSourceIDFromHash::new_for_device(
            MnemonicWithPassphrase::placeholder_other(),
        );

        assert_eq!(
            profile.device_factor_source_by_id(&id),
            Err(CommonError::ProfileDoesNotContainFactorSourceWithID(
                id.into()
            ))
        );
    }

    #[test]
    fn bdfs_success_without_explicit_main_flag() {
        let profile =
            Profile::placeholder_no_factor_source_explicitly_marked_as_main();
        assert_eq!(profile.bdfs().id, DeviceFactorSource::placeholder().id);
    }

    #[test]
    fn bdfs_success_with_explicit_main_flag() {
        let profile = Profile::placeholder();
        assert_eq!(profile.bdfs().id, DeviceFactorSource::placeholder().id);
    }

    #[test]
    #[should_panic(
        expected = "A Profile should always contain Babylon DeviceFactorSource"
    )]
    fn bdfs_fail_for_invalid_profile_without_device_factor_source() {
        let profile = Profile::placeholder_no_device_factor_source();
        _ = profile.bdfs();
    }

    #[test]
    #[should_panic(
        expected = "A Profile should always contain Babylon DeviceFactorSource"
    )]
    fn bdfs_fail_for_invalid_profile_without_babylon_device_factor_source() {
        let profile = Profile::placeholder_no_babylon_device_factor_source();
        _ = profile.bdfs();
    }

    #[test]
    fn next_derivation_index_for_entity_account_bdfs_mainnet() {
        let profile = Profile::placeholder();
        assert_eq!(
            profile.next_derivation_index_for_entity(
                EntityKind::Accounts,
                NetworkID::Mainnet
            ),
            2
        );
    }

    #[test]
    fn next_derivation_index_for_entity_account_bdfs_stokenet() {
        let profile = Profile::placeholder();
        assert_eq!(
            profile.next_derivation_index_for_entity(
                EntityKind::Accounts,
                NetworkID::Stokenet
            ),
            2
        );
    }

    #[test]
    fn next_derivation_index_for_entity_account_olympia_dfs_mainnet() {
        let profile = Profile::placeholder();
        assert_eq!(
            profile.next_derivation_index_for_entity_for_factor_source(
                EntityKind::Accounts,
                NetworkID::Mainnet,
                DeviceFactorSource::placeholder_olympia().id
            ),
            0
        );
    }
}
