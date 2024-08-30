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
pub fn manifest_create_fungible_token_with_metadata(
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
pub fn manifest_create_fungible_token(
    address_of_owner: &AccountAddress,
) -> TransactionManifest {
    TransactionManifest::create_fungible_token(address_of_owner)
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
    TransactionManifest::create_multiple_fungible_tokens(
        address_of_owner,
        count,
    )
}

#[uniffi::export]
pub fn manifest_create_non_fungible_token(
    address_of_owner: &AccountAddress,
    nfts_per_collection: Option<u8>,
) -> TransactionManifest {
    TransactionManifest::create_single_nft_collection(
        address_of_owner,
        nfts_per_collection.map(|n| n as u64).unwrap_or(20),
    )
}

#[uniffi::export]
pub fn manifest_create_multiple_non_fungible_tokens(
    address_of_owner: &AccountAddress,
    collection_count: Option<u8>,
    nfts_per_collection: Option<u8>,
) -> TransactionManifest {
    TransactionManifest::create_multiple_nft_collections(
        address_of_owner,
        collection_count.map(|n| n as u16).unwrap_or(15),
        nfts_per_collection.map(|n| n as u64).unwrap_or(10),
    )
}

#[uniffi::export]
pub fn manifest_stakes_claim(
    account_address: &AccountAddress,
    stake_claims: Vec<StakeClaim>,
) -> TransactionManifest {
    TransactionManifest::stake_claims(account_address, stake_claims)
}

#[uniffi::export]
pub fn manifest_third_party_deposit_update(
    account_address: &AccountAddress,
    from: ThirdPartyDeposits,
    to: ThirdPartyDeposits,
) -> TransactionManifest {
    TransactionManifest::third_party_deposit_update(account_address, from, to)
}

#[uniffi::export]
pub fn modify_manifest_lock_fee(
    manifest: TransactionManifest,
    address_of_fee_payer: &AccountAddress,
    fee: Option<Decimal192>,
) -> TransactionManifest {
    manifest.modify_add_lock_fee(address_of_fee_payer, fee)
}

/// Modifies `manifest` by inserting transaction "guarantees", which is the wallet
/// term for `assert_worktop_contains`.
#[uniffi::export]
pub fn modify_manifest_add_guarantees(
    manifest: TransactionManifest,
    guarantees: Vec<TransactionGuarantee>,
) -> Result<TransactionManifest> {
    manifest.modify_add_guarantees(guarantees)
}

#[uniffi::export]
pub fn build_information() -> SargonBuildInformation {
    SargonBuildInformation::get()
}

#[uniffi::export]
pub fn hash(data: BagOfBytes) -> Hash {
    hash_of::<Vec<u8>>(data.to_vec())
}

#[uniffi::export]
pub fn xrd_address_of_network(network_id: NetworkID) -> ResourceAddress {
    ResourceAddress::xrd_on_network(network_id)
}

#[uniffi::export]
pub fn debug_print_compiled_notarized_intent(
    compiled: CompiledNotarizedIntent,
) -> String {
    let notarized = compiled.decompile();
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
    TransactionManifest::per_recipient_transfers(transfers)
}

#[uniffi::export]
pub fn manifest_per_asset_transfers(
    transfers: PerAssetTransfers,
) -> TransactionManifest {
    TransactionManifest::per_asset_transfers(transfers)
}

#[uniffi::export]
pub fn manifest_account_locker_claim(
    locker_address: &LockerAddress,
    claimant: &AccountAddress,
    claimable_resources: Vec<AccountLockerClaimableResource>,
    use_try_deposit_or_abort: bool,
) -> TransactionManifest {
    TransactionManifest::account_locker_claim(
        locker_address,
        claimant,
        claimable_resources,
        use_try_deposit_or_abort,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_for_faucet() {
        manifest_eq(
            manifest_for_faucet(false, &AccountAddress::sample_mainnet()),
            r#"
            CALL_METHOD
                Address("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet")
                "free"
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "try_deposit_batch_or_abort"
                Expression("ENTIRE_WORKTOP")
                Enum<0u8>()
            ;
            "#,
        );

        manifest_eq(
            manifest_for_faucet(true, &AccountAddress::sample_stokenet_other()),
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
                Address("account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp")
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
                &AccountAddress::sample_stokenet_other(),
            ),
            r#"
            SET_METADATA
                Address("account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp")
                "account_type"
                Enum<0u8>(
                    "dapp definition"
                )
            ;
            "#,
        );
    }

    #[test]
    fn test_manifest_set_owner_keys_hashes() {
        manifest_eq(
            manifest_set_owner_keys_hashes(
                &AccountAddress::sample_mainnet().into(),
                vec![
                    PublicKeyHash::hash(Ed25519PublicKey::sample_alice()),
                    PublicKeyHash::hash(Secp256k1PublicKey::sample_bob()),
                ],
            ),
            r#"
            SET_METADATA
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
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
            "#,
        );
    }

    #[test]
    fn test_manifest_create_fungible_token_with_metadata() {
        let string = manifest_create_fungible_token_with_metadata(
            &AccountAddress::sample_stokenet_other(),
            748392.into(),
            TokenDefinitionMetadata::new(
                "Foobar",
                "foobar",
                "FOO",
                "example.com",
                ["Tag0".to_string(), "Tag1".to_string()],
            ),
        )
        .to_string();

        assert!(string.contains("748392"));
        assert!(string.contains("Foobar"));
        assert!(string.contains("foobar"));
        assert!(string.contains("FOO"));
        assert!(string.contains("example.com"));
        assert!(string.contains("Tag0"));
        assert!(string.contains("Tag1"));
        assert!(string
            .contains(&AccountAddress::sample_stokenet_other().to_string()));
    }

    #[test]
    fn test_manifest_create_fungible_token_owner() {
        let test = |a| {
            let manifest = manifest_create_fungible_token(&a);
            assert!(manifest.to_string().contains(&a.to_string()))
        };
        test(AccountAddress::sample_mainnet());
        test(AccountAddress::sample_mainnet_other());
        test(AccountAddress::sample_stokenet());
        test(AccountAddress::sample_stokenet_other());
    }

    #[test]
    fn test_manifest_create_multiple_fungible_tokens_owner() {
        let test = |a| {
            let manifest = manifest_create_multiple_fungible_tokens(&a, None);
            assert!(manifest.to_string().contains(&a.to_string()))
        };
        test(AccountAddress::sample_stokenet());
        test(AccountAddress::sample_stokenet_other());
    }

    #[test]
    fn test_manifest_create_multiple_fungible_tokens_number_of_tokens() {
        let test = |n: u8| {
            let manifest = manifest_create_multiple_fungible_tokens(
                &AccountAddress::sample_stokenet(),
                Some(n),
            );
            assert_eq!(
                manifest.to_string().matches("symbol").count(),
                n as usize
            );
        };
        test(1);
        test(2);
        test(24);
        test(25);
    }

    #[test]
    fn test_manifest_create_non_fungible_token() {
        assert_eq!(
            manifest_create_non_fungible_token(&AccountAddress::sample(), None)
                .instructions()
                .len(),
            2
        );
    }

    #[test]
    fn test_manifest_create_multiple_non_fungible_tokens() {
        let test = |n: u8| {
            let manifest = manifest_create_multiple_non_fungible_tokens(
                &AccountAddress::sample_mainnet(),
                Some(n),
                None,
            );
            assert_eq!(manifest.instructions().len(), (n as usize) + 1);
        };
        test(1);
        test(2);
        test(15);
    }

    #[test]
    fn test_manifest_stakes_claim() {
        let manifest = manifest_stakes_claim(
            &AccountAddress::sample_mainnet(),
            vec![StakeClaim::sample(), StakeClaim::sample_other()],
        );
        assert_eq!(manifest.instructions().len(), 10);
    }

    #[test]
    fn test_manifest_third_party_deposit_update() {
        let manifest = manifest_third_party_deposit_update(
            &AccountAddress::sample_mainnet(),
            ThirdPartyDeposits::sample(),
            ThirdPartyDeposits::sample_other(),
        );
        assert_eq!(manifest.instructions().len(), 3);
    }

    #[test]
    fn test_modify_manifest_lock_fee() {
        let mut manifest =
            TransactionManifest::sample_mainnet_without_lock_fee();
        let instruction_count_before_lock_fee = manifest.instructions().len();
        manifest = modify_manifest_lock_fee(
            manifest,
            &AccountAddress::sample(),
            Some(Decimal192::one()),
        );
        assert_eq!(
            manifest.instructions().len(),
            instruction_count_before_lock_fee + 1
        );
    }

    #[test]
    fn test_modify_manifest_add_guarantees_to_manifest_with_or_without_lock_fee(
    ) {
        let do_test = |manifest: TransactionManifest, i: usize| {
            let modified = modify_manifest_add_guarantees(
                manifest,
                vec![TransactionGuarantee::new(
                    0,
                    0,
                    0,
                    ResourceAddress::sample(),
                    None,
                )],
            )
            .unwrap();
            let idx = modified
                .instructions()
                .clone()
                .into_iter()
                .position(|i| i.is_assert_worktop_contains())
                .unwrap();
            assert_eq!(idx, i)
        };
        do_test(TransactionManifest::sample(), 1);
        do_test(TransactionManifest::sample_mainnet_without_lock_fee(), 0);
    }

    #[test]
    fn test_build_information() {
        let info = build_information();

        assert_eq!(info.sargon_version.matches('.').count(), 2);
        assert!(
            !format!("{:?}", info.dependencies.radix_engine_toolkit).is_empty()
        );
        assert!(
            !format!("{:?}", info.dependencies.scrypto_radix_engine).is_empty()
        );
    }

    #[test]
    fn test_hash() {
        assert_eq!(
            hash("Hello Radix".as_bytes().into()).to_string(),
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
        );
    }

    #[test]
    fn xrd_address_of_network_mainnet() {
        assert_eq!(xrd_address_of_network(NetworkID::Mainnet).to_string(), "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
    }

    #[test]
    fn xrd_address_of_network_stokenet() {
        assert_eq!(xrd_address_of_network(NetworkID::Stokenet).to_string(), "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc");
    }

    #[test]
    fn test_debug_print_compiled_notarized_intent() {
        assert!(
            debug_print_compiled_notarized_intent(
                CompiledNotarizedIntent::sample()
            )
            .len()
                > 1000
        );
    }

    #[test]
    fn per_recipient_uses_per_asset_transfer() {
        let transfers = PerRecipientAssetTransfers::sample();
        assert_eq!(
            manifest_per_asset_transfers(transfers.clone().transpose()),
            manifest_per_recipient_transfers(transfers)
        );
    }

    #[test]
    fn test_account_locker_claim() {
        let manifest = manifest_account_locker_claim(
            &LockerAddress::sample(),
            &AccountAddress::sample(),
            vec![AccountLockerClaimableResource::sample()],
            true,
        );
        assert_eq!(manifest.instructions().len(), 2);
    }
}
