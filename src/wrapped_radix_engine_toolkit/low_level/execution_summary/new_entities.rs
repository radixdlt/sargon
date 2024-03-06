use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NewEntities {
    pub metadata: HashMap<ResourceAddress, NewlyCreatedResource>,
}

impl NewEntities {
    pub fn new<I>(resources: I) -> Self
    where
        I: IntoIterator<Item = NewlyCreatedResource>,
    {
        Self {
            metadata: resources
                .into_iter()
                .map(|r| (r.resource_address.clone(), r))
                .collect::<HashMap<ResourceAddress, NewlyCreatedResource>>(),
        }
    }
}

impl HasSampleValues for NewEntities {
    fn sample() -> Self {
        Self::new([
            NewlyCreatedResource::sample_mainnet_xrd(),
            NewlyCreatedResource::sample_mainnet_candy(),
        ])
    }

    fn sample_other() -> Self {
        Self::new([
            NewlyCreatedResource::sample_stokenet_gc(),
            NewlyCreatedResource::sample_stokenet_gum(),
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
