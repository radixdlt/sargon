use core_misc::decl_bool_type;

use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`SecurityStructureOfFactorSourceIDs`]
    SecurityStructuresOfFactorSourceIDs,
    SecurityStructureOfFactorSourceIDs
);

impl HasSampleValues for SecurityStructuresOfFactorSourceIDs {
    fn sample() -> Self {
        Self::from_iter([
            SecurityStructureOfFactorSourceIDs::sample(),
            SecurityStructureOfFactorSourceIDs::sample_other(),
        ])
    }
    fn sample_other() -> Self {
        Self::from_iter([SecurityStructureOfFactorSourceIDs::sample_other()])
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
)]
#[serde(rename_all = "camelCase")]
#[display(
    "cloud? {}, dev? {}, advanced lock? {}",
    is_cloud_profile_sync_enabled,
    is_developer_mode_enabled,
    is_advanced_lock_enabled
)]
pub struct Security {
    pub is_cloud_profile_sync_enabled: IsCloudProfileSyncEnabled,
    pub is_developer_mode_enabled: IsDeveloperModeEnabled,

    #[serde(default)]
    pub is_advanced_lock_enabled: IsAdvancedLockEnabled,

    #[serde(rename = "securityStructuresOfFactorSourceIDs")]
    #[serde(default)]
    pub security_structures_of_factor_source_ids:
        SecurityStructuresOfFactorSourceIDs,
}

decl_bool_type!(IsCloudProfileSyncEnabled, true);
decl_bool_type!(IsDeveloperModeEnabled, false);
decl_bool_type!(IsAdvancedLockEnabled, false);

impl Security {
    /// Instantiates a new AppPreferences Security configuration.
    pub fn new(
        is_cloud_profile_sync_enabled: IsCloudProfileSyncEnabled,
        is_developer_mode_enabled: IsDeveloperModeEnabled,
        is_advanced_lock_enabled: IsAdvancedLockEnabled,
        security_structures_of_factor_source_ids: SecurityStructuresOfFactorSourceIDs,
    ) -> Self {
        Self {
            is_cloud_profile_sync_enabled,
            is_developer_mode_enabled,
            is_advanced_lock_enabled,
            security_structures_of_factor_source_ids,
        }
    }
}

impl Default for Security {
    /// By default cloud profile sync is enabled, while developer mode and avdanced lock is disabled, with an empty `structure_configuration_references` list.
    fn default() -> Self {
        Self::new(
            IsCloudProfileSyncEnabled::default(),
            IsDeveloperModeEnabled::default(),
            IsAdvancedLockEnabled::default(),
            SecurityStructuresOfFactorSourceIDs::new(),
        )
    }
}

impl HasSampleValues for Security {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(
            IsCloudProfileSyncEnabled::sample(),
            IsDeveloperModeEnabled::sample(),
            IsAdvancedLockEnabled::sample(),
            SecurityStructuresOfFactorSourceIDs::new(),
        )
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::new(
            IsCloudProfileSyncEnabled::sample_other(),
            IsDeveloperModeEnabled::sample_other(),
            IsAdvancedLockEnabled::sample_other(),
            SecurityStructuresOfFactorSourceIDs::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Security;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn default_is_cloud_profile_sync_enabled() {
        assert!(SUT::default().is_cloud_profile_sync_enabled.0);
    }

    #[test]
    fn default_is_developer_mode_disabled() {
        assert!(!SUT::default().is_developer_mode_enabled.0);
    }

    #[test]
    fn default_is_advanced_lock_disabled() {
        assert!(!SUT::default().is_advanced_lock_enabled.0);
    }

    #[test]
    fn default_security_structures_of_factor_source_ids_is_empty() {
        assert!(SUT::default()
            .security_structures_of_factor_source_ids
            .is_empty());
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "isCloudProfileSyncEnabled": true,
                "securityStructuresOfFactorSourceIDs": [],
                "isDeveloperModeEnabled": false,
                "isAdvancedLockEnabled": false
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_with_security_structures() {
        let mut sut = SUT::sample();

        sut.security_structures_of_factor_source_ids =
            SecurityStructuresOfFactorSourceIDs::sample();

        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "isCloudProfileSyncEnabled": true,
                "isDeveloperModeEnabled": false,
                "isAdvancedLockEnabled": false,
                "securityStructuresOfFactorSourceIDs": [
                    {
                        "metadata": {
                            "id": "ffffffff-ffff-ffff-ffff-ffffffffffff",
                            "displayName": "Spending Account",
                            "createdOn": "2023-09-11T16:05:56.000Z",
                            "lastUpdatedOn": "2023-09-11T16:05:56.000Z",
                            "flags": ["main"]
                        },
                        "authenticationSigningFactor": {
                            "discriminator": "fromHash",
                            "fromHash": {
                                "kind": "device",
                                "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                            }
                        },
                        "matrixOfFactors": {
                            "primaryRole": {
                                "threshold": "all",
                                "thresholdFactors": [
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "device",
                                            "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                                        }
                                    },
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "ledgerHQHardwareWallet",
                                            "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
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
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "device",
                                            "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                                        }
                                    },
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "ledgerHQHardwareWallet",
                                            "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                                        }
                                    }
                                ]
                            },
                            "confirmationRole": {
                                "threshold": "all",
                                "thresholdFactors": [],
                                "overrideFactors": [
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "password",
                                            "body": "181ab662e19fac3ad9f08d5c673b286d4a5ed9cd3762356dc9831dc42427c1b9"
                                        }
                                    }
                                ]
                            },
                            "timeUntilDelayedConfirmationIsCallable": {
	                            "value": 2,
	                            "unit": "weeks"
                            }
                        }
                    },
                    {
                        "metadata": {
                            "id": "dededede-dede-dede-dede-dededededede",
                            "displayName": "Savings Account",
                            "createdOn": "2023-12-24T17:13:56.123Z",
                            "lastUpdatedOn": "2023-12-24T17:13:56.123Z",
                            "flags": []
                        },
                        "authenticationSigningFactor": {
                          "discriminator": "fromHash",
                          "fromHash": {
                            "kind": "ledgerHQHardwareWallet",
                            "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                          }
                        },
                        "matrixOfFactors": {
                            "primaryRole": {
                                "threshold": "all",
                                "thresholdFactors": [
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "device",
                                            "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
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
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "ledgerHQHardwareWallet",
                                            "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                                        }
                                    }
                                ]
                            },
                            "confirmationRole": {
                                "threshold": "all",
                                "thresholdFactors": [],
                                "overrideFactors": [
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "ledgerHQHardwareWallet",
                                            "body": "52ef052a0642a94279b296d6b3b17dedc035a7ae37b76c1d60f11f2725100077"
                                        }
                                    }
                                ]
                            },
                            "timeUntilDelayedConfirmationIsCallable": {
	                            "value": 2,
                            	"unit": "weeks"
                            }
                        }
                    }
                ]
            }
            "#,
        )
    }

    #[test]
    fn json_deserialize_without_security_structures_nor_advanced_lock() {
        let json = r#"
            {
                "isCloudProfileSyncEnabled": true,
                "isDeveloperModeEnabled": true
            }
            "#;

        let sut: SUT = serde_json::from_str(json).unwrap();
        assert!(sut.security_structures_of_factor_source_ids.is_empty());
        assert!(!sut.is_advanced_lock_enabled.0);
    }
}
