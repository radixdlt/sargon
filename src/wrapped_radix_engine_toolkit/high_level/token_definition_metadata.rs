use crate::prelude::*;

use radix_engine::types::node_modules::ModuleConfig as ScryptoModuleConfig;
use radix_engine::types::{
    MetadataInit as ScryptoMetadataInit,
    RoleAssignmentInit as ScryptoRoleAssignmentInit,
};
use std::collections::BTreeMap;
use transaction::prelude::MetadataValue as ScryptoMetadataValue;

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct TokenDefinitionMetadata {
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub icon_url: String,
}
impl From<TokenDefinitionMetadata>
    for ScryptoModuleConfig<ScryptoMetadataInit>
{
    fn from(value: TokenDefinitionMetadata) -> Self {
        let map = BTreeMap::<String, ScryptoMetadataValue>::from([
            ("name".to_owned(), ScryptoMetadataValue::String(value.name)),
            (
                "symbol".to_owned(),
                ScryptoMetadataValue::String(value.symbol),
            ),
            (
                "description".to_owned(),
                ScryptoMetadataValue::String(value.description),
            ),
            (
                "icon_url".to_owned(),
                ScryptoMetadataValue::String(value.icon_url),
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
    ) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            description: description.as_ref().to_owned(),
            symbol: symbol.as_ref().to_owned(),
            icon_url: icon_url.as_ref().to_owned(),
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
            "https://uxwing.com/wp-content/themes/uxwing/download/arts-graphic-shapes/star-full-icon.png"
        )
    }

    fn non_fungible() -> Self {
        Self::new(
            "Heroes", 
            "An NFT collection of heroes", 
            "HEROES", 
            "https://uxwing.com/wp-content/themes/uxwing/download/crime-security-military-law/shield-black-icon.png"
        )
    }
}
