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
                            "lastUpdatedOn": "2023-09-11T16:05:56.000Z"
                        },
                        "numberOfEpochsUntilAutoConfirmation": 4032,
                        "matrixOfFactors": {
                            "primaryRole": {
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
                                            "kind": "arculusCard",
                                            "body": "12f36554769cd96614776e6dbd5629825b8e87366eec5e515de32bb1ea153820"
                                        }
                                    },
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "offDeviceMnemonic",
                                            "body": "820122c9573768ab572b0c9fa492a45b7b451a2740291b3da908ad423d10e410"
                                        }
                                    }
                                ],
                                "threshold": 2,
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
                            "recoveryRole": {
                                "thresholdFactors": [
                                    {
                                        "discriminator": "fromAddress",
                                        "fromAddress": {
                                            "kind": "trustedContact",
                                            "body": "account_rdx1298d59ae3k94htjzpy2z6mx4436h98e5u4qpnwhek8lukv7lkfrank"
                                        }
                                    },
                                    {
                                        "discriminator": "fromAddress",
                                        "fromAddress": {
                                            "kind": "trustedContact",
                                            "body": "account_rdx128c4f8dnuvd73d2r3fl95ryfuavw5zjf8zr57hjw0qjagz7s7grace"
                                        }
                                    },
                                    {
                                        "discriminator": "fromAddress",
                                        "fromAddress": {
                                            "kind": "trustedContact",
                                            "body": "account_rdx12y0389ew2xn7w02d059hhye6t0mjzqxqyavsetyg2j3p3xqyepjudy"
                                        }
                                    }
                                ],
                                "threshold": 2,
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
                                "thresholdFactors": [],
                                "threshold": 0,
                                "overrideFactors": [
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "securityQuestions",
                                            "body": "aabc6041d95785ecfabe7d5ed5af259e20e4e3f5f95b16fdeca386bc75796b46"
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
                            }
                        }
                    },
                    {
                        "metadata": {
                            "id": "dededede-dede-dede-dede-dededededede",
                            "displayName": "Savings Account",
                            "createdOn": "2023-12-24T17:13:56.123Z",
                            "lastUpdatedOn": "2023-12-24T17:13:56.123Z"
                        },
                        "numberOfEpochsUntilAutoConfirmation": 8064,
                        "matrixOfFactors": {
                            "primaryRole": {
                                "thresholdFactors": [
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "device",
                                            "body": "5255999c65076ce9ced5a1881f1a621bba1ce3f1f68a61df462d96822a5190cd"
                                        }
                                    },
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "arculusCard",
                                            "body": "3ac064d4b40f78effe7037a12f3287efc67aa87af7c6a083738eae05e28dadaf"
                                        }
                                    },
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "offDeviceMnemonic",
                                            "body": "5c308b9c3e41912d4af4c5ff088e84877aac5de01c95f32dedd280d55a6d8262"
                                        }
                                    }
                                ],
                                "threshold": 2,
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
                            "recoveryRole": {
                                "thresholdFactors": [
                                    {
                                        "discriminator": "fromAddress",
                                        "fromAddress": {
                                            "kind": "trustedContact",
                                            "body": "account_rdx1284z0gpg9vnhevn7sytdncszc7ukcrycntg7zjktqvggmwe6ctrudy"
                                        }
                                    },
                                    {
                                        "discriminator": "fromAddress",
                                        "fromAddress": {
                                            "kind": "trustedContact",
                                            "body": "account_rdx129uc6rf5vmkj2gau7fgxlsqdg8008nca8yd57sxx4v67dyw7u0scar"
                                        }
                                    },
                                    {
                                        "discriminator": "fromAddress",
                                        "fromAddress": {
                                            "kind": "trustedContact",
                                            "body": "account_rdx12y7uww27s250g9d3d72ey9wdp5z78zpmq5la0r0wgw4fkf6y8eerdx"
                                        }
                                    }
                                ],
                                "threshold": 2,
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
                            "confirmationRole": {
                                "thresholdFactors": [],
                                "threshold": 0,
                                "overrideFactors": [
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "securityQuestions",
                                            "body": "bb0ac72196f748bba4ddf9c6d87c4e3ea939750e3a207f312653aa25f3f9c060"
                                        }
                                    },
                                    {
                                        "discriminator": "fromHash",
                                        "fromHash": {
                                            "kind": "ledgerHQHardwareWallet",
                                            "body": "52ef052a0642a94279b296d6b3b17dedc035a7ae37b76c1d60f11f2725100077"
                                        }
                                    }
                                ]
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
