use crate::prelude::*;

/// Metadata about a newly created Resource
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NewlyCreatedResource {
    pub resource_address: ResourceAddress,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub description: Option<String>,
    pub icon_url: Option<Url>,
    pub tags: Vec<String>,
}

impl NewlyCreatedResource {
    pub fn new<I>(
        resource_address: ResourceAddress,
        name: impl Into<Option<String>>,
        symbol: impl Into<Option<String>>,
        description: impl Into<Option<String>>,
        icon_url: impl Into<Option<Url>>,
        tags: I,
    ) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        Self {
            resource_address,
            name: name.into(),
            symbol: symbol.into(),
            description: description.into(),
            icon_url: icon_url.into(),
            tags: tags.into_iter().collect_vec(),
        }
    }

    pub fn with<I>(
        resource_address: ResourceAddress,
        name: impl AsRef<str>,
        symbol: impl AsRef<str>,
        description: impl AsRef<str>,
        icon_url: impl AsRef<str>,
        tags: I,
    ) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        Self::new(
            resource_address,
            name.as_ref().to_owned(),
            symbol.as_ref().to_owned(),
            description.as_ref().to_owned(),
            icon_url.as_ref().parse().ok(),
            tags,
        )
    }
}

#[allow(unused)]
impl NewlyCreatedResource {
    pub(crate) fn sample_mainnet_xrd() -> Self {
        Self::with(
            ResourceAddress::sample_mainnet_xrd(),
            "Rad",
            "XRD",
            "The Radix Public Network's native token, used to pay the network's required transaction fees and to secure the network through staking to its validator nodes.",
            "https://assets.radixdlt.com/icons/icon-xrd-32x32.png",
            ["Official Radix".to_owned()],
        )
    }

    pub(crate) fn sample_mainnet_candy() -> Self {
        Self::with(
            ResourceAddress::sample_mainnet_candy(),
            "Candy",
            "CANDY",
            "Sweetest token on Radix",
            "https://gumball-club.radixdlt.com/assets/candy-token.png",
            ["Gumball Club Token".to_owned(), "Sweet".to_owned()],
        )
    }

    pub(crate) fn sample_stokenet_gc() -> Self {
        Self::with(
            ResourceAddress::sample_stokenet_gc_tokens(),
            "GC Tokens (GC)",
            "GC",
            "Only for use at the Gumball Club. Not legal tender.",
            "https://gumball-club.radixdlt.com/assets/gc-token.png",
            [],
        )
    }

    pub(crate) fn sample_stokenet_gum() -> Self {
        Self::with(
            ResourceAddress::sample_stokenet_gum(),
            "GC Gumballs (GUM)",
            "GUM",
            "Official Gumball Club gumballs, for those who are all out of bubblegum.",
            "https://stokenet-gumball-club.radixdlt.com/assets/gumball-token.png",
            [],
        )
    }
}

impl HasSampleValues for NewlyCreatedResource {
    fn sample() -> Self {
        Self::sample_mainnet_candy()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_gum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NewlyCreatedResource;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
