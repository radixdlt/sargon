use std::collections::BTreeSet;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

// FIXME: MFA this is in fact not used, so ok to be a `bool` for now. The AppPreferences Security type has
// a field `structure_configuration_references` but no client can populate it yet, so the list will always
// be empty, thus save to used a serializable trivial type such as `bool` as a placeholder for now.
pub type SecurityStructureConfigurationReference = bool;

/// Controls e.g. if Profile Snapshot gets synced to iCloud or not, and whether
/// developer mode is enabled or not. In future (MFA) we will also save a list of
/// MFA security structure configurations.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct Security {
    pub is_cloud_profile_sync_enabled: bool,
    pub is_developer_mode_enabled: bool,
    pub structure_configuration_references: Vec<SecurityStructureConfigurationReference>, // FIXME: Change to Set once UniFFI supports sets
}

impl Security {
    /// Instantiates a new AppPreferences Security configuration.
    pub fn new(
        is_cloud_profile_sync_enabled: bool,
        is_developer_mode_enabled: bool,
        structure_configuration_references: BTreeSet<SecurityStructureConfigurationReference>,
    ) -> Self {
        Self {
            is_cloud_profile_sync_enabled,
            is_developer_mode_enabled,
            structure_configuration_references: structure_configuration_references
                .into_iter()
                .collect_vec(),
        }
    }
}

impl Default for Security {
    /// By default we cloud profile sync is enabled and developer mode is disabled, with an empty `structure_configuration_references` list.
    fn default() -> Self {
        Self::new(true, false, BTreeSet::new())
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for Security {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new(true, true, BTreeSet::new())
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::new(false, false, BTreeSet::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::Security;

    #[test]
    fn equality() {
        assert_eq!(Security::placeholder(), Security::placeholder());
        assert_eq!(Security::placeholder_other(), Security::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Security::placeholder(), Security::placeholder_other());
    }

    #[test]
    fn default_developer_mode_is_enabled() {
        assert!(Security::default().is_cloud_profile_sync_enabled);
    }

    #[test]
    fn default_is_developer_mode_disabled() {
        assert_eq!(Security::default().is_developer_mode_enabled, false);
    }

    #[test]
    fn default_structure_configuration_references_is_empty() {
        assert!(Security::default()
            .structure_configuration_references
            .is_empty());
    }

    #[test]
    fn json_roundtrip() {
        let sut = Security::placeholder();
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
