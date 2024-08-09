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
#[display("{:#?}", self.flags)]
pub struct ResourcePreferences {
    /// A dictionary detailing the user preferences for each resource.
    #[serde(default)]
    pub flags: HashMap<ResourceAddress, EntityFlags>,
}

impl Hash for ResourcePreferences {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.flags.iter().collect();
        pairs.sort_by_key(|i| i.0);

        Hash::hash(&pairs, state);
    }
}

impl HasSampleValues for ResourcePreferences {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self {
            flags: [(ResourceAddress::sample(), EntityFlags::sample())].into(),
        }
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self {
            flags: [(
                ResourceAddress::sample_other(),
                EntityFlags::sample_other(),
            )]
            .into(),
        }
    }
}
