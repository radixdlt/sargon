use crate::prelude::*;
use radix_engine::prelude::ToMetadataEntry as ScryptoToMetadataEntry;
use radix_engine::types::{
    node_modules::ModuleConfig as ScryptoModuleConfig,
    AccessRule as ScryptoAccessRule,
    FungibleResourceRoles as ScryptoFungibleResourceRoles,
    GlobalAddress as ScryptoGlobalAddress,
    ManifestEncode as ScryptoManifestEncode,
    MetadataInit as ScryptoMetadataInit,
    NonFungibleData as ScryptoNonFungibleData,
    NonFungibleIdType as ScryptoNonFungibleIdType,
    NonFungibleResourceRoles as ScryptoNonFungibleResourceRoles,
    OwnerRole as ScryptoOwnerRole,
    RoleAssignmentInit as ScryptoRoleAssignmentInit,
};

use radix_engine_common::math::Decimal as ScryptoDecimal;
use radix_engine_common::prelude::NonFungibleLocalId as ScryptoNonFungibleLocalId;
use radix_engine_derive::{ManifestSbor as ScryptoManifestSbor, ScryptoSbor};
use radix_engine_toolkit::models::node_id::TypedNodeId as RetTypedNodeId;

use std::collections::BTreeMap;
use std::fs;

use transaction::{
    builder::ResolvableComponentAddress as ScryptoResolvableComponentAddress,
    model::DynamicGlobalAddress as ScryptoDynamicGlobalAddress,
    prelude::{
        ManifestBuilder as ScryptoManifestBuilder,
        MetadataValue as ScryptoMetadataValue,
        TransactionManifestV1 as ScryptoTransactionManifest,
    },
};
impl TransactionManifest {
    pub fn create_fungible_token(address_of_owner: &AccountAddress) -> Self {
        Self::create_fungible_token_with_metadata(
            address_of_owner,
            21_000_000.into(),
            TokenDefinitionMetadata::sample(),
        )
    }

    pub fn create_fungible_token_with_metadata(
        address_of_owner: &AccountAddress,
        initial_supply: Decimal192,
        metadata: TokenDefinitionMetadata,
    ) -> Self {
        let mut builder = ScryptoManifestBuilder::new();
        builder = Self::create_fungible_token_with_metadata_without_deposit(
            builder,
            initial_supply,
            metadata,
        );
        let scrypto_manifest = builder
            .try_deposit_entire_worktop_or_abort(
                address_of_owner.scrypto(),
                None,
            )
            .build();

        TransactionManifest::from_scrypto(
            scrypto_manifest,
            address_of_owner.network_id(),
        )
    }

    pub fn create_non_fungible_token(
        address_of_owner: &AccountAddress,
    ) -> Self {
        let count = 10;

        #[derive(Clone, PartialEq, Eq, ScryptoSbor, ScryptoManifestSbor)]
        pub struct NfData {
            pub name: String,
        }
        impl NfData {
            fn new(i: u64) -> Self {
                Self {
                    name: format!("nf-number-{}", i),
                }
            }
        }
        impl ScryptoNonFungibleData for NfData {
            const MUTABLE_FIELDS: &'static [&'static str] = &["name"];
        }

