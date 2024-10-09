use crate::prelude::*;
use sargon::DeviceFactorSourceType as InternalDeviceFactorSourceType;

/// If we wanna create an Olympia DeviceFactorSource or
/// a Babylon one, either main or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum DeviceFactorSourceType {
    Babylon { is_main: bool },
    Olympia,
}

impl From<InternalDeviceFactorSourceType> for DeviceFactorSourceType {
    fn from(value: InternalDeviceFactorSourceType) -> Self {
        unimplemented!()
    }
}

impl Into<InternalDeviceFactorSourceType> for DeviceFactorSourceType {
    fn into(self) -> InternalDeviceFactorSourceType {
        unimplemented!()
    }
}

#[uniffi::export]
impl SargonOS {
    /// Returns the "main Babylon" `DeviceFactorSource` of the current account as
    /// a `DeviceFactorSource`.
    pub fn bdfs(&self) -> Result<DeviceFactorSource> {
        map_result_from_internal(self.wrapped.bdfs())
    }

    /// Returns all the factor sources
    pub fn factor_sources(&self) -> Result<FactorSources> {
        map_result_from_internal(self.wrapped.factor_sources())
    }

    /// Updates the factor source `updated` by mutating current profile and persisting
    /// the change to secure storage. Throws `UpdateFactorSourceMutateFailed` error if the
    /// factor source is not found.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn update_factor_source(
        &self,
        updated: FactorSource,
    ) -> Result<()> {
        map_result_from_internal(self.wrapped.update_factor_source(updated.into()).await)
    }

    /// Returns `Ok(false)` if the Profile already contained a factor source with the
    /// same id (Profile unchanged occurred).
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceAdded }`
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`,
    /// if the newly added FactorSource is a new **main** flag, then we remove the
    /// main flag from the old BDFS.
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn add_factor_source(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        map_result_from_internal(self.wrapped.add_factor_source(factor_source.into()).await)
    }

    /// Adds all of the provided `factor_sources` to Profile in one single go.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourcesAdded }`
    ///
    /// And also emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated }`,
    /// if the newly added FactorSource is a new **main** flag, then we remove the
    /// main flag from the old BDFS.
    ///
    /// And also emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    pub async fn add_factor_sources(
        &self,
        factor_sources: FactorSources,
    ) -> Result<Vec<FactorSourceID>> {
        self.wrapped.add_factor_sources(factor_sources.into_internal_vec()).await.map_result()
    }

    pub async fn debug_add_all_sample_factors(
        &self,
    ) -> Result<Vec<FactorSourceID>> {
        self.wrapped.debug_add_all_sample_factors().await.map_result()
    }

    /// Creates a new unsaved DeviceFactorSource from the provided `mnemonic_with_passphrase`,
    /// either a "BDFS" or an "Olympia" one.
    pub async fn create_device_factor_source(
        &self,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        factor_type: DeviceFactorSourceType,
    ) -> Result<DeviceFactorSource> {
        self.wrapped.create_device_factor_source(
            mnemonic_with_passphrase.into(),
            factor_type.into(),
        ).await.map_result()
    }

    /// Loads a `MnemonicWithPassphrase` with the `id` of `device_factor_source`,
    /// from SecureStorage, and returns a `PrivateHierarchicalDeterministicFactorSource`
    /// built from both.
    ///
    /// Useful for when you will want to sign transactions or derive public keys for
    /// creation of new entities.
    ///
    /// Returns `Err` if loading or decoding of `MnemonicWithPassphrase` from
    /// SecureStorage fails.
    pub async fn load_private_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        self.wrapped.load_private_device_factor_source_by_id(id.into()).await.map_result()
    }
}