use crate::prelude::*;
use radix_engine::prelude::ToMetadataEntry as ScryptoToMetadataEntry;
use radix_engine::types::GlobalAddress as ScryptoGlobalAddress;
use radix_engine_toolkit::models::node_id::TypedNodeId as RetTypedNodeId;
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

    pub fn set_metadata<A>(
        address: &A,
        key: MetadataKey,
        value: MetadataValueStr,
    ) -> Self
    where
        A: AddressViaRet,
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
}

impl From<MetadataKey> for String {
    fn from(value: MetadataKey) -> Self {
        value.to_string()
    }
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
}
