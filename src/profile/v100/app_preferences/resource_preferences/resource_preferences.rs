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
            non_fungible: [].into(),
            pool_unit: [].into(),
        }
    }

    fn sample_other() -> Self {
        Self {
            fungible: [(
                ResourceAddress::sample_other(),
                EntityFlags::sample_other(),
            )]
            .into(),
            non_fungible: [].into(),
            pool_unit: [].into(),
        }
    }
}

impl ResourcePreferences {
    pub fn get_hidden_resources(&self) -> Vec<ResourceAddress> {
        self.fungible
            .iter()
            .filter(|(_, flags)| {
                flags.contains_by_id(&EntityFlag::DeletedByUser)
            })
            .map(|(resource, _)| *resource)
            .sorted()
            .collect()
    }

    pub fn is_resource_hidden(&self, resource: ResourceAddress) -> bool {
        match self.fungible.get(&resource) {
            Some(flags) => flags.contains_by_id(&EntityFlag::DeletedByUser),
            None => false,
        }
    }

    pub fn hide_resource(&mut self, resource: ResourceAddress) {
        self.fungible
            .entry(resource)
            .or_default()
            .insert(EntityFlag::DeletedByUser);
    }

    pub fn unhide_resource(&mut self, resource: ResourceAddress) {
        if let Some(flags) = self.fungible.get_mut(&resource) {
            flags.remove_flag(&EntityFlag::DeletedByUser);
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
        let mut sut = SUT::new();
        assert!(sut.get_hidden_resources().is_empty());

        let resource_one = ResourceAddress::sample_other();
        let resource_two = ResourceAddress::sample();
        sut.hide_resource(resource_one);
        sut.hide_resource(resource_two);

        assert_eq!(
            vec![resource_one, resource_two],
            sut.get_hidden_resources()
        );
    }

    #[test]
    fn hide_unhide_resource() {
        let mut sut = SUT::new();
        let resource = ResourceAddress::sample();

        // Test the resource isn't hidden by default
        assert!(!sut.is_resource_hidden(resource));

        // Hide the resource
        sut.hide_resource(resource);
        assert!(sut.is_resource_hidden(resource));

        // Unhide the resource
        sut.unhide_resource(resource);
        assert!(!sut.is_resource_hidden(resource));
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
                }
            }            
            "#,
        )
    }
}
