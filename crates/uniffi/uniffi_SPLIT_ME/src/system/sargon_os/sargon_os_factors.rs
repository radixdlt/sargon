use crate::prelude::*;
use sargon::DeviceFactorSourceType as InternalDeviceFactorSourceType;
use sargon::HasIndexInLocalKeySpace;
use sargon::HierarchicalDeterministicFactorInstance as InternalHierarchicalDeterministicFactorInstance;
use sargon::ToAgnosticPath;

/// If we wanna create an Olympia DeviceFactorSource or
/// a Babylon one, either main or not.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum DeviceFactorSourceType {
    Babylon { is_main: bool },
    Olympia,
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct FactorInstanceForDebugPurposes {
    pub derivation_path_full: String,
    pub index_agnostic_derivation_path: String,
    pub public_key_hex: String,
    pub factor_source_id: String,
    pub derivation_entity_index: u32,
    pub factor_source_kind: FactorSourceKind,
}
impl From<InternalHierarchicalDeterministicFactorInstance>
    for FactorInstanceForDebugPurposes
{
    fn from(x: InternalHierarchicalDeterministicFactorInstance) -> Self {
        Self {
            derivation_path_full: x.derivation_path().to_string(),
            index_agnostic_derivation_path: x
                .derivation_path()
                .agnostic()
                .to_string(),
            derivation_entity_index: u32::from(
                x.derivation_entity_index().index_in_local_key_space(),
            ),
            public_key_hex: x.public_key().to_hex(),
            factor_source_id: x.factor_source_id.clone().to_string(),
            factor_source_kind: x.factor_source_id.kind.into(),
        }
    }
}

#[uniffi::export]
impl SargonOS {
    /// Returns the "main Babylon" `DeviceFactorSource` of the current account as
    /// a `DeviceFactorSource`.
    pub fn main_bdfs(&self) -> Result<DeviceFactorSource> {
        self.wrapped.main_bdfs().into_result()
    }

    /// Returns all the factor sources
    pub fn factor_sources(&self) -> Result<Vec<FactorSource>> {
        self.wrapped.factor_sources().into_iter_result()
    }

    pub async fn __debug_factor_instances_in_cache(
        &self,
    ) -> HashMap<FactorSourceIDFromHash, Vec<Vec<FactorInstanceForDebugPurposes>>>
    {
        self.wrapped
            .factor_instances_in_cache()
            .await
            .into_iter()
            .map(|(k, v)| {
                (
                    k.into(),
                    v.into_iter()
                        .map(|x| {
                            x.into_iter()
                                .map(FactorInstanceForDebugPurposes::from)
                                .collect()
                        })
                        .collect(),
                )
            })
            .collect()
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
        self.wrapped
            .update_factor_source(updated.into_internal())
            .await
            .into_result()
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
        self.wrapped
            .add_factor_source(factor_source.into_internal())
            .await
            .into_result()
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
        factor_sources: Vec<FactorSource>,
    ) -> Result<Vec<FactorSourceID>> {
        self.wrapped
            .add_factor_sources(factor_sources.into_internal())
            .await
            .into_iter_result()
    }

    /// Updates the name of the corresponding `factor_source` in Profile. Throws `UpdateFactorSourceMutateFailed` error if the
    /// factor source is not found. Returns the updated `FactorSource`.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    pub async fn update_factor_source_name(
        &self,
        factor_source: FactorSource,
        name: String,
    ) -> Result<FactorSource> {
        self.wrapped
            .update_factor_source_name(factor_source.into_internal(), name)
            .await
            .into_result()
    }

    pub async fn debug_add_all_sample_factors(
        &self,
    ) -> Result<Vec<FactorSourceID>> {
        self.wrapped
            .debug_add_all_sample_factor_sources()
            .await
            .into_iter_result()
    }

    /// Creates a new unsaved DeviceFactorSource from the provided `mnemonic_with_passphrase`,
    /// either a "BDFS" or an "Olympia" one.
    pub fn create_device_factor_source(
        &self,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        factor_type: DeviceFactorSourceType,
    ) -> DeviceFactorSource {
        self.wrapped
            .create_device_factor_source(
                mnemonic_with_passphrase.into_internal(),
                factor_type.into_internal(),
            )
            .into()
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
        self.wrapped
            .load_private_device_factor_source_by_id(&id.into_internal())
            .await
            .into_result()
    }

    /// Set the FactorSource with the given `factor_source_id` as the main factor source of its kind.
    /// Throws `UpdateFactorSourceMutateFailed` error if the factor source is not found.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileSaved` after having successfully written the JSON
    /// of the active profile to secure storage.
    ///
    /// Also emits `EventNotification::ProfileModified { change: EventProfileModified::FactorSourceUpdated { id } }`
    ///
    /// If there is any main `FactorSource` of the given `FactorSourceKind`, such events are emitted also when
    /// removing the flag from the old main factor source.
    pub async fn set_main_factor_source(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Result<()> {
        self.wrapped
            .set_main_factor_source(factor_source_id.into_internal())
            .await
            .into_result()
    }

    /// Triggers the spot check for the given factor source, and returns whether the spot check was successful.
    pub async fn trigger_spot_check(
        &self,
        factor_source: FactorSource,
    ) -> Result<bool> {
        self.wrapped
            .trigger_spot_check(factor_source.into_internal(), false)
            .await
            .into_result()
    }
}
