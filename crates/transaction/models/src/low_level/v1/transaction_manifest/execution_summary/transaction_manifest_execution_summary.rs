use crate::prelude::*;

use radix_engine_toolkit::functions::transaction_v1::manifest::dynamically_analyze as RET_dynamically_analyze;

impl TransactionManifest {
    /// Creates the `ExecutionSummary` based on the `engine_toolkit_receipt`.
    ///
    /// Such value should be obtained from the Gateway `/transaction/preview` endpoint, under the `radix_engine_toolkit_receipt` field.
    pub fn execution_summary(
        &self,
        engine_toolkit_receipt: ScryptoSerializableToolkitTransactionReceipt,
    ) -> Result<ExecutionSummary> {
        DynamicallyAnalyzableManifest::execution_summary(
            self,
            engine_toolkit_receipt,
            self.network_id(),
        )
    }
}

impl DynamicallyAnalyzableManifest for TransactionManifest {
    fn ret_dynamically_analyze(
        &self,
        receipt: ScryptoRuntimeToolkitTransactionReceipt,
    ) -> Result<RetDynamicAnalysis, RetManifestAnalysisError> {
        RET_dynamically_analyze(&self.scrypto_manifest(), receipt)
    }
}

impl Default for FeeLocks {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use prelude::{fixture_rtm, fixture_tx};

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ExecutionSummary;

    #[test]
    fn failure_if_receipt_result_is_abort() {
        let wrong_receipt =
            ScryptoSerializableToolkitTransactionReceipt::Abort {
                reason: "whatever".to_owned(),
            };

        assert_eq!(
            TransactionManifest::sample().execution_summary(wrong_receipt),
            Err(CommonError::ExecutionSummaryFail {
                underlying: "NotACommitSuccessReceipt".to_owned()
            })
        );
    }

