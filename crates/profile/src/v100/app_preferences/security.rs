use crate::prelude::*;

// FIXME: MFA this is in fact not used, so ok to be a `bool` for now. The AppPreferences Security type has
// a field `structure_configuration_references` but no client can populate it yet, so the list will always
// be empty, thus save to used a serializable trivial type such as `bool` as a sample for now.
#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct SecurityStructureConfigurationReference;
impl Identifiable for SecurityStructureConfigurationReference {
    type ID = Self;

    fn id(&self) -> Self::ID {
        *self
    }
}

/// Controls e.g. if Profile Snapshot gets synced to iCloud or not, and whether
/// developer mode is enabled or not. In future (MFA) we will also save a list of
/// MFA security structure configurations.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display(
    "cloud? {}, dev? {}",
    is_cloud_profile_sync_enabled,
    is_developer_mode_enabled
)]
pub struct Security {
    pub is_cloud_profile_sync_enabled: bool,
    pub is_developer_mode_enabled: bool,
    pub structure_configuration_references:
        IdentifiedVecOf<SecurityStructureConfigurationReference>,
}

impl Security {
    /// Instantiates a new AppPreferences Security configuration.
    pub fn new(
        is_cloud_profile_sync_enabled: bool,
        is_developer_mode_enabled: bool,
        structure_configuration_references: IdentifiedVecOf<
            SecurityStructureConfigurationReference,
        >,
    ) -> Self {
        Self {
            is_cloud_profile_sync_enabled,
            is_developer_mode_enabled,
            structure_configuration_references,
        }
    }
}

impl Default for Security {
    /// By default we cloud profile sync is enabled and developer mode is disabled, with an empty `structure_configuration_references` list.
    fn default() -> Self {
        Self::new(true, false, IdentifiedVecOf::new())
    }
}

impl HasSampleValues for Security {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(true, true, IdentifiedVecOf::new())
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::new(false, false, IdentifiedVecOf::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(Security::sample(), Security::sample());
        assert_eq!(Security::sample_other(), Security::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Security::sample(), Security::sample_other());
    }

    #[test]
    fn default_developer_mode_is_enabled() {
        assert!(Security::default().is_cloud_profile_sync_enabled);
    }

    #[test]
    fn default_is_developer_mode_disabled() {
        assert!(!Security::default().is_developer_mode_enabled);
    }

    #[test]
    fn default_structure_configuration_references_is_empty() {
        assert!(Security::default()
            .structure_configuration_references
            .is_empty());
    }

    #[test]
    fn json_roundtrip() {
        let sut = Security::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "isCloudProfileSyncEnabled": true,
                "structureConfigurationReferences": [],
                "isDeveloperModeEnabled": true
            }
            "#,
        )
    }
}
