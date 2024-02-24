use crate::prelude::*;
// use transaction::{
//     ManifestBuilder as RetManifestBuilder,
//     TransactionManifest as RetTransactionManifest,
// };

use transaction::prelude::{
    ManifestBuilder as ScryptoManifestBuilder,
    TransactionManifestV1 as ScryptoTransactionManifest,
};

#[uniffi::export]
pub fn manifest_third_party_deposit_update(
    _to: ThirdPartyDeposits,
) -> Result<Manifest> {
    todo!()
}

#[uniffi::export]
pub fn manifest_for_faucet(
    _include_lock_fee_instruction: bool,
    _network_id: NetworkID,
    _address_of_receiving_account: AccountAddress,
) -> Result<Manifest> {
    // RetManifestBuilder::new()
    //     .faucet_free_xrd()
    //     .and_then(|b| {
    //         if include_lock_fee_instruction {
    //             b.faucet_lock_fee()
    //         } else {
    //             Ok(b)
    //         }
    //     })
    //     .map(|b| b.build(network_id.discriminant()))
    //     .map(|r: Arc<RetTransactionManifest>| r.into())
    //     .map(|m: Manifest| m)
    //     .map_err(|_e| CommonError::Unknown)
    todo!()
}

#[uniffi::export]
pub fn manifest_set_owner_keys(
    _address_of_account_or_persona: AddressOfAccountOrPersona,
    _owner_key_hashes: Vec<PublicKeyHash>,
    _network_id: NetworkID,
) -> Manifest {
    todo!()
}

#[uniffi::export]
pub fn manifest_for_create_fungible_token(
    _address_of_owner: AccountAddress,
    _network_id: NetworkID,
) -> Result<Manifest> {
    todo!()
}

#[uniffi::export]
pub fn manifest_for_create_multiple_fungible_tokens(
    _address_of_owner: AccountAddress,
    _network_id: NetworkID,
) -> Result<Manifest> {
    todo!()
}

#[uniffi::export]
pub fn manifest_for_create_multiple_non_fungible_tokens(
    _address_of_owner: AccountAddress,
    _network_id: NetworkID,
) -> Result<Manifest> {
    todo!()
}

#[uniffi::export]
pub fn manifest_for_create_non_fungible_token(
    _address_of_owner: AccountAddress,
    _network_id: NetworkID,
) -> Result<Manifest> {
    todo!()
}

#[uniffi::export]
pub fn manifest_marking_account_as_dapp_definition_type(
    _account_address: AccountAddress,
) -> Result<Manifest> {
    todo!()
}

#[uniffi::export]
pub fn manifest_stakes_claim(
    _account_address: AccountAddress,
    _stake_claims: Vec<StakeClaim>,
) -> Result<Manifest> {
    todo!()
}

/// REQUIRES NETWORK CALL (and probable cache)
#[uniffi::export]
pub async fn manifest_assets_transfers(
    _transfers: AssetsTransfersTransactionPrototype,
    _message: Option<Message>,
) -> Result<Manifest> {
    todo!()
}

#[uniffi::export]
pub fn updating_manifest_lock_fee(
    _manifest: Manifest,
    _address_of_fee_payer: AccountAddress,
    _fee: Option<Decimal192>,
) -> Result<Manifest> {
    todo!()
}

#[uniffi::export]
pub fn updating_manifest_add_guarantees(
    _manifest: Manifest,
    _guarantees: Vec<TransactionGuarantee>,
) -> Result<Manifest> {
    todo!()
}

/// REQUIRES NETWORK CALL (and probable cache)
#[uniffi::export]
pub async fn needs_signature_for_depositing(
    _into_account: Account,
    _resource: ResourceAddress,
) -> Result<bool> {
    todo!()
}

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