    #[test]
    fn transfer_1to2_multiple_nf_and_f_tokens() {
        let instructions_string =
            fixture_rtm!("transfer_1to2_multiple_nf_and_f_tokens");

        let receipt = deserialize_receipt(fixture_tx!(
            "transfer_1to2_multiple_nf_and_f_tokens"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_v3 = "account_tdx_2_12x55t8w9lf2qjh6z98jjxy04tkpjgjxawqm252gerhyath7qka34v3".parse::<AccountAddress>().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [
                    (
                        acc_v3,
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(),
                                FungibleResourceIndicator::new_guaranteed(10)
                            ),
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1t4nnqzrdympy0n74yhvkp7vnver90j5yurlhqu3083z8mt2gdvu7sq".parse::<ResourceAddress>().unwrap(),
                                FungibleResourceIndicator::new_guaranteed(5)
                            ),
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1n2uml563pnl0yjmd57xnj80mzdwyh4ca4w29zt2ljwpwq2p837c4a7".parse::<ResourceAddress>().unwrap(),
                                NonFungibleResourceIndicator::new_guaranteed([
                                    NonFungibleLocalId::integer(64),
                                ])
                            ),
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue".parse::<ResourceAddress>().unwrap(),
                                NonFungibleResourceIndicator::new_guaranteed([
                                    NonFungibleLocalId::string("Member_6").unwrap(),
                                ])
                            ),
                        ]
                    )
                ], //withdrawals
                [
                    (
                        AccountAddress::from_str("account_tdx_2_129n8v0x9q2zj78293sv7xhz9zcysvcvwp70pc6826k8f7dte96sfcn").unwrap(),
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(),
                                FungibleResourceIndicator::new_guaranteed(10)
                            ),
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1n2uml563pnl0yjmd57xnj80mzdwyh4ca4w29zt2ljwpwq2p837c4a7".parse::<ResourceAddress>().unwrap(),
                                NonFungibleResourceIndicator::new_guaranteed([
                                    NonFungibleLocalId::integer(64),
                                ])
                            ),
                        ]
                    ),
                    (
                        AccountAddress::from_str("account_tdx_2_12x58hfy598wl5vukv3sqrkc7h3n699nqygufncycayeuwydel8esnu").unwrap(),
                        vec![
                            ResourceIndicator::fungible(
                                "resource_tdx_2_1t4nnqzrdympy0n74yhvkp7vnver90j5yurlhqu3083z8mt2gdvu7sq".parse::<ResourceAddress>().unwrap(),
                                FungibleResourceIndicator::new_guaranteed(5)
                            ),
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue".parse::<ResourceAddress>().unwrap(),
                                NonFungibleResourceIndicator::new_guaranteed([
                                    NonFungibleLocalId::string("Member_6").unwrap(),
                                ])
                            ),
                        ]
                    )
                ], //deposits
                [acc_v3],
                [],
                [],
                [],
                [],
                [],
                Some(
                    DetailedManifestClass::Transfer {
                        is_one_to_one_transfer: false
                    }
                ),
                FeeLocks::default(),
                FeeSummary::new(
                    "0.37765305".parse::<Decimal>().unwrap(),
                    "0.10476895".parse::<Decimal>().unwrap(),
                    "0.3614425597".parse::<Decimal>().unwrap(),
                    0,
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn third_party_deposits_update() {
        let instructions_string = fixture_rtm!("third_party_deposits_update");
        let receipt =
            deserialize_receipt(fixture_tx!("third_party_deposits_update"));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_g2: AccountAddress = "account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a".parse().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [],
                [],
                [acc_g2], // addresses_of_accounts_requiring_auth
                [],               // addresses_of_identities_requiring_auth
                [],               // newly_created_non_fungibles
                [],
                [],               // presented_proofs
                [],               // encountered_component_addresses
                Some(
                    DetailedManifestClass::AccountDepositSettingsUpdate {
                        resource_preferences_updates: HashMap::<
                            AccountAddress,
                            HashMap<ResourceAddress, ResourcePreferenceUpdate>,
                        >::from_iter([(
                            acc_g2,
                            HashMap::<_, _>::from_iter([(
                                "resource_tdx_2_1t4nnqzrdympy0n74yhvkp7vnver90j5yurlhqu3083z8mt2gdvu7sq".parse::<ResourceAddress>().unwrap(),
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
                ),
                FeeLocks::default(),
                FeeSummary::new(
                    "0.07638415".parse::<Decimal>().unwrap(),
                    "0.0105008".parse::<Decimal>().unwrap(),
                    "0.03871917658".parse::<Decimal>().unwrap(),
                    0
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn create_single_fungible() {
        let instructions_string = fixture_rtm!("create_single_fungible");
        let receipt =
            deserialize_receipt(fixture_tx!("create_single_fungible"));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_v3 = "account_tdx_2_12x55t8w9lf2qjh6z98jjxy04tkpjgjxawqm252gerhyath7qka34v3".parse::<AccountAddress>().unwrap();
        let token_address = "resource_tdx_2_1tkd0xf49tvq4wjdxn7khkt9l900750rd2jqdajukgwsyv8k9md5hh6".parse::<ResourceAddress>().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [],
                [(
                    acc_v3,
                    vec![ResourceIndicator::fungible(
                        token_address,
                        FungibleResourceIndicator::new_predicted(21000000, 1)
                    )]
                )],
                [], // addresses_of_accounts_requiring_auth
                [], // addresses_of_identities_requiring_auth
                [], // newly_created_non_fungibles
                [], // reserved_instructions
                [], // presented_proofs
                [], // encountered_component_addresses
                Some(DetailedManifestClass::General),
                FeeLocks::default(),
                FeeSummary::new(
                    "0.1585925".parse::<Decimal>().unwrap(),
                    "0.1210119".parse::<Decimal>().unwrap(),
                    "0.26540755769".parse::<Decimal>().unwrap(),
                    0,
                ),
                NewEntities::new([(
                    token_address,
                    NewlyCreatedResource::empty()
                )])
            )
        );
    }

    #[test]
    fn create_nft_collection() {
        let instructions_string = fixture_rtm!("create_nft_collection");
        let receipt = deserialize_receipt(fixture_tx!("create_nft_collection"));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc: AccountAddress = "account_tdx_2_12x55t8w9lf2qjh6z98jjxy04tkpjgjxawqm252gerhyath7qka34v3".parse().unwrap();
        let non_fungible_address: ResourceAddress = "resource_tdx_2_1nfnyenkeznzwpnf0nufa6ajsahpu00quhm8xwfrzt8u3dqm2ltzzhl".parse().unwrap();

        pretty_assertions::assert_eq!(
                sut,
                SUT::new(
                    [], // addresses_of_accounts_withdrawn_from
                    [(
                        acc,
                        vec![
                            ResourceIndicator::non_fungible(
                                non_fungible_address,
                                NonFungibleResourceIndicator::new_predicted(
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
                    Some(DetailedManifestClass::General),
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.18451315".parse::<Decimal>().unwrap(),
                        "0.40604035".parse::<Decimal>().unwrap(),
                        "0.96845625165".parse::<Decimal>().unwrap(),
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
        let instructions_string = fixture_rtm!("mint_nft_gumball_card");
        let receipt = deserialize_receipt(fixture_tx!("mint_nft_gumball_card"));

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
                                "resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(),
                                FungibleResourceIndicator::new_guaranteed(5)
                            ),
                        ]
                    )
                ],
                [
                    (
                        acc_ac,
                        vec![
                            ResourceIndicator::non_fungible(
                                "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue".parse::<ResourceAddress>().unwrap(),
                                NonFungibleResourceIndicator::new_predicted(
                                    [NonFungibleLocalId::string("Member_83").unwrap()],
                                    3
                                )
                            )
                        ]
                    )
                ],
                [
                    acc_ac,
                ], // addresses_of_accounts_requiring_auth
                [], // addresses_of_identities_requiring_auth
                ["resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue:<Member_83>".parse::<NonFungibleGlobalId>().unwrap()], // newly_created_non_fungibles
                [], // reserved_instructions
                [], // presented_proofs
                ["component_tdx_2_1cpd3cgy9kaxvxlptkkgxkm3qvfyqkrsl03kyz532p7e2gk0ygs4xrd".parse::<ManifestEncounteredComponentAddress>().unwrap()], // encountered_component_addresses
                Some(DetailedManifestClass::General),
                FeeLocks::default(),
                FeeSummary::new(
                    "0.3737913".parse::<Decimal>().unwrap(),
                    "0.0467599".parse::<Decimal>().unwrap(),
                    "0.14677047477".parse::<Decimal>().unwrap(),
                    0,
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn present_proof_swap_candy() {
        let instructions_string = fixture_rtm!("present_proof_swap_candy");
        let receipt =
            deserialize_receipt(fixture_tx!("present_proof_swap_candy"));

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
                                    "resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(),
                                    FungibleResourceIndicator::new_guaranteed(1)
                                ),
                            ]
                        )
                    ], // addresses_of_accounts_withdrawn_from
                    [
                        (
                            acc_ac,
                            vec![
                                ResourceIndicator::fungible(
                                    "resource_tdx_2_1thcmn5q5ww3fm0mx55zs9cj0n36qc0jtx956q7vh9ycxk3vh8553qc".parse::<ResourceAddress>().unwrap(),
                                    FungibleResourceIndicator::new_predicted(40, 4),
                                ),
                            ]
                        )
                    ], // addresses_of_accounts_deposited_into
                    [acc_ac], // addresses_of_accounts_requiring_auth
                    [], // addresses_of_identities_requiring_auth
                    [], // newly_created_non_fungibles
                    [], // reserved_instructions
                    [ResourceSpecifier::non_fungible("resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue".parse::<ResourceAddress>().unwrap(), vec!["<Member_83>".parse().unwrap()])], // presented_proofs
                    ["component_tdx_2_1cr4pa9ex9xhwzfjzclv8vjnfylw93wvhkwcwc0xlahpkel0krxqedw".parse::<ManifestEncounteredComponentAddress>().unwrap()], // encountered_component_addresses
                    Some(DetailedManifestClass::General),
                    FeeLocks::default(),
                    FeeSummary::new(
                        "0.4943021".parse::<Decimal>().unwrap(),
                        "0.0467589".parse::<Decimal>().unwrap(),
                        "0.13551711803".parse::<Decimal>().unwrap(),
                        0,
                    ),
                    NewEntities::default()
                )
            );
    }

    #[test]
    fn create_pool() {
        let instructions_string = fixture_rtm!("create_pool");

        let receipt = deserialize_receipt(fixture_tx!("create_pool"));

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
                Some(DetailedManifestClass::General),
                FeeLocks::default(),
                FeeSummary::new("0.15184175".parse::<Decimal>().unwrap(), "0.1607719".parse::<Decimal>().unwrap(), "0.33388137243".parse::<Decimal>().unwrap(), 0,),
                NewEntities::new([
                    (ResourceAddress::try_from_bech32("resource_tdx_2_1tkrg7uwrc9sw3tkckuxwu65euwppxa00f7aqtx5ujd0aa22gej6nm7").unwrap(), NewlyCreatedResource::default())
                ])
            )
        );
    }

    #[test]
    fn contribute_to_bi_pool() {
        let instructions_string = fixture_rtm!("contribute_to_bi_pool");

        let receipt = deserialize_receipt(fixture_tx!("contribute_to_bi_pool"));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_12xy3a3eme090gwzku277q0awutftu76nd4wddjvhtzelf84glmghae".parse().unwrap();
        let pool_address: PoolAddress = "pool_tdx_2_1c4cex49kkyxd4sscv4r9etzfr4tkm8aehwntqk4jak04zpjpelg3us".parse().unwrap();
        let star_resource_address: ResourceAddress = "resource_tdx_2_1t5gweua2ggcgw6u0g6xvf7l5t40mvwww8a856q0tzcjdnut7jdh84d".parse().unwrap();
        let pool_unit_resource_address: ResourceAddress = "resource_tdx_2_1t4u9jzqe69k3ghuhe4ulrmsthqyk2qw8wh6l4hjkt9r8m0736xx4qd".parse().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [(
                    acc_gk,
                    vec![
                        ResourceIndicator::fungible(
                            ResourceAddress::sample_stokenet_xrd(),
                            FungibleResourceIndicator::new_guaranteed(100)
                        ),
                        ResourceIndicator::fungible(
                            star_resource_address,
                            FungibleResourceIndicator::new_guaranteed(100)
                        ),
                    ]
                )],
                [(
                    acc_gk,
                    vec![ResourceIndicator::fungible(
                        pool_unit_resource_address,
                        FungibleResourceIndicator::new_predicted(100, 5)
                    ),]
                )], // addresses_of_accounts_deposited_into
                [acc_gk], // addresses_of_accounts_requiring_auth
                [],       // addresses_of_identities_requiring_auth
                [],       // newly_created_non_fungibles
                [],       // reserved_instructions
                [],       // presented_proofs
                [],       // encountered_component_addresses
                Some(DetailedManifestClass::PoolContribution {
                    pool_addresses: vec![pool_address],
                    pool_contributions: vec![TrackedPoolContribution::new(
                        pool_address,
                        [
                            (
                                star_resource_address,
                                "100".parse::<Decimal>().unwrap()
                            ),
                            (
                                ResourceAddress::sample_stokenet_xrd(),
                                "100".parse::<Decimal>().unwrap()
                            ),
                        ],
                        pool_unit_resource_address,
                        100,
                    )]
                }),
                FeeLocks::default(),
                FeeSummary::new(
                    "0.27887505".parse::<Decimal>().unwrap(),
                    "0.04776125".parse::<Decimal>().unwrap(),
                    "0.17910003354".parse::<Decimal>().unwrap(),
                    0,
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn stake_to_three_validators() {
        let instructions_string = fixture_rtm!("stake_to_three_validators");
        let receipt =
            deserialize_receipt(fixture_tx!("stake_to_three_validators"));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a".parse().unwrap();

        let validator_0: ValidatorAddress = "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".parse().unwrap();
        let validator_0_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t45l9ku3r5mwxazht2qutmhhk3660hqqvxkkyl8rxs20n9k2zv0w7t".parse::<_>().unwrap();

        let validator_1: ValidatorAddress = "validator_tdx_2_1sdlkptcwjpajqawnuya8r2mgl3eqt89hw27ww6du8kxmx3thmyu8l4".parse().unwrap();
        let validator_1_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t5hpjckz9tm63gqvxsl60ejhzvnlguly77tltvywnj06s2x9wjdxjn".parse::<_>().unwrap();

        let validator_2: ValidatorAddress = "validator_tdx_2_1svr6rmtd9ts5zx8d3euwmmp6mmjdtcj2q7zlmd8xjrn4qx7q5snkas".parse().unwrap();
        let validator_2_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t48zl3qmcv3pf24r0765q4zc6rrk83cfjv6wza2xksej80pcfd7p5g".parse::<_>().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [(
                    acc_gk,
                    vec![ResourceIndicator::fungible(
                        ResourceAddress::sample_stokenet_xrd(),
                        FungibleResourceIndicator::new_guaranteed(3000)
                    )]
                )], // addresses_of_accounts_withdrawn_from
                [(
                    acc_gk,
                    vec![
                        ResourceIndicator::fungible(
                            validator_0_resource_address_of_stake,
                            FungibleResourceIndicator::new_predicted(1000, 3)
                        ),
                        ResourceIndicator::fungible(
                            validator_1_resource_address_of_stake,
                            FungibleResourceIndicator::new_predicted(1000, 7)
                        ),
                        ResourceIndicator::fungible(
                            validator_2_resource_address_of_stake,
                            FungibleResourceIndicator::new_predicted(1000, 11)
                        ),
                    ]
                )], // addresses_of_accounts_deposited_into
                [acc_gk], // addresses_of_accounts_requiring_auth
                [],       // addresses_of_identities_requiring_auth
                [],       // newly_created_non_fungibles
                [],       // reserved_instructions
                [],       // presented_proofs
                [],       // encountered_component_addresses
                Some(DetailedManifestClass::ValidatorStake {
                    validator_addresses: vec![
                        validator_0,
                        validator_1,
                        validator_2
                    ],
                    validator_stakes: vec![
                        TrackedValidatorStake::new(
                            validator_0,
                            1000,
                            validator_0_resource_address_of_stake,
                            1000,
                        ),
                        TrackedValidatorStake::new(
                            validator_1,
                            1000,
                            validator_1_resource_address_of_stake,
                            1000,
                        ),
                        TrackedValidatorStake::new(
                            validator_2,
                            1000,
                            validator_2_resource_address_of_stake,
                            1000,
                        ),
                    ]
                }),
                FeeLocks::default(),
                FeeSummary::new(
                    "0.3527215".parse::<Decimal>().unwrap(),
                    "0.1150347".parse::<Decimal>().unwrap(),
                    "0.32796859177".parse::<Decimal>().unwrap(),
                    0,
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn redeem_from_bi_pool() {
        let instructions_string = fixture_rtm!("redeem_from_bi_pool");
        let receipt = deserialize_receipt(fixture_tx!("redeem_from_bi_pool"));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk = AccountAddress::from_str("account_tdx_2_12x55t8w9lf2qjh6z98jjxy04tkpjgjxawqm252gerhyath7qka34v3").unwrap();
        let resource_address_of_pool = ResourceAddress::from_str("resource_tdx_2_1thgnc84xkcjhs46pfvm9s8zn8t9kxwryvyr9x3947xpt6jxty7qn25").unwrap();
        let pool_address = PoolAddress::from_str("pool_tdx_2_1c5mygu9t8rlfq6j8v2ynrg60ltts2dctsghust8u2tuztrml427830").unwrap();
        let pool_resource_xrd_address = ResourceAddress::sample_stokenet_xrd();
        let pool_resource_vip_address = ResourceAddress::from_str("resource_tdx_2_1tkx5re7dyhr8updty5spde2nsr0kpnhq953m09xg3u6hf3u6j4fsqf").unwrap();

        pretty_assertions::assert_eq!(
            sut.withdrawals,
            HashMap::from([(
                acc_gk,
                vec![ResourceIndicator::fungible(
                    resource_address_of_pool,
                    FungibleResourceIndicator::new_guaranteed(1)
                )]
            )])
        );

        let deposits = sut
            .deposits
            .get(&acc_gk)
            .unwrap()
            .iter()
            .cloned()
            .collect::<IndexSet<_>>();

        pretty_assertions::assert_eq!(
            deposits,
            IndexSet::from([
                ResourceIndicator::fungible(
                    pool_resource_vip_address,
                    FungibleResourceIndicator::new_predicted(
                        "0.316227766016837933".parse::<Decimal>().unwrap(),
                        3
                    )
                ),
                ResourceIndicator::fungible(
                    pool_resource_xrd_address,
                    FungibleResourceIndicator::new_predicted(3.162277, 3)
                ),
            ])
        );

        pretty_assertions::assert_eq!(
            sut.addresses_of_accounts_requiring_auth,
            vec![acc_gk]
        );

        pretty_assertions::assert_eq!(
            sut.addresses_of_identities_requiring_auth,
            vec![]
        );

        pretty_assertions::assert_eq!(sut.newly_created_non_fungibles, vec![]);

        pretty_assertions::assert_eq!(
            sut.reserved_instructions,
            IndexSet::new()
        );

        pretty_assertions::assert_eq!(sut.presented_proofs, vec![]);

        pretty_assertions::assert_eq!(sut.encountered_addresses, vec![]);

        pretty_assertions::assert_eq!(
            sut.detailed_classification,
            Some(
                DetailedManifestClass::PoolRedemption {
                    pool_addresses: vec![pool_address],
                    pool_redemptions: vec![
                        TrackedPoolRedemption::new(
                            pool_address,
                            ResourceAddress::from_str("resource_tdx_2_1thgnc84xkcjhs46pfvm9s8zn8t9kxwryvyr9x3947xpt6jxty7qn25").unwrap(),
                            1,
                            [
                                (
                                    pool_resource_vip_address,
                                    "0.316227766016837933".parse::<Decimal>().unwrap()
                                ),
                                (
                                    pool_resource_xrd_address,
                                    "3.162277".parse::<Decimal>().unwrap()
                                ),
                            ]
                        )
                    ]
                }
            )
        );

        pretty_assertions::assert_eq!(sut.fee_locks, FeeLocks::default());

        pretty_assertions::assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.26154965".parse::<Decimal>().unwrap(),
                "0.0325088".parse::<Decimal>().unwrap(),
                "0.12760162134".parse::<Decimal>().unwrap(),
                0
            )
        );

        pretty_assertions::assert_eq!(sut.new_entities, NewEntities::default());
    }

    #[test]
    fn unstake_partially_from_one_validator() {
        let instructions_string =
            fixture_rtm!("unstake_partially_from_one_validator");
        let receipt = deserialize_receipt(fixture_tx!(
            "unstake_partially_from_one_validator"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a".parse().unwrap();

        let nf_global_id: NonFungibleGlobalId = "resource_tdx_2_1ngw8z6ut9mw54am4rr65kwcuz24q3n7waxtzyfvug5g4yuc00jydqj:{8a190d8fd0725713-e9072f0fd954196f-5f9be7adaf8d5b78-cf811ea9992983c3}".parse().unwrap();
        let validator_address: ValidatorAddress = "validator_tdx_2_1sdlkptcwjpajqawnuya8r2mgl3eqt89hw27ww6du8kxmx3thmyu8l4".parse().unwrap();
        let validator_resource_address: ResourceAddress = "resource_tdx_2_1t5hpjckz9tm63gqvxsl60ejhzvnlguly77tltvywnj06s2x9wjdxjn".parse().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [(
                    acc_gk,
                    vec![ResourceIndicator::fungible(
                        validator_resource_address,
                        FungibleResourceIndicator::new_guaranteed(500)
                    ),]
                )], // addresses_of_accounts_withdrawn_from
                [(
                    acc_gk,
                    vec![ResourceIndicator::non_fungible(
                        nf_global_id.resource_address,
                        NonFungibleResourceIndicator::new_predicted(
                            [nf_global_id.non_fungible_local_id.clone()],
                            3
                        )
                    )]
                )], // addresses_of_accounts_deposited_into
                [acc_gk],
                [], // addresses_of_identities_requiring_auth
                [nf_global_id.clone()], // newly_created_non_fungibles
                [], // reserved_instructions
                [], // presented_proofs
                [], // encountered_component_addresses
                Some(DetailedManifestClass::ValidatorUnstake {
                    validator_addresses: vec![validator_address],
                    claims_non_fungible_data: HashMap::from([(
                        nf_global_id,
                        UnstakeData::new("Stake Claim", 37923, 500)
                    )]),
                }),
                FeeLocks::default(),
                FeeSummary::new(
                    "0.2848875".parse::<Decimal>().unwrap(),
                    "0.06251535".parse::<Decimal>().unwrap(),
                    "0.16927718825".parse::<Decimal>().unwrap(),
                    0,
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn claim_two_stakes_from_one_validator() {
        let instructions_string =
            fixture_rtm!("claim_two_stakes_from_one_validator");
        let receipt = deserialize_receipt(fixture_tx!(
            "claim_two_stakes_from_one_validator"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc_gk: AccountAddress = "account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a".parse().unwrap();
        let validator_address: ValidatorAddress = "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".parse().unwrap();
        let claim_nft_resource_address: ResourceAddress =  "resource_tdx_2_1ng3g2nj5pfpmdphgz0nrh8z0gtqcxx5z5dn48t85ar0z0zjhefufaw".parse::<ResourceAddress>().unwrap();
        let nf_id_1 = NonFungibleLocalId::ruid(hex_decode("1c1ce92c810094a765659db6a666c19c6cea4367bb789b55276b137712ceecce").unwrap()).unwrap();
        let nf_id_2 = NonFungibleLocalId::ruid(hex_decode("5aebd0270caf3f8751031498741f57b5d24fe0d62a976589519c6a92423888cc").unwrap()).unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [(
                    acc_gk,
                    vec![ResourceIndicator::non_fungible(
                        claim_nft_resource_address,
                        NonFungibleResourceIndicator::new_guaranteed([
                            nf_id_1.clone(),
                            nf_id_2.clone(),
                        ])
                    ),]
                )], // addresses_of_accounts_withdrawn_from
                [(
                    acc_gk,
                    vec![ResourceIndicator::fungible(
                        ResourceAddress::sample_stokenet_xrd(),
                        FungibleResourceIndicator::new_guaranteed(150)
                    ),]
                ),], // addresses_of_accounts_deposited_into
                [acc_gk], // addresses_of_accounts_requiring_auth
                [],       // addresses_of_identities_requiring_auth
                [],       // newly_created_non_fungibles
                [],       // reserved_instructions
                [],       // presented_proofs
                [],       // encountered_component_addresses
                Some(DetailedManifestClass::ValidatorClaim {
                    validator_addresses: vec![validator_address],
                    validator_claims: vec![TrackedValidatorClaim::new(
                        validator_address,
                        claim_nft_resource_address,
                        [nf_id_1, nf_id_2],
                        150
                    )]
                }),
                FeeLocks::default(),
                FeeSummary::new(
                    "0.2383276".parse::<Decimal>().unwrap(),
                    "0.041757".parse::<Decimal>().unwrap(),
                    "0.11224746511".parse::<Decimal>().unwrap(),
                    0,
                ),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn account_locker_claim_fungibles_and_non_fungibles() {
        let instructions_string =
            fixture_rtm!("account_locker_claim_fungibles_and_non_fungibles");

        let receipt = deserialize_receipt(fixture_tx!(
            "account_locker_claim_fungibles_and_non_fungibles"
        ));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc: AccountAddress = "account_tdx_2_12x2lmewv5lfen4x96aurw7a5z5ukdzyyc0fkytamqgml77lah44kkp".parse().unwrap();
        let fungible_address: ResourceAddress = "resource_tdx_2_1th75jg2gx9l3v0r8duzrmknfarhd3ha0387lg9n78qc9849xsfyq32".parse().unwrap();
        let non_fungible_address: ResourceAddress = "resource_tdx_2_1n2z4k99wuqlph9lj64ckc64znm48axl37xctsa0xqmm2sqg7klrte3".parse::<ResourceAddress>().unwrap();

        pretty_assertions::assert_eq!(
            sut,
            SUT::new(
                [],
                [(
                    acc,
                    vec![
                        ResourceIndicator::non_fungible(
                            non_fungible_address,
                            NonFungibleResourceIndicator::new_predicted(
                                [NonFungibleLocalId::integer(8)],
                                1
                            )
                        ),
                        ResourceIndicator::fungible(
                            fungible_address,
                            FungibleResourceIndicator::new_guaranteed(3)
                        ),
                    ]
                )],
                vec!["account_tdx_2_12x2lmewv5lfen4x96aurw7a5z5ukdzyyc0fkytamqgml77lah44kkp".parse::<AccountAddress>().unwrap()], // addresses_of_accounts_requiring_auth
                [], // addresses_of_identities_requiring_auth
                [], // newly_created_non_fungibles
                [], // reserved_instructions
                [], // presented_proofs
                ["locker_tdx_2_1dr6v4fwufgacxqwxsm44ysglhdv7yyxgvq6xazcwzvu35937wzsjnx".parse::<ManifestEncounteredComponentAddress>().unwrap()],
                Some(DetailedManifestClass::General),
                FeeLocks::default(),
                FeeSummary::new("0.2516311".parse::<Decimal>().unwrap(), "0.03200635".parse::<Decimal>().unwrap(), "0.12903213279".parse::<Decimal>().unwrap(), 0,),
                NewEntities::default()
            )
        );
    }

    #[test]
    fn account_delete() {
        let instructions_string = fixture_rtm!("account_delete");

        let receipt = deserialize_receipt(fixture_tx!("account_delete"));

        let transaction_manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Stokenet,
            Blobs::default(),
        )
        .unwrap();

        let sut = transaction_manifest.execution_summary(receipt).unwrap();

        let acc = "account_tdx_2_12xy65ekdcrehj24t0ks5lvvqcvr48qgac4efq3phecp0xyetze5nyy".parse::<AccountAddress>().unwrap();
        let badge_address = "resource_tdx_2_1nfxxxxxxxxxxaccwnrxxxxxxxxx006664022062xxxxxxxxx4vczzk".parse::<ResourceAddress>().unwrap();
        let badge_id = NonFungibleLocalId::bytes(
            NonEmptyMax64Bytes::from_hex(
                "5189aa66cdc0f3792aab7da14fb180c30753811dc572904437ce02f3132b",
            )
            .unwrap(),
        )
        .unwrap();
        let global_badge_address = NonFungibleGlobalId::new(
            NonFungibleResourceAddress::new(badge_address).unwrap(),
            badge_id.clone(),
        );

        let expected_summary = SUT::new(
            [], // No withdrawals
            [(
                acc,
                vec![ResourceIndicator::non_fungible(
                    badge_address,
                    NonFungibleResourceIndicator::new_guaranteed([badge_id]),
                )],
            )],
            vec![acc], // addresses_of_accounts_requiring_auth
            [],        // addresses_of_identities_requiring_auth
            [global_badge_address], // newly_created_non_fungibles
            [ReservedInstruction::AccountSecurify], // reserved_instructions
            [],        // presented_proofs
            [],
            Some(DetailedManifestClass::DeleteAccounts {
                account_addresses: vec![acc],
            }),
            FeeLocks::default(),
            FeeSummary::new(
                "0.21017315".parse::<Decimal>().unwrap(),
                "0.04175875".parse::<Decimal>().unwrap(),
                "0.1564025852".parse::<Decimal>().unwrap(),
                0,
            ),
            NewEntities::default(),
        );

        pretty_assertions::assert_eq!(sut, expected_summary)
    }

    fn deserialize_receipt(
        value: impl AsRef<str>,
    ) -> ScryptoSerializableToolkitTransactionReceipt {
        serde_json::from_str::<ScryptoSerializableToolkitTransactionReceipt>(
            value.as_ref(),
        )
        .unwrap()
    }
}
