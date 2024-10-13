use crate::prelude::*;
use sargon::TokenDefinitionMetadata as InternalTokenDefinitionMetadata;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct TokenDefinitionMetadata {
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub icon_url: String,
    pub tags: Vec<String>,
}