        Self::create_non_fungible_tokens(
            address_of_owner,
            (0..count).map(|i| {
                (NonFungibleLocalId::Integer { value: i }, NfData::new(i))
            }),
        )
    }

    fn create_non_fungible_tokens<T, V>(
        address_of_owner: &AccountAddress,
        initial_supply: T,
    ) -> Self
    where
        T: IntoIterator<Item = (NonFungibleLocalId, V)>,
        V: ScryptoManifestEncode + ScryptoNonFungibleData,
    {
        let scrypto_manifest = ScryptoManifestBuilder::new()
            .create_non_fungible_resource(
                ScryptoOwnerRole::None,
                ScryptoNonFungibleIdType::Integer,
                true,
                ScryptoNonFungibleResourceRoles::single_locked_rule(
                    ScryptoAccessRule::DenyAll,
                ),
                TokenDefinitionMetadata::sample().into(),
                Some(
                    initial_supply
                        .into_iter()
                        .map(|t| {
                            (Into::<ScryptoNonFungibleLocalId>::into(t.0), t.1)
                        })
                        .collect::<Vec<(ScryptoNonFungibleLocalId, V)>>(),
                ),
            )
            .try_deposit_entire_worktop_or_abort(
                address_of_owner.scrypto(),
                None,
            )
            .build();

        TransactionManifest::from_scrypto(
            scrypto_manifest,
            address_of_owner.network_id(),
        )
    }

    pub fn create_fungible_token_with_metadata_without_deposit(
        builder: ScryptoManifestBuilder,
        initial_supply: Decimal192,
        metadata: TokenDefinitionMetadata,
    ) -> ScryptoManifestBuilder {
        let initial_supply: ScryptoDecimal = initial_supply.into();
        builder.create_fungible_resource(
            ScryptoOwnerRole::None,
            true,
            10,
            ScryptoFungibleResourceRoles::single_locked_rule(
                ScryptoAccessRule::DenyAll,
            ),
            metadata.into(),
            Some(initial_supply),
        )
    }

    pub fn create_multiple_fungible_tokens(
        address_of_owner: &AccountAddress,
    ) -> TransactionManifest {
        if address_of_owner.network_id() == NetworkID::Mainnet {
            panic!("To be 100% sure about license of the images, we do not allow these sample fungible tokens to be created on Mainnet.");
        }
        let path = "src/wrapped_radix_engine_toolkit/high_level/sample_resource_definition_metadata.json";
        let json_str = fs::read_to_string(path).unwrap();
        let json = serde_json::Value::from_str(&json_str).unwrap();

        #[derive(Deserialize)]
        struct MultipleFungibleTokens {
            description: String,
            tokens: Vec<TokenDefinitionMetadata>,
        }

        let multiple_fungibles: MultipleFungibleTokens =
            serde_json::from_value(json).unwrap();
        info!("Generating multiple fungibles using bundled vector in file at '{}'\nDescription:\n'{}'", path, &multiple_fungibles.description);
        let fungibles = multiple_fungibles.tokens;

        let mut builder = ScryptoManifestBuilder::new();

        for metadata in fungibles.iter() {
            builder = Self::create_fungible_token_with_metadata_without_deposit(
                builder,
                21_000_000.into(),
                metadata.clone(),
            );
        }

        let scrypto_manifest = builder
            .try_deposit_entire_worktop_or_abort(
                address_of_owner.scrypto(),
                None,
            )
            .build();

        TransactionManifest::from_scrypto(
            scrypto_manifest,
            address_of_owner.network_id(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use rand::Rng;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn manifest_create_fungible_token_stella() {
        assert_eq!(
            SUT::create_fungible_token(
                &AccountAddress::sample_mainnet().into(),
            )
            .to_string(),
            r#"CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
    Enum<0u8>()
    true
    10u8
    Decimal("21000000")
    Tuple(
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<1u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<1u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<1u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<1u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<1u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        ),
        Enum<1u8>(
            Tuple(
                Enum<1u8>(
                    Enum<1u8>()
                ),
                Enum<1u8>(
                    Enum<1u8>()
                )
            )
        )
    )
    Tuple(
        Map<String, Tuple>(
            "description" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "The brightest component in the Radix ecosystem."
                    )
                ),
                false
            ),
            "icon_url" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "https://uxwing.com/wp-content/themes/uxwing/download/arts-graphic-shapes/star-full-icon.png"
                    )
                ),
                false
            ),
            "name" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "Stella"
                    )
                ),
                false
            ),
            "symbol" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "STAR"
                    )
                ),
                false
            )
        ),
        Map<String, Enum>()
    )
    Enum<0u8>()
;
CALL_METHOD
    Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
    "try_deposit_batch_or_abort"
    Expression("ENTIRE_WORKTOP")
    Enum<0u8>()
;
"#
        );
    }

    #[test]
    fn create_multiple_fungible_tokens() {
        let manifest = TransactionManifest::create_multiple_fungible_tokens(
            &AccountAddress::sample_stokenet(),
        );
        assert_eq!(manifest.instructions().len(), 26);
    }

    #[test]
    #[should_panic(
        expected = "To be 100% sure about license of the images, we do not allow these sample fungible tokens to be created on Mainnet."
    )]
    fn create_multiple_fungible_tokens_panics_for_mainnet() {
        TransactionManifest::create_multiple_fungible_tokens(
            &AccountAddress::sample_mainnet(),
        );
    }

    #[test]
    fn create_non_fungible_token() {
        let manifest = TransactionManifest::create_non_fungible_token(
            &AccountAddress::sample_stokenet(),
        );
        assert_eq!(manifest.instructions().len(), 2);
        assert_eq!(manifest.to_string().len(), 5048);
    }
}
