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
        let resource_address = ResourceAddress::sample_stokenet_nft_abandon();
        let non_fungible_local_id = NonFungibleLocalId::Integer { value: 1 };
        let non_fungible_local_id_other =
            NonFungibleLocalId::Integer { value: 2 };
        let non_fungible_resource_address =
            NonFungibleResourceAddress::new(resource_address).unwrap();
        let non_fungible_global_id = NonFungibleGlobalId::new(
            non_fungible_resource_address,
            non_fungible_local_id.clone(),
        );
        let non_fungible_global_id_other = NonFungibleGlobalId::new(
            non_fungible_resource_address,
            non_fungible_local_id_other.clone(),
        );

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
                "extra_string".to_string(),
                ScryptoMetadataValue::String("foo bar".to_string()),
            ),
            ("extra_bool".to_string(), ScryptoMetadataValue::Bool(true)),
            ("extra_u8".to_string(), ScryptoMetadataValue::U8(8)),
            ("extra_u32".to_string(), ScryptoMetadataValue::U32(32)),
            ("extra_u64".to_string(), ScryptoMetadataValue::U64(64)),
            ("extra_i32".to_string(), ScryptoMetadataValue::I32(32)),
            ("extra_i64".to_string(), ScryptoMetadataValue::I64(64)),
            (
                "extra_decimal".to_string(),
                ScryptoMetadataValue::Decimal(Decimal::eight().into()),
            ),
            (
                "extra_global_address".to_string(),
                ScryptoMetadataValue::GlobalAddress(GlobalAddress::from(
                    AccountAddress::sample(),
                )),
            ),
            (
                "extra_public_key".to_string(),
                ScryptoMetadataValue::PublicKey(PublicKey::sample().into()),
            ),
            (
                "extra_non_fungible_global_id".to_string(),
                ScryptoMetadataValue::NonFungibleGlobalId(
                    non_fungible_global_id.clone().into(),
                ),
            ),
            (
                "extra_non_fungible_local_id".to_string(),
                ScryptoMetadataValue::NonFungibleLocalId(
                    non_fungible_local_id.clone().into(),
                ),
            ),
            (
                "extra_instant".to_string(),
                ScryptoMetadataValue::Instant(Instant::new(1891)),
            ),
            (
                "extra_url".to_string(),
                ScryptoMetadataValue::Url(ScryptoUncheckedUrl::of(
                    "https://radixdlt.com",
                )),
            ),
            (
                "extra_origin".to_string(),
                ScryptoMetadataValue::Origin(UncheckedOrigin::of(
                    "https://radixdlt.com",
                )),
            ),
            (
                "extra_public_key_hash".to_string(),
                ScryptoMetadataValue::PublicKeyHash(
                    PublicKeyHash::sample().into(),
                ),
            ),
            (
                "extra_string_array".to_string(),
                ScryptoMetadataValue::StringArray(vec![
                    "foo".to_string(),
                    "bar".to_string(),
                ]),
            ),
            (
                "extra_bool_array".to_string(),
                ScryptoMetadataValue::BoolArray(vec![true, false]),
            ),
            (
                "extra_u8_array".to_string(),
                ScryptoMetadataValue::U8Array(vec![8, 9, 10, 11]),
            ),
            (
                "extra_u32_array".to_string(),
                ScryptoMetadataValue::U32Array(vec![32, 33, 34, 35]),
            ),
            (
                "extra_u64_array".to_string(),
                ScryptoMetadataValue::U64Array(vec![64, 65, 66, 67]),
            ),
            (
                "extra_i32_array".to_string(),
                ScryptoMetadataValue::I32Array(vec![32, 33, 34, 35]),
            ),
            (
                "extra_i64_array".to_string(),
                ScryptoMetadataValue::I64Array(vec![64, 65, 66, 67]),
            ),
            (
                "extra_decimal_array".to_string(),
                ScryptoMetadataValue::DecimalArray(vec![
                    Decimal::one().into(),
                    Decimal::two().into(),
                ]),
            ),
            (
                "extra_global_address_array".to_string(),
                ScryptoMetadataValue::GlobalAddressArray(vec![
                    GlobalAddress::from(AccountAddress::sample_stokenet()),
                    GlobalAddress::from(AccountAddress::sample_stokenet_other()),
                ]),
            ),
            (
                "extra_public_key_array".to_string(),
                ScryptoMetadataValue::PublicKeyArray(vec![
                    PublicKey::sample().into(),
                    PublicKey::sample_other().into(),
                ]),
            ),
            (
                "extra_non_fungible_global_id_array".to_string(),
                ScryptoMetadataValue::NonFungibleGlobalIdArray(vec![
                    non_fungible_global_id.clone().into(),
                    non_fungible_global_id_other.clone().into(),
                ]),
            ),
            (
                "extra_non_fungible_local_id_array".to_string(),
                ScryptoMetadataValue::NonFungibleLocalIdArray(vec![
                    non_fungible_local_id.clone().into(),
                    non_fungible_local_id_other.clone().into(),
                ]),
            ),
            (
                "extra_instant_array".to_string(),
                ScryptoMetadataValue::InstantArray(vec![
                    Instant::new(5),
                    Instant::new(1891),
                ]),
            ),
            (
                "extra_url_array".to_string(),
                ScryptoMetadataValue::UrlArray(vec![
                    ScryptoUncheckedUrl::of("https://radixdlt.com"),
                    ScryptoUncheckedUrl::of("https://ociswap.com"),
                ]),
            ),
            (
                "extra_origin_array".to_string(),
                ScryptoMetadataValue::OriginArray(vec![
                    UncheckedOrigin::of("https://radixdlt.com"),
                    UncheckedOrigin::of("https://ociswap.com"),
                ]),
            ),
            (
                "extra_public_key_hash_array".to_string(),
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
