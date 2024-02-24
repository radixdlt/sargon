use crate::prelude::*;
use radix_engine::types::GlobalAddress as ScryptoGlobalAddress;
use radix_engine_toolkit::models::node_id::TypedNodeId as RetTypedNodeId;
use transaction::{
    builder::ResolvableComponentAddress as ScryptoResolvableComponentAddress,
    model::DynamicGlobalAddress as ScryptoDynamicGlobalAddress,
    prelude::{
        ManifestBuilder as ScryptoManifestBuilder,
        TransactionManifestV1 as ScryptoTransactionManifest,
    },
};

impl TransactionManifest {
    pub fn manifest_for_faucet(
        include_lock_fee_instruction: bool,
        address_of_receiving_account: AccountAddress,
    ) -> Result<Self> {
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

        Ok(TransactionManifest::from_scrypto(
            scrypto_manifest,
            address_of_receiving_account.network_id(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn manifest_for_faucet() {
        assert_eq!(
            TransactionManifest::manifest_for_faucet(
                true,
                AccountAddress::placeholder_mainnet()
            )
            .unwrap()
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
}
