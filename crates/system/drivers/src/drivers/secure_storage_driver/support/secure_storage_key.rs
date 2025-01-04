use crate::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq, derive_more::Display)]
pub enum SecureStorageKey {
    HostID,
    DeviceFactorSourceMnemonic {
        factor_source_id: FactorSourceIDFromHash,
    },
    ProfileSnapshot {
        // Note:
        // `profile_id` is only meant to be used by the iOS Host for backward compatibility.
        // iOS Host stores multiple profiles in the secure storage uniquely identified by `profile_id`,
        // while Android Host stores only one profile in the secure storage.
        profile_id: ProfileID,
    },
}

impl PartialEq<SecureStorageKey> for SecureStorageKey {
    fn eq(&self, other: &SecureStorageKey) -> bool {
        match (self, other) {
            (SecureStorageKey::HostID, SecureStorageKey::HostID) => true,
            (
                SecureStorageKey::DeviceFactorSourceMnemonic {
                    factor_source_id: a,
                },
                SecureStorageKey::DeviceFactorSourceMnemonic {
                    factor_source_id: b,
                },
            ) => a == b,
            (
                SecureStorageKey::ProfileSnapshot { .. },
                SecureStorageKey::ProfileSnapshot { .. },
            ) => true, // Note: `profile_id` is not used for comparison, as it is only forwarded as additional payload to the iOS Host.
            _ => false,
        }
    }
}

impl Hash for SecureStorageKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            SecureStorageKey::HostID => {
                "host_id".hash(state);
            }
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id,
            } => {
                "device_factor_source".hash(state);
                factor_source_id.hash(state);
            }
            // Note: `profile_id` is not used for computing the hash, as it is only forwarded as additional payload to the iOS Host.
            SecureStorageKey::ProfileSnapshot { .. } => {
                "profile_snapshot".hash(state);
            }
        }
    }
}

impl SecureStorageKey {
    pub fn identifier(&self) -> String {
        format!(
            "secure_storage_key_{}",
            match self {
                SecureStorageKey::HostID => "host_id".to_owned(),
                SecureStorageKey::DeviceFactorSourceMnemonic {
                    factor_source_id,
                } => format!("device_factor_source_{}", factor_source_id),
                SecureStorageKey::ProfileSnapshot { .. } =>
                    "profile_snapshot".to_owned(),
            }
        )
    }
}

impl SecureStorageKey {
    pub fn load_profile_snapshot() -> Self {
        // This id will not be used to load the profile snapshot.
        // It is only a stub to conform to the SecureStorageKey definition.
        let dummy_id = ProfileID(Uuid::from_bytes([0x00; 16]));
        SecureStorageKey::ProfileSnapshot {
            profile_id: dummy_id,
        }
    }
}

impl HasSampleValues for SecureStorageKey {
    fn sample() -> Self {
        SecureStorageKey::HostID
    }

    fn sample_other() -> Self {
        SecureStorageKey::DeviceFactorSourceMnemonic {
            factor_source_id: FactorSourceIDFromHash::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn identifier() {
        assert_eq!(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: FactorSourceIDFromHash::sample()
            }
            .identifier(),
            "secure_storage_key_device_factor_source_device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
        );
        assert_eq!(
            SecureStorageKey::load_profile_snapshot().identifier(),
            "secure_storage_key_profile_snapshot"
        );
    }
}
