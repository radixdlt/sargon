use crate::prelude::*;
use sargon::TokenDefinitionMetadata as InternalTokenDefinitionMetadata;

#[derive(
    Clone,  PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
pub struct TokenDefinitionMetadata {
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub icon_url: String,
    pub tags: Vec<String>,
}

impl From<InternalTokenDefinitionMetadata> for TokenDefinitionMetadata {
    fn from(value: InternalTokenDefinitionMetadata) -> Self {
        Self {
            name: value.name,
            description: value.description,
            symbol: value.symbol,
            icon_url: value.icon_url,
            tags: value.tags,
        }
    }
}

impl Into<InternalTokenDefinitionMetadata> for TokenDefinitionMetadata {
    fn into(self) -> InternalTokenDefinitionMetadata {
        InternalTokenDefinitionMetadata {
            name: self.name,
            description: self.description,
            symbol: self.symbol,
            icon_url: self.icon_url,
            tags: self.tags,
        }
    }
}