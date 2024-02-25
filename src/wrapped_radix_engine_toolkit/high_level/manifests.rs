use crate::prelude::*;
use radix_engine::types::node_modules::ModuleConfig as ScryptoModuleConfig;
use radix_engine::types::{
    FungibleResourceRoles as ScryptoFungibleResourceRoles,
    GlobalAddress as ScryptoGlobalAddress, MetadataInit as ScryptoMetadataInit,
    RoleAssignmentInit as ScryptoRoleAssignmentInit,
};
use radix_engine::{
    prelude::ToMetadataEntry as ScryptoToMetadataEntry,
    types::OwnerRole as ScryptoOwnerRole,
};
use radix_engine_common::math::Decimal as ScryptoDecimal;
use radix_engine_toolkit::models::node_id::TypedNodeId as RetTypedNodeId;
use std::collections::BTreeMap;
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
    pub fn manifest_for_faucet(
        include_lock_fee_instruction: bool,
        address_of_receiving_account: &AccountAddress,
    ) -> Self {
        let mut builder = ScryptoManifestBuilder::new();

        if include_lock_fee_instruction {
            builder = builder.lock_fee_from_faucet()
        }

        let scrypto_manifest = builder
            .get_free_xrd_from_faucet()
            .try_deposit_entire_worktop_or_abort(
                address_of_receiving_account.scrypto(),
                None,
            )
            .build();

        TransactionManifest::from_scrypto(
            scrypto_manifest,
            address_of_receiving_account.network_id(),
        )
    }

    pub fn manifest_marking_account_as_dapp_definition_type(
        account_address: &AccountAddress,
    ) -> Self {
        Self::set_metadata(
            account_address,
            MetadataKey::AccountType,
            MetadataValueStr::DappDefinition,
        )
    }

    pub fn manifest_set_owner_keys_hashes(
        address_of_account_or_persona: &AddressOfAccountOrPersona,
        owner_key_hashes: Vec<PublicKeyHash>,
    ) -> Self {
        Self::set_metadata(
            address_of_account_or_persona,
            MetadataKey::OwnerKeys,
            ScryptoMetadataValue::PublicKeyHashArray(
                owner_key_hashes.into_iter().map(|h| h.into()).collect_vec(),
            ),
        )
    }

    pub fn manifest_for_create_fungible_token(
        address_of_owner: &AccountAddress,
    ) -> Self {
        Self::manifest_for_create_fungible_token_with_metadata(
            address_of_owner,
            FungibleResourceDefinitionMetadata::placeholder(),
        )
    }

    pub fn manifest_for_create_fungible_token_with_metadata(
        address_of_owner: &AccountAddress,
        metadata: FungibleResourceDefinitionMetadata,
    ) -> Self {
        let initial_supply: ScryptoDecimal = metadata.initial_supply.into();
        let scrypto_manifest = ScryptoManifestBuilder::new()
            .create_fungible_resource(
                ScryptoOwnerRole::None,
                true,
                10,
                ScryptoFungibleResourceRoles::single_locked_rule(
                    radix_engine::types::AccessRule::DenyAll,
                ),
                metadata.into(),
                Some(initial_supply),
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
}

impl TransactionManifest {
    fn set_metadata<A>(
        address: &A,
        key: MetadataKey,
        value: impl ScryptoToMetadataEntry,
    ) -> Self
    where
        A: IntoScryptoAddress,
    {
        let scrypto_manifest = ScryptoManifestBuilder::new()
            .set_metadata(address.scrypto(), key, value)
            .build();

        TransactionManifest::from_scrypto(
            scrypto_manifest,
            address.network_id(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
pub enum MetadataKey {
    #[display("account_type")]
    AccountType,

    #[display("owner_keys")]
    OwnerKeys,
}

impl From<MetadataKey> for String {
    fn from(value: MetadataKey) -> Self {
        value.to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MetadataValue {
    Str(MetadataValueStr),
}
impl MetadataValue {
    pub const DAPP_DEFINITION: Self =
        Self::Str(MetadataValueStr::DappDefinition);
}

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
pub enum MetadataValueStr {
    #[display("dapp definition")]
    DappDefinition,
}
impl ScryptoToMetadataEntry for MetadataValueStr {
    fn to_metadata_entry(self) -> Option<ScryptoMetadataValue> {
        Some(ScryptoMetadataValue::String(self.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use pretty_assertions::{assert_eq, assert_ne};
    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn manifest_for_faucet() {
        assert_eq!(
            SUT::manifest_for_faucet(
                true,
                &AccountAddress::placeholder_mainnet()
            )
            .to_string(),
            r#"CALL_METHOD
    Address("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet")
    "lock_fee"
    Decimal("5000")
;
CALL_METHOD
    Address("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet")
    "free"
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
    fn manifest_for_set_account_to_dapp_definition_address() {
        assert_eq!(
            SUT::manifest_marking_account_as_dapp_definition_type(
                &AccountAddress::placeholder_mainnet()
            )
            .to_string(),
            r#"SET_METADATA
    Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
    "account_type"
    Enum<0u8>(
        "dapp definition"
    )
;
"#
        );
    }

    #[test]
    fn manifest_for_owner_keys() {
        assert_eq!(
            SUT::manifest_set_owner_keys_hashes(
                &AccountAddress::placeholder_mainnet().into(),
                vec![
                    PublicKeyHash::hash(Ed25519PublicKey::placeholder_alice()),
                    PublicKeyHash::hash(Secp256k1PublicKey::placeholder_bob()),
                ]
            )
            .to_string(),
            r#"SET_METADATA
    Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
    "owner_keys"
    Enum<143u8>(
        Array<Enum>(
            Enum<1u8>(
                Bytes("f4e18c034e069baee91ada4764fdfcf2438b8f976861df00557d4cc9e7")
            ),
            Enum<0u8>(
                Bytes("169b4cc19da76c93d4ec3d13ad12cdd5762a8318a643d50f09d0121d94")
            )
        )
    )
;
"#
        );
    }

    #[test]
    fn manifest_for_create_fungible_token_stella() {
        assert_eq!(
            SUT::manifest_for_create_fungible_token(
                &AccountAddress::placeholder_mainnet().into(),
            )
            .to_string(),
            r#"CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY
    Enum<0u8>()
    true
    10u8
    Decimal("24000000000")
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
    fn manifest_for_create_fungible_token_with_metadata_zelda() {
        assert_eq!(
            SUT::manifest_for_create_fungible_token_with_metadata(
                &AccountAddress::placeholder_mainnet_other().into(),
                FungibleResourceDefinitionMetadata::placeholder_other()
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
                        "A brave soul."
                    )
                ),
                false
            ),
            "icon_url" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "https://uxwing.com/wp-content/themes/uxwing/download/crime-security-military-law/shield-black-icon.png"
                    )
                ),
                false
            ),
            "name" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "Zelda"
                    )
                ),
                false
            ),
            "symbol" => Tuple(
                Enum<1u8>(
                    Enum<0u8>(
                        "HERO"
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
    Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
    "try_deposit_batch_or_abort"
    Expression("ENTIRE_WORKTOP")
    Enum<0u8>()
;
"#
        );
    }
}
