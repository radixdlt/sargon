use crate::prelude::*;

/// Information on the global entities created in the transaction.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

type GlobalEntitiesMetadata = IndexMap<
    ScryptoGlobalAddress,
    IndexMap<String, Option<ScryptoMetadataValue>>,
>;
type NewEntitiesWithMetadata =
    (IndexSet<ScryptoResourceAddress>, GlobalEntitiesMetadata);

impl From<(NewEntitiesWithMetadata, NetworkID)> for NewEntities {
    fn from(value: (NewEntitiesWithMetadata, NetworkID)) -> Self {
        let ((new_resources, global_metadata), network_id) = value;

        Self::new(new_resources.into_iter().map(|r| {
            let resource_address = ResourceAddress::from((r, network_id));
            let global_address = ScryptoGlobalAddress::from(resource_address);

            let newly_created_resource = global_metadata
                .get(&global_address)
                .map(|m| NewlyCreatedResource::from(m.clone()))
                .unwrap_or_default();

            (resource_address, newly_created_resource)
        }))
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
