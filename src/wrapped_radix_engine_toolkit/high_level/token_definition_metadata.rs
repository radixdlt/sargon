use crate::prelude::*;
use radix_common::prelude::NodeId;
use radix_common::time::Instant;
use radix_common::types::GlobalAddress;
use radix_engine_interface::prelude::UncheckedOrigin;

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
            (
                "extra_String".to_string(),
                ScryptoMetadataValue::String("foo bar".to_string()),
            ),
            ("extra_bool".to_string(), ScryptoMetadataValue::Bool(true)),
            ("extra_u8".to_string(), ScryptoMetadataValue::U8(8)),
            ("extra_u32".to_string(), ScryptoMetadataValue::U32(32)),
            ("extra_u64".to_string(), ScryptoMetadataValue::U64(64)),
            ("extra_i32".to_string(), ScryptoMetadataValue::I32(32)),
            ("extra_i64".to_string(), ScryptoMetadataValue::I64(64)),
            (
                "extra_Decimal".to_string(),
                ScryptoMetadataValue::Decimal(Decimal::eight().into()),
            ),
            (
                "extra_GlobalAddress".to_string(),
                ScryptoMetadataValue::GlobalAddress(GlobalAddress::from(
                    AccountAddress::sample(),
                )),
            ),
            (
                "extra_PublicKey".to_string(),
                ScryptoMetadataValue::PublicKey(PublicKey::sample().into()),
            ),
            (
                "extra_NonFungibleGlobalId".to_string(),
                ScryptoMetadataValue::NonFungibleGlobalId(
                    NonFungibleGlobalId::sample().into(),
                ),
            ),
            (
                "extra_NonFungibleLocalId".to_string(),
                ScryptoMetadataValue::NonFungibleLocalId(
                    NonFungibleLocalId::sample().into(),
                ),
            ),
            (
                "extra_Instant".to_string(),
                ScryptoMetadataValue::Instant(Instant::new(1891)),
            ),
            (
                "extra_Url".to_string(),
                ScryptoMetadataValue::Url(ScryptoUncheckedUrl::of(
                    "https://radixdlt.com",
                )),
            ),
            (
                "extra_Origin".to_string(),
                ScryptoMetadataValue::Origin(UncheckedOrigin::of(
                    "https://radixdlt.com",
                )),
            ),
            (
                "extra_PublicKeyHash".to_string(),
                ScryptoMetadataValue::PublicKeyHash(
                    PublicKeyHash::sample().into(),
                ),
            ),
            (
                "extra_StringArray".to_string(),
                ScryptoMetadataValue::StringArray(vec![
                    "foo".to_string(),
                    "bar".to_string(),
                ]),
            ),
            (
                "extra_BoolArray".to_string(),
                ScryptoMetadataValue::BoolArray(vec![true, false]),
            ),
            (
                "extra_U8Array".to_string(),
                ScryptoMetadataValue::U8Array(vec![8, 9, 10, 11]),
            ),
            (
                "extra_U32Array".to_string(),
                ScryptoMetadataValue::U32Array(vec![32, 33, 34, 35]),
            ),
            (
                "extra_U64Array".to_string(),
                ScryptoMetadataValue::U64Array(vec![64, 65, 66, 67]),
            ),
            (
                "extra_I32Array".to_string(),
                ScryptoMetadataValue::I32Array(vec![32, 33, 34, 35]),
            ),
            (
                "extra_I64Array".to_string(),
                ScryptoMetadataValue::I64Array(vec![64, 65, 66, 67]),
            ),
            (
                "extra_DecimalArray".to_string(),
                ScryptoMetadataValue::DecimalArray(vec![
                    Decimal::one().into(),
                    Decimal::two().into(),
                ]),
            ),
            (
                "extra_GlobalAddressArray".to_string(),
                ScryptoMetadataValue::GlobalAddressArray(vec![
                    GlobalAddress::from(AccountAddress::sample()),
                    GlobalAddress::from(AccountAddress::sample_other()),
                ]),
            ),
            (
                "extra_PublicKeyArray".to_string(),
                ScryptoMetadataValue::PublicKeyArray(vec![
                    PublicKey::sample().into(),
                    PublicKey::sample_other().into(),
                ]),
            ),
            (
                "extra_NonFungibleGlobalIdArray".to_string(),
                ScryptoMetadataValue::NonFungibleGlobalIdArray(vec![
                    NonFungibleGlobalId::sample().into(),
                    NonFungibleGlobalId::sample_other().into(),
                ]),
            ),
            (
                "extra_NonFungibleLocalIdArray".to_string(),
                ScryptoMetadataValue::NonFungibleLocalIdArray(vec![
                    NonFungibleLocalId::sample().into(),
                    NonFungibleLocalId::sample_other().into(),
                ]),
            ),
            (
                "extra_InstantArray".to_string(),
                ScryptoMetadataValue::InstantArray(vec![
                    Instant::new(5),
                    Instant::new(1891),
                ]),
            ),
            (
                "extra_UrlArray".to_string(),
                ScryptoMetadataValue::UrlArray(vec![
                    ScryptoUncheckedUrl::of("https://radixdlt.com"),
                    ScryptoUncheckedUrl::of("https://ociswap.com"),
                ]),
            ),
            (
                "extra_OriginArray".to_string(),
                ScryptoMetadataValue::OriginArray(vec![
                    UncheckedOrigin::of("https://radixdlt.com"),
                    UncheckedOrigin::of("https://ociswap.com"),
                ]),
            ),
            (
                "extra_PublicKeyHashArray".to_string(),
                ScryptoMetadataValue::PublicKeyHashArray(vec![
                    PublicKeyHash::sample().into(),
                    PublicKeyHash::sample_other().into(),
                ]),
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
