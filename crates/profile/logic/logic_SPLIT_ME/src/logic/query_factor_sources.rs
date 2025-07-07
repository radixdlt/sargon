use crate::prelude::*;

pub trait ProfileFactorSourceQuerying {
    fn factor_source_by_id<F>(
        &self,
        id: impl Into<FactorSourceID>,
    ) -> Result<F>
    where
        F: IsFactorSource;

    fn device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<DeviceFactorSource> {
        self.factor_source_by_id(*id)
    }

    fn device_factor_sources(&self) -> Vec<DeviceFactorSource>;
}

impl ProfileFactorSourceQuerying for Profile {
    fn factor_source_by_id<F>(&self, id: impl Into<FactorSourceID>) -> Result<F>
    where
        F: IsFactorSource,
    {
        let id = id.into();
        self.factor_sources
            .get_id(id)
            .ok_or(CommonError::ProfileDoesNotContainFactorSourceWithID {
                bad_value: id.to_string(),
            })
            .and_then(|f| {
                f.clone().try_into().map_err(|_| {
                    CommonError::CastFactorSourceWrongKind {
                        expected: <F as IsFactorSource>::kind().to_string(),
                        found: f.factor_source_kind().to_string(),
                    }
                })
            })
    }

    fn device_factor_sources(&self) -> Vec<DeviceFactorSource> {
        self.factor_sources
            .iter()
            .filter_map(|f| f.as_device().cloned())
            .collect_vec()
    }
}

pub trait ProfileSampleValuesWithSpecificFactorSources: Sized {
    fn sample_no_device_factor_source() -> Self;
    fn sample_no_babylon_device_factor_source() -> Self;
    fn sample_no_factor_source_explicitly_marked_as_main() -> Self;
}

impl ProfileSampleValuesWithSpecificFactorSources for Profile {
    fn sample_no_device_factor_source() -> Self {
        let networks = ProfileNetworks::sample();
        let mut header = Header::sample();
        header.content_hint = networks.content_hint();
        Self::with(
            header,
            FactorSources::from_iter([FactorSource::sample_ledger()]),
            AppPreferences::sample(),
            networks,
        )
    }

    fn sample_no_babylon_device_factor_source() -> Self {
        let networks = ProfileNetworks::sample();
        let mut header = Header::sample();
        header.content_hint = networks.content_hint();
        Self::with(
            header,
            FactorSources::from_iter([
                DeviceFactorSource::sample_olympia().into()
            ]),
            AppPreferences::sample(),
            networks,
        )
    }

    fn sample_no_factor_source_explicitly_marked_as_main() -> Self {
        let mut profile = Profile::sample();

        let main_factors = profile
            .factor_sources
            .iter()
            .filter_map(|f| f.as_device().cloned())
            .filter(|df| df.common.flags.contains_id(&FactorSourceFlag::Main))
            .map(|f| f.factor_source_id())
            .collect_vec();

        main_factors.iter().for_each(|id| {
            profile.factor_sources.update_with(id, |f| {
                f.as_device_mut()
                    .unwrap()
                    .common
                    .flags
                    .remove_id(&FactorSourceFlag::Main);
            });
        });

        profile
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn factor_source_by_id_success_device() {
        let profile = Profile::sample();
        let dfs = DeviceFactorSource::sample_babylon();
        assert_eq!(
            profile.factor_source_by_id::<DeviceFactorSource>(
                dfs.factor_source_id()
            ),
            Ok(dfs)
        );
    }

    #[test]
    fn factor_source_by_id_success_ledger() {
        let profile = Profile::sample();
        let lfs = LedgerHardwareWalletFactorSource::sample();
        assert_eq!(
            profile.factor_source_by_id::<LedgerHardwareWalletFactorSource>(
                lfs.factor_source_id()
            ),
            Ok(lfs)
        );
    }

    #[test]
    fn factor_source_by_id_fail_wrong_kind() {
        let profile = Profile::sample();
        let dfs = DeviceFactorSource::sample_babylon();
        assert_eq!(
            profile.factor_source_by_id::<LedgerHardwareWalletFactorSource>(
                dfs.factor_source_id()
            ),
            Err(CommonError::CastFactorSourceWrongKind {
                expected: FactorSourceKind::LedgerHQHardwareWallet.to_string(),
                found: FactorSourceKind::Device.to_string(),
            })
        );
    }

    #[test]
    fn factor_source_by_id_fail_unknown_id() {
        let profile = Profile::sample();
        let lfs = LedgerHardwareWalletFactorSource::sample_other();
        assert_eq!(
            profile.factor_source_by_id::<LedgerHardwareWalletFactorSource>(
                lfs.factor_source_id()
            ),
            Err(CommonError::ProfileDoesNotContainFactorSourceWithID {
                bad_value: lfs.factor_source_id().to_string()
            })
        );
    }

    #[test]
    fn device_factor_source_by_id_success_device() {
        let profile = Profile::sample();
        let dfs = DeviceFactorSource::sample_babylon();
        pretty_assertions::assert_eq!(
            profile.device_factor_source_by_id(&dfs.id),
            Ok(dfs)
        );
    }

    #[test]
    fn device_factor_source_by_id_fail_unknown_id() {
        let profile = Profile::sample();

        let id = FactorSourceIDFromHash::new_for_device(
            &MnemonicWithPassphrase::sample_other(),
        );

        assert_eq!(
            profile.device_factor_source_by_id(&id),
            Err(CommonError::ProfileDoesNotContainFactorSourceWithID {
                bad_value: id.to_string()
            })
        );
    }
}
