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
#[display("{:#?}", self.resource_flags)]
pub struct ResourcePreferences {
    /// A dictionary detailing the user preferences for each resource.
    #[serde(default)]
    pub resource_flags: HashMap<ResourceAddress, EntityFlags>,
}

impl Hash for ResourcePreferences {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.resource_flags.iter().collect();
        pairs.sort_by_key(|i| i.0);

        Hash::hash(&pairs, state);
    }
}

impl ResourcePreferences {
    pub fn new() -> Self {
        Self {
            resource_flags: HashMap::new(),
        }
    }
}

impl HasSampleValues for ResourcePreferences {
    fn sample() -> Self {
        Self {
            resource_flags: [(
                ResourceAddress::sample(),
                EntityFlags::sample(),
            )]
            .into(),
        }
    }

    fn sample_other() -> Self {
        Self {
            resource_flags: [(
                ResourceAddress::sample_other(),
                EntityFlags::sample_other(),
            )]
            .into(),
        }
    }
}

impl ResourcePreferences {
    pub fn get_hidden_resources(&self) -> Vec<ResourceAddress> {
        self.resource_flags
            .iter()
            .filter(|(_, flags)| {
                flags.contains_by_id(&EntityFlag::DeletedByUser)
            })
            .map(|(resource, _)| *resource)
            .sorted()
            .collect()
    }

    pub fn is_resource_hidden(&self, resource: ResourceAddress) -> bool {
        match self.resource_flags.get(&resource) {
            Some(flags) => flags.contains_by_id(&EntityFlag::DeletedByUser),
            None => false,
        }
    }

    pub fn hide_resource(&mut self, resource: ResourceAddress) {
        self.resource_flags
            .entry(resource)
            .or_default()
            .insert(EntityFlag::DeletedByUser);
    }

    pub fn unhide_resource(&mut self, resource: ResourceAddress) {
        if let Some(flags) = self.resource_flags.get_mut(&resource) {
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
                "resourceFlags": {
                    "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd": [
                        "deletedByUser"
                    ]
                }
            }            
            "#,
        )
    }
}
