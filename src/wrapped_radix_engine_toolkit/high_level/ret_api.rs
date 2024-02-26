use crate::prelude::*;

#[uniffi::export]
pub fn manifest_for_faucet(
    include_lock_fee_instruction: bool,
    address_of_receiving_account: &AccountAddress,
) -> TransactionManifest {
    TransactionManifest::faucet(
        include_lock_fee_instruction,
        address_of_receiving_account,
    )
}

#[uniffi::export]
pub fn manifest_marking_account_as_dapp_definition_type(
    account_address: &AccountAddress,
) -> TransactionManifest {
    TransactionManifest::marking_account_as_dapp_definition_type(
        account_address,
    )
}

#[uniffi::export]
pub fn manifest_set_owner_keys_hashes(
    address_of_account_or_persona: &AddressOfAccountOrPersona,
    owner_key_hashes: Vec<PublicKeyHash>,
) -> TransactionManifest {
    TransactionManifest::set_owner_keys_hashes(
        address_of_account_or_persona,
        owner_key_hashes,
    )
}

#[uniffi::export]
pub fn manifest_for_create_fungible_token_with_metadata(
    address_of_owner: &AccountAddress,
    initial_supply: Decimal192,
    metadata: TokenDefinitionMetadata,
) -> TransactionManifest {
    TransactionManifest::create_fungible_token_with_metadata(
        address_of_owner,
        initial_supply,
        metadata,
    )
}

#[uniffi::export]
pub fn manifest_for_create_fungible_token(
    address_of_owner: &AccountAddress,
) -> TransactionManifest {
    TransactionManifest::create_fungible_token(address_of_owner)
}

#[uniffi::export]
pub fn manifest_for_create_multiple_fungible_tokens(
    address_of_owner: &AccountAddress,
) -> TransactionManifest {
    TransactionManifest::create_multiple_fungible_tokens(address_of_owner)
}

#[uniffi::export]
pub fn manifest_for_create_non_fungible_token(
    address_of_owner: &AccountAddress,
) -> TransactionManifest {
    TransactionManifest::create_non_fungible_token(address_of_owner)
}

#[uniffi::export]
pub fn manifest_for_create_multiple_non_fungible_tokens(
    _address_of_owner: &AccountAddress,
) -> Result<TransactionManifest> {
    todo!()
}

#[uniffi::export]
pub fn manifest_stakes_claim(
    _account_address: AccountAddress,
    _stake_claims: Vec<StakeClaim>,
) -> Result<TransactionManifest> {
    todo!()
}

#[uniffi::export]
pub fn manifest_third_party_deposit_update(
    _to: ThirdPartyDeposits,
) -> Result<TransactionManifest> {
    todo!()
}

/// REQUIRES NETWORK CALL (and probable cache)
/// Requires kotlinx to be setup
// #[uniffi::export]
// pub async fn manifest_assets_transfers(
//     _transfers: AssetsTransfersTransactionPrototype,
//     _message: Option<Message>,
// ) -> Result<Manifest> {
//     todo!()
// }

#[uniffi::export]
pub fn updating_manifest_lock_fee(
    _manifest: TransactionManifest,
    _address_of_fee_payer: AccountAddress,
    _fee: Option<Decimal192>,
) -> Result<TransactionManifest> {
    todo!()
}

#[uniffi::export]
pub fn updating_manifest_add_guarantees(
    _manifest: TransactionManifest,
    _guarantees: Vec<TransactionGuarantee>,
) -> Result<TransactionManifest> {
    todo!()
}

/// REQUIRES NETWORK CALL (and probable cache)
/// Requires kotlinx to be setup
// #[uniffi::export]
// pub async fn needs_signature_for_depositing(
//     _into_account: Account,
//     _resource: ResourceAddress,
// ) -> Result<bool> {
//     todo!()
// }

#[uniffi::export]
pub fn build_information() -> SargonBuildInformation {
    SargonBuildInformation::get()
}

#[uniffi::export]
pub fn hash(data: BagOfBytes) -> Hex32Bytes {
    let h: radix_engine_common::crypto::Hash =
        hash_of::<Vec<u8>>(data.to_vec());
    h.into()
}

#[uniffi::export]
pub fn xrd_address_of_network(_network_id: NetworkID) -> ResourceAddress {
    todo!()
}

#[uniffi::export]
pub fn debug_print_compiled_notarized_intent(
    _data: CompiledNotarizedIntent,
) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_manifest_for_faucet() {
        manifest_eq(
            manifest_for_faucet(false, &AccountAddress::placeholder_mainnet()),
            r#"
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
            "#,
        );

        manifest_eq(
            manifest_for_faucet(
                true,
                &AccountAddress::placeholder_stokenet_other(),
            ),
            r#"
            CALL_METHOD
                Address("component_tdx_2_1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxyulkzl")
                "lock_fee"
                Decimal("5000")
            ;
            CALL_METHOD
                Address("component_tdx_2_1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxyulkzl")
                "free"
            ;
            CALL_METHOD
                Address("account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr")
                "try_deposit_batch_or_abort"
                Expression("ENTIRE_WORKTOP")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_manifest_marking_account_as_dapp_definition_type() {
        manifest_eq(
            manifest_marking_account_as_dapp_definition_type(
                &AccountAddress::placeholder_stokenet_other(),
            ),
            r#"
            SET_METADATA
                Address("account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr")
                "account_type"
                Enum<0u8>(
                    "dapp definition"
                )
            ;
            "#,
        );
    }
}
