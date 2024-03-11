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

#[uniffi::export]
pub fn manifest_create_multiple_fungible_tokens(
    address_of_owner: &AccountAddress,
) -> TransactionManifest {
    TransactionManifest::create_multiple_fungible_tokens(address_of_owner)
}

#[uniffi::export]
pub fn manifest_create_non_fungible_token(
    address_of_owner: &AccountAddress,
) -> TransactionManifest {
    TransactionManifest::create_single_nft_collection(address_of_owner, 20)
}

#[uniffi::export]
pub fn manifest_create_multiple_non_fungible_tokens(
    address_of_owner: &AccountAddress,
) -> TransactionManifest {
    TransactionManifest::create_multiple_nft_collections(
        address_of_owner,
        15,
        10,
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
///
/// # Panics
/// Panics if any of the TransactionGuarantee's `instruction_index` is out of
/// bounds.
///
/// Also panics if the number of TransactionGuarantee's is larger than the number
/// of instructions of `manifest` (does not make any sense).
#[uniffi::export]
pub fn modify_manifest_add_guarantees(
    manifest: TransactionManifest,
    guarantees: Vec<TransactionGuarantee>,
) -> TransactionManifest {
    manifest.modify_add_guarantees(guarantees)
}

#[uniffi::export]
pub fn build_information() -> SargonBuildInformation {
    SargonBuildInformation::get()
}

#[uniffi::export]
pub fn hash(data: BagOfBytes) -> Exactly32Bytes {
    let h = hash_of::<Vec<u8>>(data.to_vec());
    h.into()
}

#[uniffi::export]
pub fn xrd_address_of_network(network_id: NetworkID) -> ResourceAddress {
    ResourceAddress::xrd_on_network(network_id)
}

#[uniffi::export]
pub fn debug_print_compiled_notarized_intent(
    compiled: CompiledNotarizedIntent,
) -> String {
    let notarized = compiled
        .decompile()
        .expect("Should never failed to decompile");
    format!("{:?}", notarized)
}

#[uniffi::export]
pub fn manifest_assets_transfers(
    transfers: AssetsTransfersTransactionPrototype,
    message: Message,
) -> Result<TransactionManifest> {
    unreachable!()
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
            let manifest = manifest_create_multiple_fungible_tokens(&a);
            assert!(manifest.to_string().contains(&a.to_string()))
        };
        test(AccountAddress::sample_stokenet());
        test(AccountAddress::sample_stokenet_other());
    }

    #[test]
    fn test_manifest_create_multiple_fungible_tokens_number_of_tokens() {
        let manifest = manifest_create_multiple_fungible_tokens(
            &AccountAddress::sample_stokenet(),
        );
        assert_eq!(manifest.to_string().matches("symbol").count(), 25);
    }

    #[test]
    fn test_manifest_create_non_fungible_token() {
        assert_eq!(
            manifest_create_non_fungible_token(&AccountAddress::sample())
                .instructions()
                .len(),
            2
        );
    }

    #[test]
    fn test_manifest_create_multiple_non_fungible_tokens() {
        let manifest = manifest_create_multiple_non_fungible_tokens(
            &AccountAddress::sample_mainnet(),
        );
        assert_eq!(manifest.instructions().len(), 16);
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
                    ResourceAddress::sample(),
                    None,
                )],
            );
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
        let ret_v = "0.0.1";
        let re_rev =
            "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
        std::env::set_var(
            RADIX_ENGINE_TOOLKIT_DEPENDENCY,
            format!("version = {}", ret_v),
        );
        std::env::set_var(RADIX_ENGINE_DEPENDENCY, format!("rev = {}", re_rev));
        let info = build_information();
        std::env::remove_var(RADIX_ENGINE_TOOLKIT_DEPENDENCY);
        std::env::remove_var(RADIX_ENGINE_DEPENDENCY);
        assert_eq!(info.sargon_version.matches('.').count(), 2);
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
    fn xrd_address_of_network_mainnet() {
        assert_eq!(xrd_address_of_network(NetworkID::Mainnet).to_string(), "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
    }

    #[test]
    fn xrd_address_of_network_stokenet() {
        assert_eq!(xrd_address_of_network(NetworkID::Stokenet).to_string(), "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc");
    }

    #[test]
    fn test_debug_print_compiled_notarized_intent() {
        assert_eq!(
            debug_print_compiled_notarized_intent(CompiledNotarizedIntent::sample()),
            "NotarizedTransaction { signed_intent: SignedIntent { intent: header:\nTransactionHeader { network_id: Mainnet, start_epoch_inclusive: Epoch(76935), end_epoch_exclusive: Epoch(76945), nonce: Nonce(2371337), notary_public_key: Ed25519 { value: ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf }, notary_is_signatory: true, tip_percentage: 0 }\n\nmessage:\nPlainText { plaintext: PlaintextMessage { mime_type: \"text/plain\", message: StringMessage { string: \"Hello Radix!\" } } }\n\nmanifest:\nCALL_METHOD\n    Address(\"account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease\")\n    \"lock_fee\"\n    Decimal(\"0.61\")\n;\nCALL_METHOD\n    Address(\"account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease\")\n    \"withdraw\"\n    Address(\"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd\")\n    Decimal(\"1337\")\n;\nTAKE_FROM_WORKTOP\n    Address(\"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd\")\n    Decimal(\"1337\")\n    Bucket(\"bucket1\")\n;\nCALL_METHOD\n    Address(\"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master\")\n    \"try_deposit_or_abort\"\n    Bucket(\"bucket1\")\n    Enum<0u8>()\n;\n\n\n, intent_signatures: IntentSignatures { signatures: [] } }, notary_signature: NotarySignature { secret_magic: Ed25519 { value: 839ac9c47db45950fc0cd453c5ebbbfa7ae5f7c20753abe2370b5b40fdee89e522c4d810d060e0c56211d036043fd32b9908e97bf114c1835ca02d74018fdd09 } } }"
        );
    }
}
