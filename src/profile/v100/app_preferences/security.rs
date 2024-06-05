use crate::prelude::*;

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
    #[serde(default)]
    pub security_structures_of_factor_source_ids:
        SecurityStructuresOfFactorSourceIDs,
}

impl Security {
    /// Instantiates a new AppPreferences Security configuration.
    pub fn new(
        is_cloud_profile_sync_enabled: bool,
        is_developer_mode_enabled: bool,
        security_structures_of_factor_source_ids: SecurityStructuresOfFactorSourceIDs,
    ) -> Self {
        Self {
            is_cloud_profile_sync_enabled,
            is_developer_mode_enabled,
            security_structures_of_factor_source_ids,
        }
    }
}

impl Default for Security {
    /// By default we cloud profile sync is enabled and developer mode is disabled, with an empty `structure_configuration_references` list.
    fn default() -> Self {
        Self::new(true, false, SecurityStructuresOfFactorSourceIDs::new())
    }
}

impl HasSampleValues for Security {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(true, true, SecurityStructuresOfFactorSourceIDs::new())
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::new(false, false, SecurityStructuresOfFactorSourceIDs::new())
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
    fn default_security_structures_of_factor_source_ids_is_empty() {
        assert!(Security::default()
            .security_structures_of_factor_source_ids
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
                "securityStructuresOfFactorSourceIDs": [],
                "isDeveloperModeEnabled": true
            }
            "#,
        )
    }
}
