use crate::prelude::*;

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct TokenDefinitionMetadata {
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub icon_url: String,
    pub tags: Vec<String>,
}

impl From<TokenDefinitionMetadata>
    for ScryptoModuleConfig<ScryptoMetadataInit>
{
    fn from(value: TokenDefinitionMetadata) -> Self {
        let map = BTreeMap::<String, ScryptoMetadataValue>::from([
            (
                MetadataKey::Name.to_string(),
                ScryptoMetadataValue::String(value.name),
            ),
            (
                MetadataKey::Symbol.to_string(),
                ScryptoMetadataValue::String(value.symbol),
            ),
            (
                MetadataKey::Description.to_string(),
                ScryptoMetadataValue::String(value.description),
            ),
            (
                MetadataKey::IconUrl.to_string(),
                ScryptoMetadataValue::Url(ScryptoUncheckedUrl(value.icon_url)),
            ),
            (
                MetadataKey::Tags.to_string(),
                ScryptoMetadataValue::StringArray(value.tags),
            ),
        ]);
        let init: ScryptoMetadataInit = map.into();
        ScryptoModuleConfig {
            init,
            roles: ScryptoRoleAssignmentInit::default(),
        }
    }
}

impl TokenDefinitionMetadata {
    pub fn new(
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        symbol: impl AsRef<str>,
        icon_url: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            description: description.as_ref().to_owned(),
            symbol: symbol.as_ref().to_owned(),
            icon_url: icon_url.as_ref().to_owned(),
            tags: tags.into_iter().collect(),
        }
    }
}

impl HasSampleValues for TokenDefinitionMetadata {
    fn sample() -> Self {
        Self::fungible()
    }

    fn sample_other() -> Self {
        Self::non_fungible()
    }
}

impl TokenDefinitionMetadata {
    fn fungible() -> Self {
        Self::new(
            "Stella", 
            "The brightest component in the Radix ecosystem.", 
            "STAR", 
            "https://uxwing.com/wp-content/themes/uxwing/download/arts-graphic-shapes/star-full-icon.png",
            ["Bright".to_string()],
        )
    }

    fn non_fungible() -> Self {
        Self::new(
            "Heroes", 
            "An NFT collection of heroes", 
            "HEROES", 
            "https://uxwing.com/wp-content/themes/uxwing/download/crime-security-military-law/shield-black-icon.png",
            ["Unique".to_string(), "FOMO".to_string()]
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TokenDefinitionMetadata;

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
