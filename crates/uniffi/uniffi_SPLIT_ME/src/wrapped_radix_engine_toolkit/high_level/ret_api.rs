use crate::prelude::*;
use sargon::ResourceAddress as InternalResourceAddress;
use sargon::SargonBuildInformation as InternalSargonBuildInformation;
use sargon::TransactionManifest as InternalTransactionManifest;

use sargon::{
    // manifests crate
    ManifestForAccountLockerClaim as _,
    TransactionManifestAssetTransfers as _,
    TransactionManifestFaucet as _,
    TransactionManifestMetadataSetting as _,
    TransactionManifestStakeClaim as _,
    TransactionManifestThirdPartyDepositUpdating as _,
    TransactionManifestTokenCreating as _,
};

#[uniffi::export]
pub fn manifest_for_faucet(
    include_lock_fee_instruction: bool,
    address_of_receiving_account: &AccountAddress,
) -> TransactionManifest {
    InternalTransactionManifest::faucet(
        include_lock_fee_instruction,
        &address_of_receiving_account.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn manifest_marking_account_as_dapp_definition_type(
    account_address: &AccountAddress,
) -> TransactionManifest {
    InternalTransactionManifest::marking_account_as_dapp_definition_type(
        &account_address.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn manifest_set_owner_keys_hashes(
    address_of_account_or_persona: &AddressOfAccountOrPersona,
    owner_key_hashes: Vec<PublicKeyHash>,
) -> TransactionManifest {
    InternalTransactionManifest::set_owner_keys_hashes(
        &address_of_account_or_persona.into_internal(),
        owner_key_hashes.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn manifest_create_fungible_token_with_metadata(
    address_of_owner: &AccountAddress,
    initial_supply: Decimal192,
    metadata: TokenDefinitionMetadata,
) -> TransactionManifest {
    InternalTransactionManifest::create_fungible_token_with_metadata(
        &address_of_owner.into_internal(),
        initial_supply.into_internal(),
        metadata.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn manifest_create_fungible_token(
    address_of_owner: &AccountAddress,
) -> TransactionManifest {
    InternalTransactionManifest::create_fungible_token(
        &address_of_owner.into_internal(),
    )
    .into()
}

/// Creates many fungible tokens, with initial supply, to be owned by `address_of_owner`.
///
/// # Panics
/// Panics if `address_of_owner` is on `Mainnet`, use a testnet instead.
/// Panics if `count` is zero or is greater than the number of token metadata defined in `sample_resource_definition_metadata` (25)
#[uniffi::export]
pub fn manifest_create_multiple_fungible_tokens(
    address_of_owner: &AccountAddress,
    count: Option<u8>,
) -> TransactionManifest {
    InternalTransactionManifest::create_multiple_fungible_tokens(
        &address_of_owner.into_internal(),
        count,
    )
    .into()
}

#[uniffi::export]
pub fn manifest_create_non_fungible_token(
    address_of_owner: &AccountAddress,
    nfts_per_collection: Option<u8>,
) -> TransactionManifest {
    InternalTransactionManifest::create_single_nft_collection(
        &address_of_owner.into_internal(),
        nfts_per_collection.map(|n| n as u64).unwrap_or(20),
    )
    .into()
}

#[uniffi::export]
pub fn manifest_create_multiple_non_fungible_tokens(
    address_of_owner: &AccountAddress,
    collection_count: Option<u8>,
    nfts_per_collection: Option<u8>,
) -> TransactionManifest {
    InternalTransactionManifest::create_multiple_nft_collections(
        &address_of_owner.into_internal(),
        collection_count.map(|n| n as u16).unwrap_or(15),
        nfts_per_collection.map(|n| n as u64).unwrap_or(10),
    )
    .into()
}

#[uniffi::export]
pub fn manifest_stakes_claim(
    account_address: &AccountAddress,
    stake_claims: Vec<StakeClaim>,
) -> TransactionManifest {
    InternalTransactionManifest::stake_claims(
        &account_address.into_internal(),
        stake_claims.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn manifest_third_party_deposit_update(
    account_address: &AccountAddress,
    from: ThirdPartyDeposits,
    to: ThirdPartyDeposits,
) -> TransactionManifest {
    InternalTransactionManifest::third_party_deposit_update(
        &account_address.into_internal(),
        from.into_internal(),
        to.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn build_information() -> SargonBuildInformation {
    InternalSargonBuildInformation::get().into()
}

#[uniffi::export]
pub fn hash(data: BagOfBytes) -> Hash {
    sargon::hash_of::<Vec<u8>>(data.into_internal().to_vec()).into()
}

#[uniffi::export]
pub fn xrd_address_of_network(network_id: NetworkID) -> ResourceAddress {
    InternalResourceAddress::xrd_on_network(network_id.into()).into()
}

#[uniffi::export]
pub fn debug_print_compiled_notarized_intent(
    compiled: CompiledNotarizedIntent,
) -> String {
    let notarized = compiled.into_internal().decompile();
    format!("{:?}", notarized)
}

/// Uses `per_asset_transfers` after having transposed the `PerRecipientAssetTransfers`
/// into `PerAssetTransfers`. We always use `PerAssetTransfers` when building the manifest
/// since it is more efficient (allows a single withdraw per resource) => fewer instruction =>
/// cheaper TX fee for user.
#[uniffi::export]
pub fn manifest_per_recipient_transfers(
    transfers: PerRecipientAssetTransfers,
) -> TransactionManifest {
    InternalTransactionManifest::per_recipient_transfers(transfers.into())
        .into()
}

#[uniffi::export]
pub fn manifest_per_asset_transfers(
    transfers: PerAssetTransfers,
) -> TransactionManifest {
    InternalTransactionManifest::per_asset_transfers(transfers.into()).into()
}

#[uniffi::export]
pub fn manifest_account_locker_claim(
    locker_address: &LockerAddress,
    claimant: &AccountAddress,
    claimable_resources: Vec<AccountLockerClaimableResource>,
) -> TransactionManifest {
    InternalTransactionManifest::account_locker_claim(
        &locker_address.into_internal(),
        &claimant.into_internal(),
        claimable_resources.into_internal(),
    )
    .into()
}
