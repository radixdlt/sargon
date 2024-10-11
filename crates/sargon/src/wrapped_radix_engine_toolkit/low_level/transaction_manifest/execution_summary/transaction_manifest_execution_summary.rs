use crate::prelude::*;

use radix_engine_toolkit::functions::manifest::execution_summary as RET_execution_summary;

impl TransactionManifest {
    /// Creates the `ExecutionSummary` based on the `engine_toolkit_receipt`.
    ///
    /// Such value should be obtained from the Gateway `/transaction/preview` endpoint, under the `radix_engine_toolkit_receipt` field.
    pub fn execution_summary(
        &self,
        engine_toolkit_receipt: ScryptoSerializableToolkitTransactionReceipt,
    ) -> Result<ExecutionSummary> {
        let network_definition = self.network_id().network_definition();
        let runtime_receipt = engine_toolkit_receipt
            .into_runtime_receipt(&ScryptoAddressBech32Decoder::new(
                &network_definition,
            ))
            .ok()
            .ok_or(CommonError::FailedToDecodeEngineToolkitReceipt)?;
        self.execution_summary_with_receipt(runtime_receipt)
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
    fn failure_if_receipt_result_is_abort() {
        let wrong_receipt = ScryptoRuntimeToolkitTransactionReceipt::Abort {
            reason: "whatever".to_owned(),
        };

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
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "transfer_1to2_multiple_nf_and_f_tokens.rtm"
        ));
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "transfer_1to2_multiple_nf_and_f_tokens.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_v3: AccountAddress = "account_tdx_2_12x55t8w9lf2qjh6z98jjxy04tkpjgjxawqm252gerhyath7qka34v3".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [
                    (
                        acc_v3,
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc",
                                FungibleResourceIndicator::guaranteed("10")
                            ),
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1t4nnqzrdympy0n74yhvkp7vnver90j5yurlhqu3083z8mt2gdvu7sq",
                                FungibleResourceIndicator::guaranteed("5")
                            ),
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1n2uml563pnl0yjmd57xnj80mzdwyh4ca4w29zt2ljwpwq2p837c4a7",
                                NonFungibleResourceIndicator::by_ids([
                                    NonFungibleLocalId::integer(64),
                                ])
                            ),
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue",
                                NonFungibleResourceIndicator::by_ids([
                                    NonFungibleLocalId::string("Member_6").unwrap(),
                                ])
                            ),
                        ]
                    )
                ], //withdrawals
                [
                    (
                        AccountAddress::from("account_tdx_2_129n8v0x9q2zj78293sv7xhz9zcysvcvwp70pc6826k8f7dte96sfcn"),
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc",
                                FungibleResourceIndicator::guaranteed("10")
                            ),
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1n2uml563pnl0yjmd57xnj80mzdwyh4ca4w29zt2ljwpwq2p837c4a7",
                                NonFungibleResourceIndicator::by_ids([
                                    NonFungibleLocalId::integer(64),
                                ])
                            ),
                        ]
                    ),
                    (
                        AccountAddress::from("account_tdx_2_12x58hfy598wl5vukv3sqrkc7h3n699nqygufncycayeuwydel8esnu"),
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1t4nnqzrdympy0n74yhvkp7vnver90j5yurlhqu3083z8mt2gdvu7sq",
                                FungibleResourceIndicator::guaranteed(5)
                            ),
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue",
                                NonFungibleResourceIndicator::by_ids([
                                    NonFungibleLocalId::string("Member_6").unwrap(),
                                ])
                            ),
                        ]
                    )
                ], //deposits
                [acc_v3],
                [],
                [],
                [ReservedInstruction::AccountLockFee],
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
                    "0.37765305",
                    "0.10476895",
                    "0.3614425597",
                    0,
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn third_party_deposits_update() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "third_party_deposits_update.rtm"
        ));
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "third_party_deposits_update.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_g2: AccountAddress = "account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [],
                [],
                [acc_g2], // addresses_of_accounts_requiring_auth
                [],               // addresses_of_identities_requiring_auth
                [],               // newly_created_non_fungibles
                [ReservedInstruction::AccountLockFee, ReservedInstruction::AccountUpdateSettings],
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
                                "resource_tdx_2_1t4nnqzrdympy0n74yhvkp7vnver90j5yurlhqu3083z8mt2gdvu7sq".into(),
                                ResourcePreferenceUpdate::Remove,
                            )]),
                        )]),
                        deposit_mode_updates:
                            HashMap::<AccountAddress, DepositRule>::from_iter([(
                                acc_g2,
                                DepositRule::DenyAll,
                            )]),
                        authorized_depositors_added: HashMap::new(),
                        authorized_depositors_removed: HashMap::new(),
                    }
                ],
                FeeLocks::default(),
                FeeSummary::new(
                    "0.07638415",
                    "0.0105008",
                    "0.03871917658",
                    0
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn create_single_fungible() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_single_fungible.rtm"
        ));
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_single_fungible.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_v3: AccountAddress = "account_tdx_2_12x55t8w9lf2qjh6z98jjxy04tkpjgjxawqm252gerhyath7qka34v3".into();
        let token_address: ResourceAddress = "resource_tdx_2_1tkd0xf49tvq4wjdxn7khkt9l900750rd2jqdajukgwsyv8k9md5hh6".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [],
                [(
                    acc_v3,
                    vec![ResourceIndicator::fungible(
                        token_address,
                        FungibleResourceIndicator::predicted(21000000, 1)
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
                FeeSummary::new("0.1585925", "0.1210119", "0.26540755769", 0,),
                NewEntities::new([(
                    token_address,
                    NewlyCreatedResource::empty()
                )])
            )
        );
    }

    #[test]
    fn create_nft_collection() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_nft_collection.rtm"
        ));
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_nft_collection.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc: AccountAddress = "account_tdx_2_12x55t8w9lf2qjh6z98jjxy04tkpjgjxawqm252gerhyath7qka34v3".into();
        let non_fungible_address: ResourceAddress = "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl".into();

        pretty_assertions::assert_eq!(
                sut,
                SUT::new(
                    [], // addresses_of_accounts_withdrawn_from
                    [(
                        acc,
                        vec![
                            ResourceIndicator::non_fungible(
                                non_fungible_address,
                                NonFungibleResourceIndicator::by_all(
                                    PredictedDecimal::new(10, 1),
                                    PredictedNonFungibleLocalIds::new(
                                        [
                                            NonFungibleLocalId::integer(0),
                                            NonFungibleLocalId::integer(1),
                                            NonFungibleLocalId::integer(2),
                                            NonFungibleLocalId::integer(3),
                                            NonFungibleLocalId::integer(4),
                                            NonFungibleLocalId::integer(5),
                                            NonFungibleLocalId::integer(6),
                                            NonFungibleLocalId::integer(7),
                                            NonFungibleLocalId::integer(8),
                                            NonFungibleLocalId::integer(9),
                                        ],
                                        1
                                    )
                                )
                            ),
                        ]
                    )], // addresses_of_accounts_deposited_into
                    [], // addresses_of_accounts_requiring_auth
                    [], // addresses_of_identities_requiring_auth
                    [
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#0#",
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#1#", 
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#2#",
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#3#",
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#4#",
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#5#",
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#6#",
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#7#",
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#8#",
                        "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl:#9#",
                    ].into_iter().map(NonFungibleGlobalId::from_str).map(Result::unwrap).collect_vec(), // newly_created_non_fungibles
                    [], // reserved_instructions
                    [], // presented_proofs
                    [], // encountered_component_addresses
                    [
                        DetailedManifestClass::General
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.18451315",
                        "0.40604035",
                        "0.96845625165",
                        0,
                    ),
                    NewEntities::new([
                        (non_fungible_address, NewlyCreatedResource::default())
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
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "mint_nft_gumball_card.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_ac: AccountAddress = "account_tdx_2_129qq7m9ttup2kn6t4g4s0dvazxplktj7vd7my76hfd7xh7ham5zeac".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [
                    (
                        acc_ac,
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck",
                                FungibleResourceIndicator::guaranteed(5)
                            ),
                        ]
                    )
                ],
                [
                    (
                        acc_ac,
                        vec![
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue",
                                NonFungibleResourceIndicator::by_amount(1, PredictedNonFungibleLocalIds::new(
                                    [],
                                    4
                                ))
                            )
                        ]
                    )
                ],
                [
                    acc_ac,
                ], // addresses_of_accounts_requiring_auth
                [], // addresses_of_identities_requiring_auth
                ["resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue:<Member_83>".parse::<NonFungibleGlobalId>().unwrap()], // newly_created_non_fungibles
                [ReservedInstruction::AccountLockFee], // reserved_instructions
                [], // presented_proofs
                ["component_tdx_2_1cpd3cgy9kaxvxlptkkgxkm3qvfyqkrsl03kyz532p7e2gk0ygs4xrd".parse::<ManifestEncounteredComponentAddress>().unwrap()], // encountered_component_addresses
                [
                    DetailedManifestClass::General
                ],
                FeeLocks::default(),
                FeeSummary::new(
                    "0.3737913",
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
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "present_proof_swap_candy.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_ac: AccountAddress = "account_tdx_2_129qq7m9ttup2kn6t4g4s0dvazxplktj7vd7my76hfd7xh7ham5zeac".parse().unwrap();

        pretty_assertions::assert_eq!(
                sut,
                SUT::new(
                    [
                        (
                            acc_ac,
                            vec![
                                ResourceIndicator::fungible(
                                    "resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck",
                                    FungibleResourceIndicator::guaranteed(1)
                                ),
                            ]
                        )
                    ], // addresses_of_accounts_withdrawn_from
                    [], // addresses_of_accounts_deposited_into
                    [acc_ac], // addresses_of_accounts_requiring_auth
                    [], // addresses_of_identities_requiring_auth
                    [], // newly_created_non_fungibles
                    [ReservedInstruction::AccountLockFee], // reserved_instructions
                    [ResourceSpecifier::non_fungible("resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue", vec!["<Member_83>".parse().unwrap()])], // presented_proofs
                    ["component_tdx_2_1cr4pa9ex9xhwzfjzclv8vjnfylw93wvhkwcwc0xlahpkel0krxqedw".parse::<ManifestEncounteredComponentAddress>().unwrap()], // encountered_component_addresses
                    [
                        DetailedManifestClass::General
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.4943021",
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

        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_pool.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

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
                [DetailedManifestClass::General],
                FeeLocks::default(),
                FeeSummary::new("0.15184175", "0.1607719", "0.33388137243", 0,),
                NewEntities::new([
                    (ResourceAddress::try_from_bech32("resource_tdx_2_1tkrg7uwrc9sw3tkckuxwu65euwppxa00f7aqtx5ujd0aa22gej6nm7").unwrap(), NewlyCreatedResource::default())
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

        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "contribute_to_bi_pool.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();
        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        let pool_address = "pool_tdx_2_1ckfjmjswvvf6y635f8l89uunu9cwgnglhqdk8627wrpf8ultdx2vc3".parse::<PoolAddress>().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [(
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
                )],
                [],       // addresses_of_accounts_deposited_into
                [acc_gk], // addresses_of_accounts_requiring_auth
                [],       // addresses_of_identities_requiring_auth
                [],       // newly_created_non_fungibles
                [],       // reserved_instructions
                [],       // presented_proofs
                [],       // encountered_component_addresses
                [DetailedManifestClass::PoolContribution {
                    pool_addresses: vec![pool_address],
                    pool_contributions: vec![]
                }],
                FeeLocks::new(0.36962, 0),
                FeeSummary::new("0.1493972", "0.01125345", "0.0782012926", 0,),
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
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "stake_to_three_validators.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a".parse().unwrap();

        let validator_0: ValidatorAddress = "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".parse().unwrap();
        let validator_0_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t45l9ku3r5mwxazht2qutmhhk3660hqqvxkkyl8rxs20n9k2zv0w7t".parse().unwrap();

        let validator_1: ValidatorAddress = "validator_tdx_2_1sdlkptcwjpajqawnuya8r2mgl3eqt89hw27ww6du8kxmx3thmyu8l4".parse().unwrap();
        let validator_1_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t5hpjckz9tm63gqvxsl60ejhzvnlguly77tltvywnj06s2x9wjdxjn".parse().unwrap();

        let validator_2: ValidatorAddress = "validator_tdx_2_1svr6rmtd9ts5zx8d3euwmmp6mmjdtcj2q7zlmd8xjrn4qx7q5snkas".parse().unwrap();
        let validator_2_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t48zl3qmcv3pf24r0765q4zc6rrk83cfjv6wza2xksej80pcfd7p5g".parse().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [(
                    acc_gk,
                    vec![ResourceIndicator::fungible(
                        ResourceAddress::sample_stokenet_xrd(),
                        FungibleResourceIndicator::guaranteed(3000)
                    )]
                )], // addresses_of_accounts_withdrawn_from
                [(
                    acc_gk,
                    vec![
                        ResourceIndicator::fungible(
                            validator_0_resource_address_of_stake,
                            FungibleResourceIndicator::predicted(0, 5)
                        ),
                        ResourceIndicator::fungible(
                            validator_1_resource_address_of_stake,
                            FungibleResourceIndicator::predicted(0, 10)
                        ),
                        ResourceIndicator::fungible(
                            validator_2_resource_address_of_stake,
                            FungibleResourceIndicator::predicted(0, 15)
                        ),
                    ]
                )], // addresses_of_accounts_deposited_into
                [acc_gk], // addresses_of_accounts_requiring_auth
                [],       // addresses_of_identities_requiring_auth
                [],       // newly_created_non_fungibles
                [ReservedInstruction::AccountLockFee], // reserved_instructions
                [],       // presented_proofs
                [],       // encountered_component_addresses
                [DetailedManifestClass::ValidatorStake {
                    validator_addresses: vec![
                        validator_0,
                        validator_1,
                        validator_2
                    ],
                    validator_stakes: vec![]
                }],
                FeeLocks::default(),
                FeeSummary::new("0.3527215", "0.1150347", "0.32796859177", 0,),
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
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "redeem_from_bi_pool.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk = AccountAddress::from("account_tdx_2_12x55t8w9lf2qjh6z98jjxy04tkpjgjxawqm252gerhyath7qka34v3");
        let resource_address_of_pool = ResourceAddress::from("resource_tdx_2_1thgnc84xkcjhs46pfvm9s8zn8t9kxwryvyr9x3947xpt6jxty7qn25");
        let pool_address = PoolAddress::from("pool_tdx_2_1c5mygu9t8rlfq6j8v2ynrg60ltts2dctsghust8u2tuztrml427830");

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [(
                    acc_gk,
                    vec![ResourceIndicator::fungible(
                        resource_address_of_pool,
                        FungibleResourceIndicator::guaranteed(1)
                    )]
                )], // addresses_of_accounts_withdrawn_from
                [], // addresses_of_accounts_deposited_into
                [acc_gk], // addresses_of_accounts_requiring_auth
                [],       // addresses_of_identities_requiring_auth
                [],       // newly_created_non_fungibles
                [ReservedInstruction::AccountLockFee],       // reserved_instructions
                [],       // presented_proofs
                [],       // encountered_component_addresses
                [DetailedManifestClass::PoolRedemption {
                    pool_addresses: vec![pool_address],
                    pool_redemptions: vec![TrackedPoolRedemption::new(
                        pool_address,
                        ResourceAddress::from("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"),
                        3.162277,
                        []
                    )]
                }],
                FeeLocks::default(),
                FeeSummary::new("0.26154965", "0.0325088", "0.12760162134", 0,),
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
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "unstake_partially_from_one_validator.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a".into();

        let nf_global_id: NonFungibleGlobalId = "resource_tdx_2_1ngw8z6ut9mw54am4rr65kwcuz24q3n7waxtzyfvug5g4yuc00jydqj:{8a190d8fd0725713-e9072f0fd954196f-5f9be7adaf8d5b78-cf811ea9992983c3}".into();

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
                                    FungibleResourceIndicator::guaranteed(500)
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
                                    PredictedDecimal::new(0, 4),
                                    PredictedNonFungibleLocalIds::new(
                                        [],
                                        4
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
                    [ReservedInstruction::AccountLockFee], // reserved_instructions
                    [], // presented_proofs
                    [], // encountered_component_addresses
                    [
                        DetailedManifestClass::ValidatorUnstake {
                            validator_addresses: vec![validator],
                            claims_non_fungible_data: HashMap::<NonFungibleGlobalId, UnstakeData>::new(),
                        }
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.2848875",
                        "0.06251535",
                        "0.16927718825",
                        0,
                    ),
                    NewEntities::default()
                )
            );
    }

    #[test]
    fn claim_two_stakes_from_one_validator() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "claim_two_stakes_from_one_validator.rtm"
        ));
        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "claim_two_stakes_from_one_validator.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a".into();

        let validator: ValidatorAddress = "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".into();

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
                                        NonFungibleLocalId::ruid(hex_decode("1c1ce92c810094a765659db6a666c19c6cea4367bb789b55276b137712ceecce").unwrap()).unwrap(),
                                        NonFungibleLocalId::ruid(hex_decode("5aebd0270caf3f8751031498741f57b5d24fe0d62a976589519c6a92423888cc").unwrap()).unwrap()
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
                                    FungibleResourceIndicator::guaranteed(150)
                                ),
                            ]
                        ),
                    ], // addresses_of_accounts_deposited_into
                    [acc_gk], // addresses_of_accounts_requiring_auth
                    [], // addresses_of_identities_requiring_auth
                    [], // newly_created_non_fungibles
                    [ReservedInstruction::AccountLockFee], // reserved_instructions
                    [], // presented_proofs
                    [], // encountered_component_addresses
                    [
                        DetailedManifestClass::ValidatorClaim {
                            validator_addresses: vec![validator],
                            validator_claims: vec![]
                        }
                    ],
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.2383276",
                        "0.041757",
                        "0.11224746511",
                        0,
                    ),
                    NewEntities::default()
                )
            );
    }

    #[test]
    fn account_locker_claim_fungibles_and_non_fungibles() {
        let instructions_string = include_str!(concat!(
            env!("FIXTURES_TX"),
            "account_locker_claim_fungibles_and_non_fungibles.rtm"
        ));

        let receipt = deserialize_receipt(include_str!(concat!(
            env!("FIXTURES_TX"),
            "account_locker_claim_fungibles_and_non_fungibles.dat"
        )));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc: AccountAddress = "account_tdx_2_12x2lmewv5lfen4x96aurw7a5z5ukdzyyc0fkytamqgml77lah44kkp".into();
        let fungible_address: ResourceAddress = "resource_tdx_2_1th75jg2gx9l3v0r8duzrmknfarhd3ha0387lg9n78qc9849xsfyq32".into();
        let non_fungible_address: ResourceAddress = "resource_tdx_2_1n2z4k99wuqlph9lj64ckc64znm48axl37xctsa0xqmm2sqg7klrte3".into();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [],
                [(
                    acc,
                    vec![
                        ResourceIndicator::non_fungible(
                            non_fungible_address,
                            NonFungibleResourceIndicator::by_amount(1, PredictedNonFungibleLocalIds::new(
                                [
                                ],
                                2
                            ))
                        ),
                        ResourceIndicator::fungible(
                            fungible_address,
                            FungibleResourceIndicator::guaranteed(3)
                        ),
                    ]
                )],
                vec!["account_tdx_2_12x2lmewv5lfen4x96aurw7a5z5ukdzyyc0fkytamqgml77lah44kkp".into()], // addresses_of_accounts_requiring_auth
                [], // addresses_of_identities_requiring_auth
                [], // newly_created_non_fungibles
                [ReservedInstruction::AccountLockFee], // reserved_instructions
                [], // presented_proofs
                ["locker_tdx_2_1dr6v4fwufgacxqwxsm44ysglhdv7yyxgvq6xazcwzvu35937wzsjnx".parse::<ManifestEncounteredComponentAddress>().unwrap()],
                [DetailedManifestClass::General],
                FeeLocks::default(),
                FeeSummary::new("0.2516311", "0.03200635", "0.12903213279", 0,),
                NewEntities::default()
            )
        );
    }

    fn deserialize_receipt(
        value: impl AsRef<str>,
    ) -> ScryptoSerializableToolkitTransactionReceipt {
        serde_json::from_str::<ScryptoSerializableToolkitTransactionReceipt>(
            &value.as_ref(),
        )
        .unwrap()
    }
}
