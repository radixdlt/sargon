use crate::prelude::*;

use radix_engine_toolkit::functions::manifest::execution_summary as RET_execution_summary;

impl TransactionManifest {
    pub fn execution_summary(
        &self,
        encoded_receipt: BagOfBytes,
    ) -> Result<ExecutionSummary> {
        let receipt: TransactionReceipt = encoded_receipt.try_into()?;
        let ret_execution_summary =
            RET_execution_summary(&self.scrypto_manifest(), &receipt.decoded)
                .map_err(|e| {
                error!(
                    "Failed to get execution summary from RET, error: {:?}",
                    e
                );
                CommonError::FailedToGetRetExecutionSummaryFromManifest
            })?;

        Ok(ExecutionSummary::from((
            ret_execution_summary,
            self.network_id(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_receipt() {
        assert_eq!(
            TransactionManifest::sample()
                .execution_summary(BagOfBytes::from_hex("dead").unwrap()),
            Err(CommonError::FailedToDecodeEncodedReceipt)
        );
    }

    #[test]
    fn transfer_1to2_multiple_nf_and_f_tokens() {
        let encoded_receipt_hex =
            include_str!("transfer_1to2_multiple_nf_and_f_tokens.dat");
        let instructions_string =
            include_str!("transfer_1to2_multiple_nf_and_f_tokens.rtm");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.4077857".parse::<Decimal>().unwrap(),
                "0.1307814".parse::<Decimal>().unwrap(),
                "0.52433013014".parse::<Decimal>().unwrap(),
                0,
            )
        );
        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.new_entities, NewEntities::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, vec![acc_gk]);
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());

        assert_eq!(
            sut.detailed_classification,
            vec![
                DetailedManifestClass::Transfer {
                    is_one_to_one: false
                },
                DetailedManifestClass::General
            ]
        );
    }

    #[test]
    fn third_party_deposits_update() {
        let encoded_receipt_hex =
            include_str!("third_party_deposits_update.dat");
        let instructions_string =
            include_str!("third_party_deposits_update.rtm");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.092499".parse::<Decimal>().unwrap(),
                "0.02100205".parse::<Decimal>().unwrap(),
                "0.08459091041".parse::<Decimal>().unwrap(),
                0
            )
        );
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.new_entities, NewEntities::default());
        assert_eq!(sut.presented_proofs, Vec::default());

        let acc_g2: AccountAddress = "account_tdx_2_12xx9jz27aa0mqjj8cwhk7pzkhtkthv09yclmurse42hlyme2gegyg2".parse().unwrap();

        assert_eq!(
            sut.addresses_of_accounts_requiring_auth,
            vec![acc_g2.clone()]
        );
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());
        assert_eq!(sut.detailed_classification.len(), 1);

        let (
            resource_preferences_updates,
            deposit_mode_updates,
            authorized_depositors_added,
            authorized_depositors_removed,
        ) = sut.detailed_classification[0]
            .clone()
            .into_account_deposit_settings_update()
            .unwrap();

        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(
            resource_preferences_updates,
            HashMap::<
                AccountAddress,
                HashMap<ResourceAddress, ResourcePreferenceUpdate>,
            >::from_iter([(
                acc_g2.clone(),
                HashMap::<_, _>::from_iter([(
                    ResourceAddress::sample_stokenet_gc_tokens(),
                    ResourcePreferenceUpdate::Remove
                )])
            )])
        );

        assert_eq!(
            deposit_mode_updates,
            HashMap::<AccountAddress, DepositRule>::from_iter([(
                acc_g2.clone(),
                DepositRule::DenyAll
            )])
        );

        assert_eq!(
            authorized_depositors_added,
            HashMap::<AccountAddress, Vec<ResourceOrNonFungible>>::from_iter([
                (
                    acc_g2.clone(),
                    vec![ResourceOrNonFungible::Resource {
                        value:
                            ResourceAddress::sample_stokenet_nft_gc_membership()
                    }]
                )
            ])
        );

        assert_eq!(
            authorized_depositors_removed,
            HashMap::<AccountAddress, Vec<ResourceOrNonFungible>>::from_iter([
                (
                    acc_g2.clone(),
                    vec![ResourceOrNonFungible::Resource {
                        value: ResourceAddress::sample_stokenet_nft_other()
                    }]
                )
            ])
        );
    }

    #[test]
    fn create_single_fungible() {
        let encoded_receipt_hex = include_str!("create_single_fungible.dat");
        let instructions_string = include_str!("create_single_fungible.rtm");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.15800815".parse::<Decimal>().unwrap(),
                "0.1160115".parse::<Decimal>().unwrap(),
                "0.25339126151".parse::<Decimal>().unwrap(),
                0,
            )
        );

        let resource: ResourceAddress = "resource_tdx_2_1t4wty7nq976ej4wtx7p4ckm073p32cyaajk4cq256rcvzz20e7qrm9".parse().unwrap();
        assert_eq!(
            sut.new_entities.metadata,
            HashMap::<ResourceAddress, NewlyCreatedResource>::from_iter([(
                resource,
                NewlyCreatedResource::with(
                    "MyResource",
                    "VIP",
                    "A very innovative and important resource",
                    "https://i.imgur.com/A2itmif.jpeg",
                    []
                )
            )])
        );
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, Vec::default());
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());
        assert_eq!(sut.detailed_classification.len(), 1);

        assert_eq!(
            sut.detailed_classification,
            vec![DetailedManifestClass::General]
        )
    }

    #[test]
    fn create_nft_collections() {
        let encoded_receipt_hex = include_str!("create_nft_collections.dat");
        let instructions_string = include_str!("create_nft_collections.rtm");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.21852455".parse::<Decimal>().unwrap(),
                "0.3320334".parse::<Decimal>().unwrap(),
                "0.73328016927".parse::<Decimal>().unwrap(),
                0,
            )
        );

        let resource_abandon: ResourceAddress = "resource_tdx_2_1ntwlumm8g8hsx0emmxgj3akcx6aajspx06llvfmq733x2ssm4v3g0e".parse().unwrap();
        let resource_ability: ResourceAddress = "resource_tdx_2_1nfe6ugwjvuqc7aqhcltnwmp8xfkpqnm9mc8n9jh5xcvnx38r2wncre".parse().unwrap();

        pretty_assertions::assert_eq!(
            sut.new_entities.metadata.clone().get(&resource_abandon).unwrap(),
            &NewlyCreatedResource::with(
                "Abandon",
                "ABANDON",
                "Abandon: An amazingly innovative and rare NFT collection",
                "https://image-service-test-images.s3.eu-west-2.amazonaws.com/wallet_test_images/KLHaze-medium.jpg",
                ["Unique".to_string(), "FOMO".to_string(), "Advanced".to_string()]
            )
        );

        pretty_assertions::assert_eq!(
            sut.new_entities.metadata.clone().get(&resource_ability).unwrap(),
            &NewlyCreatedResource::with(
                "Ability",
                "ABILITY",
                "Ability: An amazingly innovative and rare NFT collection",
                "https://image-service-test-images.s3.eu-west-2.amazonaws.com/wallet_test_images/KLHaze-medium.jpg",
                ["Unique".to_string(), "FOMO".to_string(), "Advanced".to_string()]
            )
        );

        pretty_assertions::assert_eq!(sut.newly_created_non_fungibles, [
            "resource_tdx_2_1nfe6ugwjvuqc7aqhcltnwmp8xfkpqnm9mc8n9jh5xcvnx38r2wncre:#0#", "resource_tdx_2_1nfe6ugwjvuqc7aqhcltnwmp8xfkpqnm9mc8n9jh5xcvnx38r2wncre:#1#", "resource_tdx_2_1ntwlumm8g8hsx0emmxgj3akcx6aajspx06llvfmq733x2ssm4v3g0e:#0#", "resource_tdx_2_1ntwlumm8g8hsx0emmxgj3akcx6aajspx06llvfmq733x2ssm4v3g0e:#1#"].into_iter().map(NonFungibleGlobalId::from_str).map(Result::unwrap).collect_vec());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, Vec::default());
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());
        assert_eq!(sut.detailed_classification.len(), 1);

        assert_eq!(
            sut.detailed_classification,
            vec![DetailedManifestClass::General]
        )
    }

    #[test]
    fn mint_nft_gumball_card() {
        let instructions_string = include_str!("mint_nft_gumball_card.rtm");
        let encoded_receipt_hex = include_str!("mint_nft_gumball_card.dat");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.3751137".parse::<Decimal>().unwrap(),
                "0.0467599".parse::<Decimal>().unwrap(),
                "0.14677047477".parse::<Decimal>().unwrap(),
                0,
            )
        );

        assert_eq!(sut.new_entities, NewEntities::default());

        assert_eq!(sut.newly_created_non_fungibles, [
    "resource_tdx_2_1ng88qk08hrgmad30rzdxpyx779yuta4cwcjc3gstk60jhachsv94g9:<Member_44>"
            ].into_iter().map(NonFungibleGlobalId::from_str).map(Result::unwrap).collect_vec());
        assert_eq!(sut.presented_proofs, Vec::default());

        let accounts_req_auth: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        assert_eq!(
            sut.encountered_component_addresses,
            vec![
                "component_tdx_2_1czg6rq9vms7t402fedtpzkjah25hh7snyu3ysgxk3pwlz4d3tugm7j".parse::<ComponentAddress>().unwrap()
            ]
        );
        assert_eq!(
            sut.addresses_of_accounts_requiring_auth,
            vec![accounts_req_auth]
        );
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());
        assert_eq!(sut.detailed_classification.len(), 1);

        assert_eq!(
            sut.detailed_classification,
            vec![DetailedManifestClass::General]
        )
    }

    #[test]
    fn present_proof_swap_candy() {
        let instructions_string = include_str!("present_proof_swap_candy.rtm");
        let encoded_receipt_hex = include_str!("present_proof_swap_candy.dat");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));

        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.50142635".parse::<Decimal>().unwrap(),
                "0.0467589".parse::<Decimal>().unwrap(),
                "0.13551711803".parse::<Decimal>().unwrap(),
                0,
            )
        );

        assert_eq!(sut.new_entities, NewEntities::default());

        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.presented_proofs, vec![
            "resource_tdx_2_1ng88qk08hrgmad30rzdxpyx779yuta4cwcjc3gstk60jhachsv94g9".parse::<ResourceAddress>().unwrap()
        ]);

        let accounts_req_auth: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        assert_eq!(
            sut.encountered_component_addresses,
            vec![
                "component_tdx_2_1crje3en7zsrna9t5vyywn3z3t9ht34l9udxjcpjvdhpcw9v6vlzru8".parse::<ComponentAddress>().unwrap()
            ]
        );
        assert_eq!(
            sut.addresses_of_accounts_requiring_auth,
            vec![accounts_req_auth]
        );
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());
        assert_eq!(sut.detailed_classification.len(), 1);

        assert_eq!(
            sut.detailed_classification,
            vec![DetailedManifestClass::General]
        )
    }

    #[test]
    fn create_pool() {
        let instructions_string = include_str!("create_pool.rtm");
        let encoded_receipt_hex = include_str!("create_pool.dat");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.1495719".parse::<Decimal>().unwrap(),
                "0.1557717".parse::<Decimal>().unwrap(),
                "0.3290176335".parse::<Decimal>().unwrap(),
                0,
            )
        );
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.new_entities, NewEntities::new([
            (
                "resource_tdx_2_1t57kl0kuvneehavfv9u0szngxwmek9qq4ecqv0yg59w83axqdak5xc".parse::<ResourceAddress>().unwrap(),
                 NewlyCreatedResource::empty()
            )
        ]));
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, Vec::default());
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());
        assert_eq!(
            sut.detailed_classification,
            vec![DetailedManifestClass::General]
        )
    }

    #[test]
    fn contribute_to_bi_pool() {
        let instructions_string = include_str!("contribute_to_bi_pool.rtm");
        let encoded_receipt_hex = include_str!("contribute_to_bi_pool.dat");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.27435815".parse::<Decimal>().unwrap(),
                "0.04276125".parse::<Decimal>().unwrap(),
                "0.17910003354".parse::<Decimal>().unwrap(),
                0,
            )
        );
        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.new_entities, NewEntities::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, vec![acc_gk]);
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());

        assert_eq!(sut.detailed_classification.len(), 1);

        let (pool_addresses, pool_contributions) = sut.detailed_classification
            [0]
        .clone()
        .into_pool_contribution()
        .unwrap();

        let resource_address_of_pool: ResourceAddress = "resource_tdx_2_1thnhmen4wg29tnqrfpk9w2v90s64z8at9sethnjma76866rfvcc2gs".parse().unwrap();
        let pool_address: PoolAddress = "pool_tdx_2_1ckfjmjswvvf6y635f8l89uunu9cwgnglhqdk8627wrpf8ultdx2vc3".parse().unwrap();
        let token0: ResourceAddress = "resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy".parse().unwrap();
        let token1 = ResourceAddress::sample_stokenet_xrd();
        assert_eq!(pool_addresses, vec![pool_address.clone()]);

        assert_eq!(
            pool_contributions,
            vec![TrackedPoolContribution::new(
                pool_address,
                [
                    (token0.clone(), Decimal::from(1337)),
                    (token1.clone(), Decimal::from(237)),
                ],
                resource_address_of_pool,
                Decimal::from_str("562.91118304755680169").unwrap()
            )]
        );
    }

    #[test]
    fn stake_to_three_validators() {
        let instructions_string = include_str!("stake_to_three_validators.rtm");
        let encoded_receipt_hex = include_str!("stake_to_three_validators.dat");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.34071685".parse::<Decimal>().unwrap(),
                "0.1150347".parse::<Decimal>().unwrap(),
                "0.32796859177".parse::<Decimal>().unwrap(),
                0,
            )
        );
        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.new_entities, NewEntities::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(
            sut.addresses_of_accounts_requiring_auth,
            vec![acc_gk.clone()]
        );
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());

        assert_eq!(sut.detailed_classification.len(), 1);

        let validator_0: ValidatorAddress = "validator_tdx_2_1sdatqsl6rx05yy2yvpf6ckfl7x8dluvzkcyljkn0x4lxkgucc0xz2w".parse().unwrap();
        let validator_0_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1th6hufew82dpntmcn7kt9f7au50cr59996tawh4syph0kz5e99v2u6".parse().unwrap();

        let validator_1: ValidatorAddress = "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".parse().unwrap();
        let validator_1_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t45l9ku3r5mwxazht2qutmhhk3660hqqvxkkyl8rxs20n9k2zv0w7t".parse().unwrap();

        let validator_2: ValidatorAddress = "validator_tdx_2_1sdlkptcwjpajqawnuya8r2mgl3eqt89hw27ww6du8kxmx3thmyu8l4".parse().unwrap();
        let validator_2_resource_address_of_stake: ResourceAddress = "resource_tdx_2_1t5hpjckz9tm63gqvxsl60ejhzvnlguly77tltvywnj06s2x9wjdxjn".parse().unwrap();

        let (validator_addresses, validator_stakes) = sut
            .detailed_classification[0]
            .clone()
            .into_validator_stake()
            .unwrap();

        assert_eq!(
            validator_addresses,
            vec![
                validator_0.clone(),
                validator_1.clone(),
                validator_2.clone()
            ]
        );

        assert_eq!(
            validator_stakes,
            vec![
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
        );
    }

    #[test]
    fn redeem_from_bi_pool() {
        let instructions_string = include_str!("redeem_from_bi_pool.rtm");
        let encoded_receipt_hex = include_str!("redeem_from_bi_pool.dat");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.25753315".parse::<Decimal>().unwrap(),
                "0.0325088".parse::<Decimal>().unwrap(),
                "0.12760162134".parse::<Decimal>().unwrap(),
                0,
            )
        );
        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.new_entities, NewEntities::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, vec![acc_gk]);
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());

        assert_eq!(sut.detailed_classification.len(), 1);

        let (pool_addresses, pool_contributions) = sut.detailed_classification
            [0]
        .clone()
        .into_pool_redemption()
        .unwrap();

        let resource_address_of_pool: ResourceAddress = "resource_tdx_2_1thnhmen4wg29tnqrfpk9w2v90s64z8at9sethnjma76866rfvcc2gs".parse().unwrap();

        let pool_address: PoolAddress = "pool_tdx_2_1ckfjmjswvvf6y635f8l89uunu9cwgnglhqdk8627wrpf8ultdx2vc3".parse().unwrap();

        let token0: ResourceAddress = "resource_tdx_2_1thw7yclz24h5xjp3086cj8z2ya0d7p9mydk0yh68c28ha02uhzrnyy".parse().unwrap();

        let token1 = ResourceAddress::sample_stokenet_xrd();

        assert_eq!(pool_addresses, vec![pool_address.clone()]);

        assert_eq!(
            pool_contributions,
            vec![TrackedPoolRedemption::new(
                pool_address,
                resource_address_of_pool,
                500,
                [
                    (token0.clone(), "1187.5763355433".parse().unwrap()),
                    (token1.clone(), "210.512783488241137505".parse().unwrap()),
                ]
            )]
        );
    }

    #[test]
    fn unstake_partially_from_one_validator() {
        let instructions_string =
            include_str!("unstake_partially_from_one_validator.rtm");
        let encoded_receipt_hex =
            include_str!("unstake_partially_from_one_validator.dat");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.2788849".parse::<Decimal>().unwrap(),
                "0.06251535".parse::<Decimal>().unwrap(),
                "0.16927718825".parse::<Decimal>().unwrap(),
                0,
            )
        );
        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();

        let nf_global_id: NonFungibleGlobalId = "resource_tdx_2_1ngw8z6ut9mw54am4rr65kwcuz24q3n7waxtzyfvug5g4yuc00jydqj:{192ed08c15075e36-ec4892a8ba3b86f1-a1e050a6563b787e-adc9813f7fc90480}".parse().unwrap();
        assert_eq!(sut.new_entities, NewEntities::default());
        assert_eq!(sut.newly_created_non_fungibles, vec![nf_global_id.clone()]);
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, vec![acc_gk]);
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());

        assert_eq!(sut.detailed_classification.len(), 1);

        let (validator_addresses, claims_non_fungible_data) = sut
            .detailed_classification[0]
            .clone()
            .into_validator_unstake()
            .unwrap();

        let validator: ValidatorAddress = "validator_tdx_2_1sdlkptcwjpajqawnuya8r2mgl3eqt89hw27ww6du8kxmx3thmyu8l4".parse().unwrap();

        assert_eq!(validator_addresses, vec![validator]);
        assert_eq!(
            claims_non_fungible_data,
            HashMap::<NonFungibleGlobalId, UnstakeData>::from_iter([(
                nf_global_id,
                UnstakeData::new("Stake Claim", 42215, 1234)
            )])
        );
    }

    #[test]
    fn claim_two_validator_stakes() {
        let instructions_string =
            include_str!("claim_two_validator_stakes.rtm");
        let encoded_receipt_hex =
            include_str!("claim_two_validator_stakes.dat");

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

        assert_eq!(sut.fee_locks, FeeLocks::new(0, 0));
        assert_eq!(
            sut.fee_summary,
            FeeSummary::new(
                "0.30518895".parse::<Decimal>().unwrap(),
                "0.05851055".parse::<Decimal>().unwrap(),
                "0.1916885343".parse::<Decimal>().unwrap(),
                0,
            )
        );
        let acc_gk: AccountAddress = "account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk".parse().unwrap();
        assert_eq!(sut.newly_created_non_fungibles, Vec::default());
        assert_eq!(sut.new_entities, NewEntities::default());
        assert_eq!(sut.presented_proofs, Vec::default());
        assert_eq!(sut.encountered_component_addresses, Vec::default());
        assert_eq!(sut.addresses_of_accounts_requiring_auth, vec![acc_gk]);
        assert_eq!(sut.addresses_of_identities_requiring_auth, Vec::default());

        assert_eq!(sut.detailed_classification.len(), 1);

        let (validator_addresses, validator_claims) = sut
            .detailed_classification[0]
            .clone()
            .into_validator_claim()
            .unwrap();

        let validator_1: ValidatorAddress = "validator_tdx_2_1sdtnujyn3720ymg8lakydkvc5tw4q3zecdj95akdwt9de362mvtd94".parse().unwrap();
        let validator_1_resource_address_of_stake: NonFungibleResourceAddress = "resource_tdx_2_1ng3g2nj5pfpmdphgz0nrh8z0gtqcxx5z5dn48t85ar0z0zjhefufaw".parse().unwrap();

        let validator_2: ValidatorAddress = "validator_tdx_2_1sdlkptcwjpajqawnuya8r2mgl3eqt89hw27ww6du8kxmx3thmyu8l4".parse().unwrap();
        let validator_2_resource_address_of_stake: NonFungibleResourceAddress = "resource_tdx_2_1ngw8z6ut9mw54am4rr65kwcuz24q3n7waxtzyfvug5g4yuc00jydqj".parse().unwrap();

        assert_eq!(
            validator_addresses,
            vec![validator_1.clone(), validator_2.clone()]
        );
        assert_eq!(
            validator_claims, vec![
                TrackedValidatorClaim::new(
                    validator_1,
                    validator_1_resource_address_of_stake,
                    [
                        NonFungibleLocalId::from_str("{97c2b05d8529be58-152d79c176d61d68-f87611f279e0daa3-d486426d5330795c}").unwrap()
                    ],
                    110
                ),
                TrackedValidatorClaim::new(
                    validator_2,
                    validator_2_resource_address_of_stake,
                    [
                        NonFungibleLocalId::from_str("{f1edc2f0f8f54d33-dab8e1bf90e196ce-9714ef7b85478c6c-82486b47a79b3002}").unwrap()
                    ],
                    1234
                ),
            ]
        );
    }
}
