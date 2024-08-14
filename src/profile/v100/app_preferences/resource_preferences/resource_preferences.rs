use crate::prelude::*;
use core::hash::Hash;
use std::hash::Hasher;

/// User off-ledger preferences regarding resources.
#[derive(
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("{}", self.description())]
pub struct ResourcePreferences {
    /// A dictionary detailing the user preferences for fungible resources.
    #[serde(default)]
    pub fungible: HashMap<ResourceAddress, EntityFlags>,

    /// A dictionary detailing the user preferences for non-fungible resources.
    #[serde(default)]
    pub non_fungible: HashMap<NonFungibleGlobalId, EntityFlags>,

    /// A dictionary detailing the user preferences for pool units.
    #[serde(default)]
    pub pool_unit: HashMap<PoolAddress, EntityFlags>,
}

impl Hash for ResourcePreferences {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut fungible_pairs: Vec<_> = self.fungible.iter().collect();
        fungible_pairs.sort_by_key(|i| i.0);

        let mut non_fungible_pairs: Vec<_> = self.non_fungible.iter().collect();
        non_fungible_pairs.sort_by_key(|i| i.0);

        let mut pool_unit_pairs: Vec<_> = self.pool_unit.iter().collect();
        pool_unit_pairs.sort_by_key(|i| i.0);

        Hash::hash(&fungible_pairs, state);
        Hash::hash(&non_fungible_pairs, state);
        Hash::hash(&pool_unit_pairs, state);
    }
}

impl ResourcePreferences {
    pub fn new() -> Self {
        Self {
            fungible: HashMap::new(),
            non_fungible: HashMap::new(),
            pool_unit: HashMap::new(),
        }
    }

    pub fn description(&self) -> String {
        format!(
            r#"
            fungible: {:#?}
            non_fungible: {:#?}
            pool_unit: {:#?}
			"#,
            self.fungible, self.non_fungible, self.pool_unit,
        )
    }
}

impl HasSampleValues for ResourcePreferences {
    fn sample() -> Self {
        Self {
            fungible: [(ResourceAddress::sample(), EntityFlags::sample())]
                .into(),
            non_fungible: [(
                NonFungibleGlobalId::sample(),
                EntityFlags::sample(),
            )]
            .into(),
            pool_unit: [(PoolAddress::sample(), EntityFlags::sample())].into(),
        }
    }

    fn sample_other() -> Self {
        Self {
            fungible: [(
                ResourceAddress::sample_other(),
                EntityFlags::sample_other(),
            )]
            .into(),
            non_fungible: [(
                NonFungibleGlobalId::sample_other(),
                EntityFlags::sample_other(),
            )]
            .into(),
            pool_unit: [(
                PoolAddress::sample_other(),
                EntityFlags::sample_other(),
            )]
            .into(),
        }
    }
}

impl ResourcePreferences {
    pub fn get_hidden_resources(&self) -> HiddenResources {
        let fungible = self
            .fungible
            .iter()
            .filter(|(_, flags)| {
                flags.contains_by_id(&EntityFlag::DeletedByUser)
            })
            .map(|(resource, _)| *resource)
            .sorted()
            .collect();

        let non_fungible: Vec<NonFungibleGlobalId> = self
            .non_fungible
            .iter()
            .filter(|(_, flags)| {
                flags.contains_by_id(&EntityFlag::DeletedByUser)
            })
            .map(|(global_id, _)| global_id.clone())
            .sorted()
            .collect();

        let pool_unit = self
            .pool_unit
            .iter()
            .filter(|(_, flags)| {
                flags.contains_by_id(&EntityFlag::DeletedByUser)
            })
            .map(|(pool_address, _)| *pool_address)
            .sorted()
            .collect();

        HiddenResources {
            fungible,
            non_fungible,
            pool_unit,
        }
    }

    pub fn hide_resource(&mut self, kind: ResourcePreferenceKind) {
        match kind {
            ResourcePreferenceKind::Fungible(value) => {
                self.fungible
                    .entry(value)
                    .or_default()
                    .insert(EntityFlag::DeletedByUser);
            }
            ResourcePreferenceKind::NonFungible(value) => {
                self.non_fungible
                    .entry(value)
                    .or_default()
                    .insert(EntityFlag::DeletedByUser);
            }
            ResourcePreferenceKind::PoolUnit(value) => {
                self.pool_unit
                    .entry(value)
                    .or_default()
                    .insert(EntityFlag::DeletedByUser);
            }
        }
    }

    pub fn unhide_resource(&mut self, kind: ResourcePreferenceKind) {
        match kind {
            ResourcePreferenceKind::Fungible(value) => {
                if let Some(flags) = self.fungible.get_mut(&value) {
                    flags.remove_flag(&EntityFlag::DeletedByUser);
                }
            }
            ResourcePreferenceKind::NonFungible(value) => {
                if let Some(flags) = self.non_fungible.get_mut(&value) {
                    flags.remove_flag(&EntityFlag::DeletedByUser);
                }
            }
            ResourcePreferenceKind::PoolUnit(value) => {
                if let Some(flags) = self.pool_unit.get_mut(&value) {
                    flags.remove_flag(&EntityFlag::DeletedByUser);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourcePreferences;

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
    fn hidden_resources() {
        use crate::ResourcePreferenceKind::*;
        let mut sut = SUT::new();

        // Test with no resources are hidden
        let mut result = sut.get_hidden_resources();
        assert!(result.fungible.is_empty());
        assert!(result.non_fungible.is_empty());
        assert!(result.pool_unit.is_empty());

        // Test with some fungible resources hidden
        let fungible_one = ResourceAddress::sample_other();
        let fungible_two = ResourceAddress::sample();
        sut.hide_resource(Fungible(fungible_one));
        sut.hide_resource(Fungible(fungible_two));

        result = sut.get_hidden_resources();
        assert_eq!(vec![fungible_one, fungible_two], result.fungible);
        assert!(result.non_fungible.is_empty());
        assert!(result.pool_unit.is_empty());

        // Test hiding some non-fungible and pool unit, and unhiding one of the fungibles
        let non_fungible = NonFungibleGlobalId::sample();
        let pool_unit = PoolAddress::sample();
        sut.unhide_resource(Fungible(fungible_one));
        sut.hide_resource(NonFungible(non_fungible.clone()));
        sut.hide_resource(PoolUnit(pool_unit));

        result = sut.get_hidden_resources();
        assert_eq!(vec![fungible_two], result.fungible);
        assert_eq!(vec![non_fungible], result.non_fungible);
        assert_eq!(vec![pool_unit], result.pool_unit);
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "fungible": {
                    "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd": [
                        "deletedByUser"
                    ]
                },
                "nonFungible": {
                    "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>": [
                        "deletedByUser"
                    ]
                },
                "poolUnit": {
                    "pool_rdx1c5dkfdtdqvczcwzdyvzeuhddyha768p2q28erden533fty8h68ay6m": [
                        "deletedByUser"
                    ]
                }
            }            
            "#,
        )
    }
}
