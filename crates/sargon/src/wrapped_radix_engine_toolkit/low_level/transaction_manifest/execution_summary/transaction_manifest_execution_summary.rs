use crate::prelude::*;

use radix_engine_toolkit::functions::manifest::execution_summary as RET_execution_summary;

impl TransactionManifest {
    pub fn execution_summary(
        &self,
        engine_toolkit_receipt: BagOfBytes,
    ) -> Result<ExecutionSummary> {
        let network_definition = self.network_id().network_definition();
        let receipt_str = String::from_utf8(engine_toolkit_receipt.bytes)
            .map_err(|_| CommonError::FailedToDecodeEngineToolkitReceipt)?;
        let receipt = serde_json::from_str::<
            ScryptoSerializableToolkitTransactionReceipt,
        >(&receipt_str)
        .ok()
        .and_then(|receipt| {
            receipt
                .into_runtime_receipt(&ScryptoAddressBech32Decoder::new(
                    &network_definition,
                ))
                .ok()
        })
        .ok_or(CommonError::FailedToDecodeEngineToolkitReceipt)?;

        self.execution_summary_with_receipt(receipt)
    }

    fn execution_summary_with_receipt(
        &self,
        receipt: ScryptoRuntimeToolkitTransactionReceipt,
    ) -> Result<ExecutionSummary> {
        let ret_execution_summary = RET_execution_summary(
            &self.scrypto_manifest(),
            &receipt,
        )
        .map_err(|e| {
            error!("Failed to get execution summary from RET, error: {:?}", e);
            CommonError::ExecutionSummaryFail {
                underlying: format!("{:?}", e),
            }
        })?;

        Ok(ExecutionSummary::from((
            ret_execution_summary,
            self.network_id(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use radix_engine::transaction::{
        AbortReason, AbortResult, TransactionResult,
    };

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ExecutionSummary;

    #[test]
    fn invalid_receipt() {
        assert_eq!(
            TransactionManifest::sample()
                .execution_summary(BagOfBytes::from_hex("dead").unwrap()),
            Err(CommonError::FailedToDecodeEncodedReceipt)
        );
    }

    #[test]
    fn failure_if_receipt_result_is_abort() {
        let encoded_receipt_hex =
            include_str!(concat!(env!("FIXTURES_TX"), "create_pool.dat"));
        let wrong_receipt_raw =
            BagOfBytes::from_hex(encoded_receipt_hex).unwrap();
        let mut wrong_receipt: TransactionReceipt =
            wrong_receipt_raw.try_into().unwrap();
        wrong_receipt.decoded.result = TransactionResult::Abort(AbortResult {
            reason: AbortReason::ConfiguredAbortTriggeredOnFeeLoanRepayment,
        });
        assert_eq!(
            TransactionManifest::sample()
                .execution_summary_with_receipt(wrong_receipt),
            Err(CommonError::ExecutionSummaryFail {
                underlying: "InvalidReceipt".to_owned()
            })
        );
    }

    impl Default for FeeLocks {
        fn default() -> Self {
            Self::new(0, 0)
        }
    }

    #[test]
    fn transfer_1to2_multiple_nf_and_f_tokens() {
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "transfer_1to2_multiple_nf_and_f_tokens.dat"
        ));
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "transfer_1to2_multiple_nf_and_f_tokens.rtm"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [
                    (
                        acc_gk,
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy",
                                FungibleResourceIndicator::guaranteed("36.4567896543")
                            ),
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1th6hufew82dpntmcn7kt9f7au50cr59996tawh4syph0kz5e99v2u6",
                                FungibleResourceIndicator::guaranteed("5.24")
                            ),
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1thnhmen4wg29tnqrfpk9w2v90s64z8at9sethnjma76866rfvcc2gs",
                                FungibleResourceIndicator::guaranteed("42.23727")
                            ),
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc",
                                FungibleResourceIndicator::guaranteed("237.987654")
                            ),
                        ]
                    )
                ],
                [
                    (
                        AccountAddress::from("account_tdx_2_12x0xzsa3dm2tthpz3nwsvh94e8kq7acu0x2kfjlpv5kulsynt7rpwp"),
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy",
                                FungibleResourceIndicator::guaranteed("12.4567896543")
                            )
                        ]
                    ),
                    (
                        AccountAddress::from("account_tdx_2_12xwdc3gsu48juzkj56s0zz0vqx26xcmw9kehcudm85w57cynter9z4"),
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy",
                                FungibleResourceIndicator::guaranteed(24)
                            ),
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1th6hufew82dpntmcn7kt9f7au50cr59996tawh4syph0kz5e99v2u6",
                                FungibleResourceIndicator::guaranteed("5.24")
                            ),
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1thnhmen4wg29tnqrfpk9w2v90s64z8at9sethnjma76866rfvcc2gs",
                                FungibleResourceIndicator::guaranteed("42.23727")
                            ),
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc",
                                FungibleResourceIndicator::guaranteed("237.987654")
                            )
                        ]
                    )
                ],
                [acc_gk],
                [],
                [],
                [],
                [],
                [],
                [
                    DetailedManifestClass::Transfer {
                        is_one_to_one: false
                    },
                    DetailedManifestClass::General
                ],
                FeeLocks::default(),
                FeeSummary::new(
                    "0.4077857",
                    "0.1307814",
                    "0.52433013014",
                    0,
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn third_party_deposits_update() {
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "third_party_deposits_update.dat"
        ));
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "third_party_deposits_update.rtm"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_g2: AccountAddress = "account_tdx_2_12xx9jz27aa0mqjj8cwhk7pzkhtkthv09yclmurse42hlyme2gegyg2".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [],
                [],
                [acc_g2], // addresses_of_accounts_requiring_auth
                [],               // addresses_of_identities_requiring_auth
                [],               // newly_created_non_fungibles
                [ReservedInstruction::AccountUpdateSettings],
                [],               // presented_proofs
                [],               // encountered_component_addresses
                [
                    DetailedManifestClass::AccountDepositSettingsUpdate {
                        resource_preferences_updates: HashMap::<
                            AccountAddress,
                            HashMap<ResourceAddress, ResourcePreferenceUpdate>,
                        >::from_iter([(
                            acc_g2,
                            HashMap::<_, _>::from_iter([(
                                ResourceAddress::sample_stokenet_gc_tokens(),
                                ResourcePreferenceUpdate::Remove,
                            )]),
                        )]),
                        deposit_mode_updates:
                            HashMap::<AccountAddress, DepositRule>::from_iter([(
                                acc_g2,
                                DepositRule::DenyAll,
                            )]),
                        authorized_depositors_added: HashMap::<
                            AccountAddress,
                            Vec<ResourceOrNonFungible>,
                        >::from_iter([(
                            acc_g2,
                            vec![ResourceOrNonFungible::Resource {
                                value: ResourceAddress::sample_stokenet_nft_gc_membership(),
                            }],
                        )]),
                        authorized_depositors_removed: HashMap::<
                            AccountAddress,
                            Vec<ResourceOrNonFungible>,
                        >::from_iter([(
                            acc_g2,
                            vec![ResourceOrNonFungible::Resource {
                                value: ResourceAddress::sample_stokenet_nft_other(),
                            }],
                        )]),
                    }
                ],
                FeeLocks::default(),
                FeeSummary::new(
                    "0.092499",
                    "0.02100205",
                    "0.08459091041",
                    0
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn create_single_fungible() {
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_single_fungible.dat"
        ));
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_single_fungible.rtm"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_g2: AccountAddress = "account_tdx_2_12xx9jz27aa0mqjj8cwhk7pzkhtkthv09yclmurse42hlyme2gegyg2".into();
        let token_address: ResourceAddress = "resource_tdx_2_1t4wty7nq976ej4wtx7p4ckm073p32cyaajk4cq256rcvzz20e7qrm9".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [],
                [(
                    acc_g2,
                    vec![ResourceIndicator::fungible(
                        token_address,
                        FungibleResourceIndicator::predicted(100000, 1)
                    )]
                )],
                [], // addresses_of_accounts_requiring_auth
                [], // addresses_of_identities_requiring_auth
                [], // newly_created_non_fungibles
                [], // reserved_instructions
                [], // presented_proofs
                [], // encountered_component_addresses
                [DetailedManifestClass::General],
                FeeLocks::default(),
                FeeSummary::new("0.15800815", "0.1160115", "0.25339126151", 0,),
                NewEntities::new([(
                    token_address,
                    NewlyCreatedResource::with(
                        "MyResource",
                        "VIP",
                        "A very innovative and important resource",
                        "https://i.imgur.com/A2itmif.jpeg",
                        []
                    )
                )])
            )
        );
    }

    #[test]
    fn create_nft_collections() {
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_nft_collections.dat"
        ));
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_nft_collections.rtm"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        pretty_assertions::assert_eq!(
                sut,
                SUT::new(
                    [], // addresses_of_accounts_withdrawn_from
                    [], // addresses_of_accounts_deposited_into
                    [], // addresses_of_accounts_requiring_auth
                    [], // addresses_of_identities_requiring_auth
                    [
                        "resource_tdx_2_1nfe6ugwjvuqc7aqhcltnwmp8xfkpqnm9mc8n9jh5xcvnx38r2wncre:#0#",
                        "resource_tdx_2_1nfe6ugwjvuqc7aqhcltnwmp8xfkpqnm9mc8n9jh5xcvnx38r2wncre:#1#", 
                        "resource_tdx_2_1ntwlumm8g8hsx0emmxgj3akcx6aajspx06llvfmq733x2ssm4v3g0e:#0#",
                        "resource_tdx_2_1ntwlumm8g8hsx0emmxgj3akcx6aajspx06llvfmq733x2ssm4v3g0e:#1#"
                    ].into_iter().map(NonFungibleGlobalId::from_str).map(Result::unwrap).collect_vec(), // newly_created_non_fungibles
                    [], // reserved_instructions
                    [], // presented_proofs
                    [], // encountered_component_addresses
                    [
                        DetailedManifestClass::General
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.21852455",
                        "0.3320334",
                        "0.73328016927",
                        0,
                    ),
                    NewEntities::new([
                        (
                            ResourceAddress::from("resource_tdx_2_1ntwlumm8g8hsx0emmxgj3akcx6aajspx06llvfmq733x2ssm4v3g0e"),
                            NewlyCreatedResource::with(
                                "Abandon",
                                "ABANDON",
                                "Abandon: An amazingly innovative and rare NFT collection",
                                "https://image-service-test-images.s3.eu-west-2.amazonaws.com/wallet_test_images/KLHaze-medium.jpg",
                                ["Unique".to_string(), "FOMO".to_string(), "Advanced".to_string()]
                            )
                        ),
                        (
                            ResourceAddress::from("resource_tdx_2_1nfe6ugwjvuqc7aqhcltnwmp8xfkpqnm9mc8n9jh5xcvnx38r2wncre"),
                            NewlyCreatedResource::with(
                                "Ability",
                                "ABILITY",
                                "Ability: An amazingly innovative and rare NFT collection",
                                "https://image-service-test-images.s3.eu-west-2.amazonaws.com/wallet_test_images/KLHaze-medium.jpg",
                                ["Unique".to_string(), "FOMO".to_string(), "Advanced".to_string()]
                            )
                        )
                    ])
                )
            );
    }

    #[test]
    fn mint_nft_gumball_card() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "mint_nft_gumball_card.rtm"
        ));
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "mint_nft_gumball_card.dat"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [
                    (
                        acc_gk,
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1thqcgjw37fjgycpvqr52nx4jcsdeuq75mf2nywme07kzsuds9a4psp",
                                FungibleResourceIndicator::guaranteed(5)
                            ),
                        ]
                    )
                ],
                [
                    (
                        acc_gk,
                        vec![
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1ng88qk08hrgmad30rzdxpyx779yuta4cwcjc3gstk60jhachsv94g9",
                                NonFungibleResourceIndicator::by_amount(1, PredictedNonFungibleLocalIds::new(
                                    [
                                        NonFungibleLocalId::string("Member_44").unwrap()
                                    ],
                                    3
                                ))
                            )
                        ]
                    )
                ],
                [
                    acc_gk
                ], // addresses_of_accounts_requiring_auth
                [], // addresses_of_identities_requiring_auth
                ["resource_tdx_2_1ng88qk08hrgmad30rzdxpyx779yuta4cwcjc3gstk60jhachsv94g9:<Member_44>".parse::<NonFungibleGlobalId>().unwrap()], // newly_created_non_fungibles
                [], // reserved_instructions
                [], // presented_proofs
                ["component_tdx_2_1czg6rq9vms7t402fedtpzkjah25hh7snyu3ysgxk3pwlz4d3tugm7j".parse::<ManifestEncounteredComponentAddress>().unwrap()], // encountered_component_addresses
                [
                    DetailedManifestClass::General
                ],
                FeeLocks::default(),
                FeeSummary::new(
                    "0.3751137",
                    "0.0467599",
                    "0.14677047477",
                    0,
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn present_proof_swap_candy() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "present_proof_swap_candy.rtm"
        ));
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "present_proof_swap_candy.dat"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();

        pretty_assertions::assert_eq!(
                sut,
                SUT::new(
                    [
                        (
                            acc_gk,
                            vec![
                                ResourceIndicator::fungible(
                                    "resource_tdx_2_1thqcgjw37fjgycpvqr52nx4jcsdeuq75mf2nywme07kzsuds9a4psp",
                                    FungibleResourceIndicator::guaranteed(1)
                                ),
                            ]
                        )
                    ], // addresses_of_accounts_withdrawn_from
                    [
                        (
                            acc_gk,
                            vec![
                                ResourceIndicator::fungible(
                                     "resource_tdx_2_1tk30vj4ene95e3vhymtf2p35fzl29rv4us36capu2rz0vretw9gzr3",
                                    FungibleResourceIndicator::predicted(30, 4)
                                )
                            ]
                        ),
                    ], // addresses_of_accounts_deposited_into
                    [acc_gk], // addresses_of_accounts_requiring_auth
                    [], // addresses_of_identities_requiring_auth
                    [], // newly_created_non_fungibles
                    [], // reserved_instructions
                    [ResourceSpecifier::non_fungible("resource_tdx_2_1ng88qk08hrgmad30rzdxpyx779yuta4cwcjc3gstk60jhachsv94g9", vec!["<Member_44>".parse().unwrap()])], // presented_proofs
                    ["component_tdx_2_1crje3en7zsrna9t5vyywn3z3t9ht34l9udxjcpjvdhpcw9v6vlzru8".parse::<ManifestEncounteredComponentAddress>().unwrap()], // encountered_component_addresses
                    [
                        DetailedManifestClass::General
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.50142635",
                        "0.0467589",
                        "0.13551711803",
                        0,
                    ),
                    NewEntities::default()
                )
            );
    }

    #[test]
    fn create_pool() {
        let instructions_string =
            include_str!(concat!(env!("FIXTURES_TX"), "create_pool.rtm"));
        let encoded_receipt_hex =
            include_str!(concat!(env!("FIXTURES_TX"), "create_pool.dat"));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        pretty_assertions::assert_eq!(
                sut,
                SUT::new(
                    [], // addresses_of_account_withdrawls
                    [], // addresses_of_accounts_deposited_into
                    [], // addresses_of_accounts_requiring_auth
                    [], // addresses_of_identities_requiring_auth
                    [], // newly_created_non_fungibles
                    [], // reserved_instructions
                    [], // presented_proofs
                    [], // encountered_component_addresses
                    [
                        DetailedManifestClass::General
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.1495719",
                        "0.1557717",
                        "0.3290176335",
                        0,
                    ),
                    NewEntities::new([
                        (
                            "resource_tdx_2_1t57kl0kuvneehavfv9u0szngxwmek9qq4ecqv0yg59w83axqdak5xc".parse::<ResourceAddress>().unwrap(),
                             NewlyCreatedResource::empty()
                        )
                    ])
                )
            );
    }

    #[test]
    fn contribute_to_bi_pool() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "contribute_to_bi_pool.rtm"
        ));
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "contribute_to_bi_pool.dat"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();
        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        let pool_address = "pool_tdx_2_1ckfjmjswvvf6y635f8l89uunu9cwgnglhqdk8627wrpf8ultdx2vc3".parse::<PoolAddress>().unwrap();

        pretty_assertions::assert_eq!(
                sut,
                SUT::new(
                    [
                        (
                            acc_gk,
                            vec![
                                ResourceIndicator::fungible(
                                    ResourceAddress::sample_stokenet_xrd(),
                                    FungibleResourceIndicator::guaranteed(237)
                                ),
                                ResourceIndicator::fungible(
                                    r"resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy",
                                    FungibleResourceIndicator::guaranteed(1337)
                                ),
                            ]
                        )
                    ],
                    [
                        (
                            acc_gk,
                            vec![
                                ResourceIndicator::fungible(
                                    "resource_tdx_2_1thnhmen4wg29tnqrfpk9w2v90s64z8at9sethnjma76866rfvcc2gs",
                                    FungibleResourceIndicator::predicted("562.91118304755680169", 5)
                                ),
                            ]
                        )
                    ], // addresses_of_accounts_deposited_into
                    [acc_gk], // addresses_of_accounts_requiring_auth
                    [], // addresses_of_identities_requiring_auth
                    [], // newly_created_non_fungibles
                    [], // reserved_instructions
                    [], // presented_proofs
                    [], // encountered_component_addresses
                    [
                        DetailedManifestClass::PoolContribution {
                            pool_addresses: vec![pool_address],
                            pool_contributions: vec![
                                TrackedPoolContribution::new(
                                    pool_address,
                                    [
                                        ("resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy".parse::<ResourceAddress>().unwrap(), Decimal::from(1337)),
                                        (ResourceAddress::sample_stokenet_xrd(), Decimal::from(237)),
                                    ],
                                    "resource_tdx_2_1thnhmen4wg29tnqrfpk9w2v90s64z8at9sethnjma76866rfvcc2gs".parse::<ResourceAddress>().unwrap(),
                                    Decimal::from_str("562.91118304755680169").unwrap()
                                )
                            ]
                        }
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.27435815",
                        "0.04276125",
                        "0.17910003354",
                        0,
                    ),
                    NewEntities::default()
                )
            );
    }

    #[test]
    fn stake_to_three_validators() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "stake_to_three_validators.rtm"
        ));
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "stake_to_three_validators.dat"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();

        let validator_0: ValidatorAddress = "validator_tdx_2_1sdatqsl6rx05yy2yvpf6ckfl7x8dluvzkcyljkn0x4lxkgucc0xz2w".parse().unwrap();
        let validator_0_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1th6hufew82dpntmcn7kt9f7au50cr59996tawh4syph0kz5e99v2u6".parse().unwrap();

        let validator_1: ValidatorAddress = "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".parse().unwrap();
        let validator_1_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t45l9ku3r5mwxazht2qutmhhk3660hqqvxkkyl8rxs20n9k2zv0w7t".parse().unwrap();

        let validator_2: ValidatorAddress = "validator_tdx_2_1sdlkptcwjpajqawnuya8r2mgl3eqt89hw27ww6du8kxmx3thmyu8l4".parse().unwrap();
        let validator_2_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t5hpjckz9tm63gqvxsl60ejhzvnlguly77tltvywnj06s2x9wjdxjn".parse().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [(
                    acc_gk,
                    vec![ResourceIndicator::fungible(
                        ResourceAddress::sample_stokenet_xrd(),
                        FungibleResourceIndicator::guaranteed(3566)
                    )]
                )], // addresses_of_accounts_withdrawn_from
                [(
                    acc_gk,
                    vec![
                        ResourceIndicator::fungible(
                            validator_0_resource_address_of_stake,
                            FungibleResourceIndicator::predicted(11, 3)
                        ),
                        ResourceIndicator::fungible(
                            validator_1_resource_address_of_stake,
                            FungibleResourceIndicator::predicted(222, 7)
                        ),
                        ResourceIndicator::fungible(
                            validator_2_resource_address_of_stake,
                            FungibleResourceIndicator::predicted(3333, 11)
                        ),
                    ]
                )], // addresses_of_accounts_deposited_into
                [acc_gk], // addresses_of_accounts_requiring_auth
                [],       // addresses_of_identities_requiring_auth
                [],       // newly_created_non_fungibles
                [],       // reserved_instructions
                [],       // presented_proofs
                [],       // encountered_component_addresses
                [DetailedManifestClass::ValidatorStake {
                    validator_addresses: vec![
                        validator_0,
                        validator_1,
                        validator_2
                    ],
                    validator_stakes: vec![
                        TrackedValidatorStake::new(
                            validator_0,
                            11,
                            validator_0_resource_address_of_stake,
                            11
                        ),
                        TrackedValidatorStake::new(
                            validator_1,
                            222,
                            validator_1_resource_address_of_stake,
                            222
                        ),
                        TrackedValidatorStake::new(
                            validator_2,
                            3333,
                            validator_2_resource_address_of_stake,
                            3333
                        ),
                    ]
                }],
                FeeLocks::default(),
                FeeSummary::new("0.34071685", "0.1150347", "0.32796859177", 0,),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn redeem_from_bi_pool() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "redeem_from_bi_pool.rtm"
        ));
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "redeem_from_bi_pool.dat"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_gk = AccountAddress::from("account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk");
        let resource_address_of_pool = ResourceAddress::from("resource_tdx_2_1thnhmen4wg29tnqrfpk9w2v90s64z8at9sethnjma76866rfvcc2gs");
        let pool_address = PoolAddress::from("pool_tdx_2_1ckfjmjswvvf6y635f8l89uunu9cwgnglhqdk8627wrpf8ultdx2vc3");
        let token0 = ResourceAddress::from("resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy");
        let token1 = ResourceAddress::sample_stokenet_xrd();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [(
                    acc_gk,
                    vec![ResourceIndicator::fungible(
                        resource_address_of_pool,
                        FungibleResourceIndicator::guaranteed(500)
                    )]
                )], // addresses_of_accounts_withdrawn_from
                [(
                    acc_gk,
                    vec![
                        ResourceIndicator::fungible(
                            ResourceAddress::sample_stokenet_xrd(),
                            FungibleResourceIndicator::predicted(
                                "210.512783488241137505",
                                3
                            )
                        ),
                        ResourceIndicator::fungible(
                            token0,
                            FungibleResourceIndicator::predicted(
                                "1187.5763355433",
                                3
                            )
                        )
                    ]
                )], // addresses_of_accounts_deposited_into
                [acc_gk], // addresses_of_accounts_requiring_auth
                [],       // addresses_of_identities_requiring_auth
                [],       // newly_created_non_fungibles
                [],       // reserved_instructions
                [],       // presented_proofs
                [],       // encountered_component_addresses
                [DetailedManifestClass::PoolRedemption {
                    pool_addresses: vec![pool_address],
                    pool_redemptions: vec![TrackedPoolRedemption::new(
                        pool_address,
                        resource_address_of_pool,
                        500,
                        [
                            (token0, "1187.5763355433".parse().unwrap()),
                            (token1, "210.512783488241137505".parse().unwrap()),
                        ]
                    )]
                }],
                FeeLocks::default(),
                FeeSummary::new("0.25753315", "0.0325088", "0.12760162134", 0,),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn unstake_partially_from_one_validator() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "unstake_partially_from_one_validator.rtm"
        ));
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "unstake_partially_from_one_validator.dat"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".into();

        let nf_global_id: NonFungibleGlobalId = "resource_tdx_2_1ngw8z6ut9mw54am4rr65kwcuz24q3n7waxtzyfvug5g4yuc00jydqj:{192ed08c15075e36-ec4892a8ba3b86f1-a1e050a6563b787e-adc9813f7fc90480}".into();

        let validator: ValidatorAddress = "validator_tdx_2_1sdlkptcwjpajqawnuya8r2mgl3eqt89hw27ww6du8kxmx3thmyu8l4".into();

        pretty_assertions::assert_eq!(
                sut,
                SUT::new(
                    [
                        (
                           acc_gk,
                            vec![
                                ResourceIndicator::fungible(
                                    "resource_tdx_2_1t5hpjckz9tm63gqvxsl60ejhzvnlguly77tltvywnj06s2x9wjdxjn", 
                                    FungibleResourceIndicator::guaranteed(1234)
                                ),
                            ]
                        )
                    ], // addresses_of_accounts_withdrawn_from
                    [
                      (
                        acc_gk,
                        vec![
                            (
                               ResourceIndicator::non_fungible(
                                "resource_tdx_2_1ngw8z6ut9mw54am4rr65kwcuz24q3n7waxtzyfvug5g4yuc00jydqj",
                                NonFungibleResourceIndicator::by_all(
                                    PredictedDecimal::new(1, 3),
                                    PredictedNonFungibleLocalIds::new(
                                        [
                                            NonFungibleLocalId::ruid(hex_decode("192ed08c15075e36ec4892a8ba3b86f1a1e050a6563b787eadc9813f7fc90480").unwrap()).unwrap()
                                        ],
                                        3
                                    )
                                )
                            )
                            )
                        ]
                      )
                    ], // addresses_of_accounts_deposited_into
                    [acc_gk],
                    [], // addresses_of_identities_requiring_auth
                    [nf_global_id.clone()], // newly_created_non_fungibles
                    [], // reserved_instructions
                    [], // presented_proofs
                    [], // encountered_component_addresses
                    [
                        DetailedManifestClass::ValidatorUnstake {
                            validator_addresses: vec![validator],
                            claims_non_fungible_data: HashMap::<NonFungibleGlobalId, UnstakeData>::from_iter([(
                                nf_global_id,
                                UnstakeData::new("Stake Claim", 42215, 1234)
                            )])
                        }
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.2788849",
                        "0.06251535",
                        "0.16927718825",
                        0,
                    ),
                    NewEntities::default()
                )
            );
    }

    #[test]
    fn claim_two_validator_stakes() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "claim_two_validator_stakes.rtm"
        ));
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "claim_two_validator_stakes.dat"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".into();

        let validator_1: ValidatorAddress = "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".into();
        let validator_1_resource_address_of_stake: NonFungibleResourceAddress = "resource_tdx_2_1ng3g2nj5pfpmdphgz0nrh8z0gtqcxx5z5dn48t85ar0z0zjhefufaw".into();

        let validator_2: ValidatorAddress = "validator_tdx_2_1sdlkptcwjpajqawnuya8r2mgl3eqt89hw27ww6du8kxmx3thmyu8l4".into();
        let validator_2_resource_address_of_stake: NonFungibleResourceAddress = "resource_tdx_2_1ngw8z6ut9mw54am4rr65kwcuz24q3n7waxtzyfvug5g4yuc00jydqj".into();

        pretty_assertions::assert_eq!(
                sut,
                SUT::new(
                    [
                        (
                            acc_gk,
                            vec![
                                ResourceIndicator::non_fungible(
                                    "resource_tdx_2_1ng3g2nj5pfpmdphgz0nrh8z0gtqcxx5z5dn48t85ar0z0zjhefufaw",
                                    NonFungibleResourceIndicator::by_ids([
                                        NonFungibleLocalId::ruid(hex_decode("97c2b05d8529be58152d79c176d61d68f87611f279e0daa3d486426d5330795c").unwrap()).unwrap()
                                    ])
                                ),
                                ResourceIndicator::non_fungible(
                                    "resource_tdx_2_1ngw8z6ut9mw54am4rr65kwcuz24q3n7waxtzyfvug5g4yuc00jydqj",
                                    NonFungibleResourceIndicator::by_ids([
                                        NonFungibleLocalId::ruid(hex_decode("f1edc2f0f8f54d33dab8e1bf90e196ce9714ef7b85478c6c82486b47a79b3002").unwrap()).unwrap()
                                    ])
                                ),
                            ]
                        )
                    ], // addresses_of_accounts_withdrawn_from
                    [
                        (
                            acc_gk,
                            vec![
                                ResourceIndicator::fungible(
                                    ResourceAddress::sample_stokenet_xrd(),
                                    FungibleResourceIndicator::guaranteed(110)
                                ),
                                ResourceIndicator::fungible(
                                    ResourceAddress::sample_stokenet_xrd(),
                                    FungibleResourceIndicator::guaranteed(1234)
                        ),
                            ]
                        ),
                    ], // addresses_of_accounts_deposited_into
                    [acc_gk], // addresses_of_accounts_requiring_auth
                    [], // addresses_of_identities_requiring_auth
                    [], // newly_created_non_fungibles
                    [], // reserved_instructions
                    [], // presented_proofs
                    [], // encountered_component_addresses
                    [
                        DetailedManifestClass::ValidatorClaim {
                            validator_addresses: vec![validator_1, validator_2],
                            validator_claims: vec![
                                TrackedValidatorClaim::new(
                                    validator_1,
                                    validator_1_resource_address_of_stake,
                                    [
                                        NonFungibleLocalId::from("{97c2b05d8529be58-152d79c176d61d68-f87611f279e0daa3-d486426d5330795c}")
                                    ],
                                    110
                                ),
                                TrackedValidatorClaim::new(
                                    validator_2,
                                    validator_2_resource_address_of_stake,
                                    [
                                        NonFungibleLocalId::from("{f1edc2f0f8f54d33-dab8e1bf90e196ce-9714ef7b85478c6c-82486b47a79b3002}")
                                    ],
                                    1234
                                ),
                            ]
                        }
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.30518895",
                        "0.05851055",
                        "0.1916885343",
                        0,
                    ),
                    NewEntities::default()
                )
            );
    }

    #[test]
    fn account_locker_claim_fungibles_and_non_fungibles() {
        let encoded_receipt_hex = include_str!(concat!(
            env!("FIXTURES_TX"),
            "account_locker_claim_fungibles_and_non_fungibles.dat"
        ));
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "account_locker_claim_fungibles_and_non_fungibles.rtm"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest
            .execution_summary(
                BagOfBytes::from_hex(encoded_receipt_hex).unwrap(),
            )
            .unwrap();

        let acc: AccountAddress = "account_tdx_2_12xlu6x99ssrwrs8cnafka8476ursxfyfde3kfyk7d4s9c5kdvcs77x".into();
        let fungible_address: ResourceAddress = "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".into();
        let non_fungible_address: ResourceAddress = "resource_tdx_2_1nflxr7dvp29hxhjjp53strsdgv2kv9dxlx6ys52lafrgyljrhwkt27".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [],
                [(
                    acc,
                    vec![
                        ResourceIndicator::non_fungible(
                            non_fungible_address,
                            NonFungibleResourceIndicator::by_amount(4, PredictedNonFungibleLocalIds::new(
                                [
                                    NonFungibleLocalId::integer(3),
                                    NonFungibleLocalId::integer(5),
                                    NonFungibleLocalId::integer(4),
                                    NonFungibleLocalId::integer(6)
                                ],
                                1
                            ))
                        ),
                        ResourceIndicator::fungible(
                            fungible_address,
                            FungibleResourceIndicator::guaranteed(30)
                        ),
                    ]
                )],
                vec!["account_tdx_2_12xlu6x99ssrwrs8cnafka8476ursxfyfde3kfyk7d4s9c5kdvcs77x".into()], // addresses_of_accounts_requiring_auth
                [], // addresses_of_identities_requiring_auth
                [], // newly_created_non_fungibles
                [], // reserved_instructions
                [], // presented_proofs
                ["locker_tdx_2_1drgp40wpu5cj0zady4s0pec6rld8muge0j2xx9xuwwc474uzlgja6a".parse::<ManifestEncounteredComponentAddress>().unwrap()],
                [DetailedManifestClass::General],
                FeeLocks::default(),
                FeeSummary::new("0.2674585", "0.07226045", "0.19378661776", 0,),
                NewEntities::default()
            )
        );
    }
}
