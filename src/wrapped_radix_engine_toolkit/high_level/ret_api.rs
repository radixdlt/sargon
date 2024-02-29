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
) -> TransactionManifest {
    todo!()
}

#[uniffi::export]
pub fn manifest_stakes_claim(
    _account_address: &AccountAddress,
    _stake_claims: Vec<StakeClaim>,
) -> TransactionManifest {
    todo!()
}

#[uniffi::export]
pub fn manifest_third_party_deposit_update(
    _to: ThirdPartyDeposits,
    _owner: &AccountAddress,
) -> TransactionManifest {
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
    _address_of_fee_payer: &AccountAddress,
    _fee: Option<Decimal192>,
) -> TransactionManifest {
    todo!()
}

#[uniffi::export]
pub fn updating_manifest_add_guarantees(
    _manifest: TransactionManifest,
    _guarantees: Vec<TransactionGuarantee>,
) -> TransactionManifest {
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
pub fn hash(data: BagOfBytes) -> Exactly32Bytes {
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
            manifest_for_faucet(false, &AccountAddress::sample_mainnet()),
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
                &AccountAddress::sample_stokenet_other(),
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
            "#,
        );
    }

    #[test]
    fn test_manifest_for_create_fungible_token_with_metadata() {
        let string = manifest_for_create_fungible_token_with_metadata(
            &AccountAddress::sample_stokenet_other(),
            748392.into(),
            TokenDefinitionMetadata::new(
                "Foobar",
                "foobar",
                "FOO",
                "example.com",
            ),
        )
        .to_string();

        assert!(string.contains("748392"));
        assert!(string.contains("Foobar"));
        assert!(string.contains("foobar"));
        assert!(string.contains("FOO"));
        assert!(string.contains("example.com"));
        assert!(string
            .contains(&AccountAddress::sample_stokenet_other().to_string()));
    }

    #[test]
    fn test_manifest_for_create_fungible_token_owner() {
        let test = |a| {
            let manifest = manifest_for_create_fungible_token(&a);
            assert!(manifest.to_string().contains(&a.to_string()))
        };
        test(AccountAddress::sample_mainnet());
        test(AccountAddress::sample_mainnet_other());
        test(AccountAddress::sample_stokenet());
        test(AccountAddress::sample_stokenet_other());
    }

    #[test]
    fn test_manifest_for_create_multiple_fungible_tokens_owner() {
        let test = |a| {
            let manifest = manifest_for_create_multiple_fungible_tokens(&a);
            assert!(manifest.to_string().contains(&a.to_string()))
        };
        test(AccountAddress::sample_stokenet());
        test(AccountAddress::sample_stokenet_other());
    }

    #[test]
    fn test_manifest_for_create_multiple_fungible_tokens_number_of_tokens() {
        let manifest = manifest_for_create_multiple_fungible_tokens(
            &AccountAddress::sample_stokenet(),
        );
        assert_eq!(manifest.to_string().matches("symbol").count(), 25);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_manifest_for_create_multiple_non_fungible_tokens() {
        manifest_eq(
            manifest_for_create_multiple_non_fungible_tokens(
                &AccountAddress::sample_mainnet(),
            ),
            r#"
            todo
            "#,
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_manifest_stakes_claim() {
        manifest_eq(
            manifest_stakes_claim(&AccountAddress::sample_mainnet(), vec![]),
            r#"
            todo
            "#,
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_manifest_third_party_deposit_update() {
        manifest_eq(
            manifest_third_party_deposit_update(
                ThirdPartyDeposits::sample(),
                &AccountAddress::sample_mainnet(),
            ),
            r#"
            todo
            "#,
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_updating_manifest_lock_fee() {
        manifest_eq(
            updating_manifest_lock_fee(
                TransactionManifest::sample(),
                &AccountAddress::sample_mainnet(),
                Some(1000.into()),
            ),
            r#"
            todo
            "#,
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_updating_manifest_add_guarantees() {
        manifest_eq(
            updating_manifest_add_guarantees(
                TransactionManifest::sample(),
                vec![],
            ),
            r#"
            todo
            "#,
        );
    }

    #[test]
    fn test_build_information() {
        let ret_v = "0.0.1";
        let re_rev =
            "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
        std::env::set_var(
            "RADIX-ENGINE-TOOLKIT-DEPENDENCY",
            format!("version = {}", ret_v),
        );
        std::env::set_var(
            "RADIX-ENGINE-DEPENDENCY",
            format!("rev = {}", re_rev),
        );
        let info = build_information();
        assert_eq!(info.sargon_version.matches(".").count(), 2);
        assert_eq!(
            info.dependencies
                .radix_engine_toolkit
                .into_version()
                .unwrap(),
            ret_v
        );
        assert_eq!(
            info.dependencies.scrypto_radix_engine.into_rev().unwrap(),
            re_rev
        );
    }

    #[test]
    fn test_hash() {
        assert_eq!(
            hash("Hello Radix".as_bytes().into()).to_hex(),
            "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935"
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_xrd_address_of_network() {
        assert_eq!(xrd_address_of_network(NetworkID::Mainnet).to_string(), "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_debug_print_compiled_notarized_intent() {
        assert_eq!(
            debug_print_compiled_notarized_intent(CompiledNotarizedIntent {}),
            "todo"
        );
    }
}
