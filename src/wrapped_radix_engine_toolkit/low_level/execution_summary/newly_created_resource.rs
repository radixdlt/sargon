use radix_engine::types::IndexMap;

use crate::prelude::*;

/// Metadata about a newly created Resource
#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct NewlyCreatedResource {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub tags: Vec<String>,
}

impl From<IndexMap<String, Option<ScryptoMetadataValue>>>
    for NewlyCreatedResource
{
    fn from(value: IndexMap<String, Option<ScryptoMetadataValue>>) -> Self {
        let get_str = |k: MetadataKey| match value.get(&k.to_string()) {
            Some(Some(ScryptoMetadataValue::String(value))) => {
                Some(value.to_owned())
            }
            _ => None,
        };

        let get_str_arr = |k: MetadataKey| match value.get(&k.to_string()) {
            Some(Some(ScryptoMetadataValue::StringArray(value))) => {
                value.to_owned()
            }
            _ => Vec::new(),
        };

        let get_url = |k: MetadataKey| match value.get(&k.to_string()) {
            Some(Some(ScryptoMetadataValue::Url(value))) => {
                Some(value.to_owned().0)
            }
            // TODO: Should we do this? Fallback to `::String` like this?
            Some(Some(ScryptoMetadataValue::String(value))) => {
                Some(value.to_owned())
            }
            _ => None,
        };

        use MetadataKey::*;

        Self::new(
            get_str(Name),
            get_str(Symbol),
            get_str(Description),
            get_url(IconUrl),
            get_str_arr(Tags),
        )
    }
}

impl NewlyCreatedResource {
    pub fn new<I>(
        name: impl Into<Option<String>>,
        symbol: impl Into<Option<String>>,
        description: impl Into<Option<String>>,
        icon_url: impl Into<Option<String>>,
        tags: I,
    ) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        Self {
            name: name.into(),
            symbol: symbol.into(),
            description: description.into(),
            icon_url: icon_url.into(),
            tags: tags.into_iter().collect_vec(),
        }
    }

    /// This can happen for e.g. creation of a new Pool, actually, the metadata
    /// from RET execution summary DOES contain a "pool" key, with value being
    /// a PoolAddress, but it is not the same pool address as the resulting pool,
    /// since it is an ephemeral PREVIEWED NodeId...
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with<I>(
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
            name.as_ref().to_owned(),
            symbol.as_ref().to_owned(),
            description.as_ref().to_owned(),
            icon_url.as_ref().to_owned(),
            tags,
        )
    }
}

#[allow(unused)]
impl NewlyCreatedResource {
    pub(crate) fn sample_mainnet_xrd() -> Self {
        Self::with(
            "Rad",
            "XRD",
            "The Radix Public Network's native token, used to pay the network's required transaction fees and to secure the network through staking to its validator nodes.",
            "https://assets.radixdlt.com/icons/icon-xrd-32x32.png",
            ["Official Radix".to_owned()],
        )
    }

    pub(crate) fn sample_mainnet_candy() -> Self {
        Self::with(
            "Candy",
            "CANDY",
            "Sweetest token on Radix",
            "https://gumball-club.radixdlt.com/assets/candy-token.png",
            ["Gumball Club Token".to_owned(), "Sweet".to_owned()],
        )
    }

    pub(crate) fn sample_stokenet_gc() -> Self {
        Self::with(
            "GC Tokens (GC)",
            "GC",
            "Only for use at the Gumball Club. Not legal tender.",
            "https://gumball-club.radixdlt.com/assets/gc-token.png",
            [],
        )
    }

    pub(crate) fn sample_stokenet_gum() -> Self {
        Self::with(
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

    #[test]
    fn default_is_empty() {
        assert_eq!(SUT::empty(), SUT::default());
    }
}
