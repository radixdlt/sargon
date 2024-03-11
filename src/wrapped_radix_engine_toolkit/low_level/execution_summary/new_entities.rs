use crate::prelude::*;

use radix_engine_toolkit::transaction_types::NewEntities as RetNewEntities;

/// Information on the global entities created in the transaction.
#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct NewEntities {
    pub metadata: HashMap<ResourceAddress, NewlyCreatedResource>,
}

impl NewEntities {
    pub fn new<I>(resources: I) -> Self
    where
        I: IntoIterator<Item = (ResourceAddress, NewlyCreatedResource)>,
    {
        Self {
            metadata: resources
                .into_iter()
                .collect::<HashMap<ResourceAddress, NewlyCreatedResource>>(),
        }
    }
}

impl From<(RetNewEntities, NetworkID)> for NewEntities {
    fn from(value: (RetNewEntities, NetworkID)) -> Self {
        let (ret, network_id) = value;

        Self::new(
            // We map from `IndexMap<GlobalAddress, IndexMap<String, Option<MetadataValue>>>`
            // into: `HashMap<ResourceAddress, NewlyCreatedResource>`,
            // and "filter out" (ignore) any GlobalAddress that is not a ResourceAddress,
            // why? Since Radix Wallets actually only use the ResourceAddress...
            ret.metadata
                .into_iter()
                .filter_map(|(k, v)| {
                    // We only care about `ResourceAddress`, and ignore other address types.
                    TryInto::<ResourceAddress>::try_into((k, network_id))
                        .map(|a| (a, v))
                        .ok()
                })
                .map(|t| (t.0, NewlyCreatedResource::from(t.1))),
        )
    }
}

impl HasSampleValues for NewEntities {
    fn sample() -> Self {
        Self::new([
            (
                ResourceAddress::sample_mainnet_xrd(),
                NewlyCreatedResource::sample_mainnet_xrd(),
            ),
            (
                ResourceAddress::sample_mainnet_candy(),
                NewlyCreatedResource::sample_mainnet_candy(),
            ),
        ])
    }

    fn sample_other() -> Self {
        Self::new([
            (
                ResourceAddress::sample_stokenet_gc_tokens(),
                NewlyCreatedResource::sample_stokenet_gc(),
            ),
            (
                ResourceAddress::sample_stokenet_gum(),
                NewlyCreatedResource::sample_stokenet_gum(),
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NewEntities;

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
    fn resource_can_be_lookup_by_address() {
        assert_eq!(
            SUT::sample().metadata[&ResourceAddress::sample_mainnet_candy()],
            NewlyCreatedResource::sample_mainnet_candy()
        );

        assert_eq!(
            SUT::sample_other().metadata
                [&ResourceAddress::sample_stokenet_gum()],
            NewlyCreatedResource::sample_stokenet_gum()
        );
    }
}
