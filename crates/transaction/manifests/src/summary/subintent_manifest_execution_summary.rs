use crate::prelude::*;

impl StaticallyAnalyzableManifest for SubintentManifest {
    fn network_id(&self) -> NetworkID {
        self.network_id()
    }

    fn summary(&self) -> Result<ManifestSummary> {
        let summary =
            RET_statically_analyze_subintent_manifest(&self.scrypto_manifest())
                .map_err(map_static_analysis_error)?;

        Ok(ManifestSummary::from((summary, self.network_id())))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use prelude::fixture_rtm;
    use radix_rust::hashmap;
    use radix_transactions::manifest::{
        compile_manifest as scrypto_compile_manifest, BlobProvider,
    };

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SubintentManifest;

    #[test]
    fn manifest_summary_simple() {
        let manifest = SUT::sample();
        let summary = manifest.summary().unwrap();
        pretty_assertions::assert_eq!(
            summary,
            ManifestSummary::new(
                hashmap!(
                    AccountAddress::sample() => vec![AccountWithdraw::amount(ResourceAddress::sample(), 1337)],
                ),
                hashmap!(
                    AccountAddress::sample_other() => AccountDeposits::sample(),
                ),
                [],
                [AccountAddress::sample()],
                [AccountAddress::sample_other()],
                [],
                [AccountAddress::sample()],
                [],
                Vec::<_>::sample(),
                [RetManifestClass::GeneralSubintent],
            )
        );
    }

    #[test]
    fn manifest_summary_multi_account_resources_transfer() {
        let a = AccountAddress::from_str("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q").unwrap();

        let manifest = SUT::sample_other();
        let summary = manifest.summary().unwrap();
        pretty_assertions::assert_eq!(
            summary,
            ManifestSummary::new(
                hashmap!(
                    a => vec![AccountWithdraw::sample()],
                ),
                hashmap!(
                    AccountAddress::from_str("account_sim1c8mulhl5yrk6hh4jsyldps5sdrp08r5v9wusupvzxgqvhlp4c4nwjz").unwrap() =>
                        AccountDeposits::new_for_test(
                            vec![SimpleResourceBounds::exact_fungible(ResourceAddress::sample_sim_xrd(), 150)],
                            UnspecifiedResources::NonePresent,
                        )
                    ,
                    AccountAddress::from_str("account_sim1c8ct6jdcwqrg3gzskyxuy0z933fe55fyjz6p56730r95ulzwl3ppva").unwrap() =>
                        AccountDeposits::new_for_test(
                            vec![SimpleResourceBounds::exact_fungible(ResourceAddress::sample_sim_xrd(), 50)],
                            UnspecifiedResources::NonePresent,
                        )
                    ,
                    AccountAddress::from_str("account_sim1c8s2hass5g62ckwpv78y8ykdqljtetv4ve6etcz64gveykxznj36tr").unwrap() =>
                        AccountDeposits::new_for_test(
                            vec![SimpleResourceBounds::exact_fungible(ResourceAddress::sample_sim_xrd(), 130)],
                            UnspecifiedResources::NonePresent,
                        ),
                ),
                [],
                [
                    a
                ],
                [
                    AccountAddress::from_str("account_sim1c8mulhl5yrk6hh4jsyldps5sdrp08r5v9wusupvzxgqvhlp4c4nwjz").unwrap(),
                    AccountAddress::from_str("account_sim1c8s2hass5g62ckwpv78y8ykdqljtetv4ve6etcz64gveykxznj36tr").unwrap(),
                    AccountAddress::from_str("account_sim1c8ct6jdcwqrg3gzskyxuy0z933fe55fyjz6p56730r95ulzwl3ppva").unwrap(),
                ],
                [],
                [a],
                [],
                Vec::<_>::sample(),
                [RetManifestClass::GeneralSubintent],
            )
        );
    }

    #[test]
    fn open_pre_auth_fungibles_deposit_summary() {
        let manifest_str = fixture_rtm!("open_subintent_fungibles");

        let network = NetworkID::Stokenet.network_definition();
        let man: ScryptoSubintentManifestV2 = scrypto_compile_manifest(
            manifest_str,
            &network,
            BlobProvider::new(),
        )
        .unwrap();

        let manifest: SUT = (man, NetworkID::Stokenet).try_into().unwrap();
        let summary = manifest.summary().unwrap();

        pretty_assertions::assert_eq!(
            summary.account_deposits,
            hashmap! {
                AccountAddress::from_str("account_tdx_2_129rfcz44zxflyaf6d65fdvaqtk5rlvdu8nzek2nz435zknhqure2xl").unwrap() =>
                    AccountDeposits::new_for_test(
                        vec![
                            SimpleResourceBounds::fungible("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::unknown_amount()),
                            SimpleResourceBounds::fungible("resource_tdx_2_1th4gzm9rk5s28yf5gud0a32m082g4x60d6na4ecsccte032y3xu785".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_least(6)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1tkd957yt3rwqze7elmzlphfjnmfyzkf9l5rau5ccsx9h2vs9nq3tzp".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_most(10)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5jcmwqx39ym7fw0hrlpnxwechr0mzlgulcfcye59qm9y9fa5uvdrd".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::between(100, 159)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::exact(3)),
                        ],
                        UnspecifiedResources::MayBePresent,
                    ),
            }
        )
    }

    #[test]
    fn open_pre_auth_non_fungibles_no_certain_ids_deposit_summary() {
        let manifest_str =
            fixture_rtm!("open_subintent_non_fungibles_no_certain_ids");

        let network = NetworkID::Stokenet.network_definition();
        let man: ScryptoSubintentManifestV2 = scrypto_compile_manifest(
            manifest_str,
            &network,
            BlobProvider::new(),
        )
        .unwrap();

        let manifest: SUT = (man, NetworkID::Stokenet).try_into().unwrap();
        let summary = manifest.summary().unwrap();

        pretty_assertions::assert_eq!(
            summary.account_deposits,
            hashmap!(
                AccountAddress::from_str("account_tdx_2_129rfcz44zxflyaf6d65fdvaqtk5rlvdu8nzek2nz435zknhqure2xl").unwrap() =>
                    AccountDeposits::new_for_test(
                        vec![
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::unknown_amount())),
                            ),
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1n2lj0rk7pye8h2cxs347lf70ksyzwaez0mjkssccfthp6m408hfny7".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::at_least(6))),
                            ),
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1nf8g5dhl6rxvq78j6q3kdxfkl7rweychjzyv848clhezg44rn0qgy5".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::at_most(10))),
                            ),
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1nfn4gd24pcpnqegcq07mgvz9cea4zryytswn5vmgepnan7tjqedkxp".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::between(100, 159))),
                            ),
                            SimpleResourceBounds::non_fungible(
                                "resource_tdx_2_1nt8pgfd7xj954403vfgkej25g8kcc56ldu4j3akl4vzlcfen6jcfjg".parse::<ResourceAddress>().unwrap(),
                                SimpleNonFungibleResourceBounds::new(Vec::new(), Some(SimpleCountedResourceBounds::exact(3))),
                            ),
                        ],
                        UnspecifiedResources::MayBePresent,
                    ),
            )
        );
    }

    #[test]
    fn open_pre_auth_non_fungibles_with_certain_ids_deposit_summary() {
        let manifest_str =
            fixture_rtm!("open_subintent_non_fungibles_with_certain_ids");

        let network = NetworkID::Stokenet.network_definition();
        let man: ScryptoSubintentManifestV2 = scrypto_compile_manifest(
            manifest_str,
            &network,
            BlobProvider::new(),
        )
        .unwrap();

        let manifest: SUT = (man, NetworkID::Stokenet).try_into().unwrap();
        let summary = manifest.summary().unwrap();

        let certain_ids_sample = vec![
            NonFungibleLocalId::from_str("#0#").unwrap(),
            NonFungibleLocalId::from_str("#1#").unwrap(),
            NonFungibleLocalId::from_str("#2#").unwrap(),
        ];

        let member_card_id =
            NonFungibleLocalId::from_str("<Member_103>").unwrap();

        pretty_assertions::assert_eq!(
            summary.account_deposits,
            hashmap!(
            AccountAddress::from_str("account_tdx_2_129rfcz44zxflyaf6d65fdvaqtk5rlvdu8nzek2nz435zknhqure2xl").unwrap() =>
                AccountDeposits::new_for_test(
                    vec![
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nfmxggm4plrrmc9ft9qn79g7uehqlhjaszv02dnuk85s0h9xnh3xue".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(vec![member_card_id], Some(SimpleCountedResourceBounds::unknown_amount())),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1n2lj0rk7pye8h2cxs347lf70ksyzwaez0mjkssccfthp6m408hfny7".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::unknown_amount())),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nf8g5dhl6rxvq78j6q3kdxfkl7rweychjzyv848clhezg44rn0qgy5".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), None),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nfn4gd24pcpnqegcq07mgvz9cea4zryytswn5vmgepnan7tjqedkxp".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::at_most(2))),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nt8pgfd7xj954403vfgkej25g8kcc56ldu4j3akl4vzlcfen6jcfjg".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), None),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1n2q3kj4sfa6sh45kvau2f08hfhjuls7zcevwl77vjzmgf3sea0uzzu".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::at_most(2))),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1n2rpk9w8d8kzu578jxvqr0dplctfh5clylmyqpu9kvnz7hvceh2mxe".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::at_least(2))),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1ngu8tgxvv26rpmdwxxfd8gclnsnjeew7zdcw2p3genru58a7wkmue4".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::exact(2))),
                        ),
                        SimpleResourceBounds::non_fungible(
                            "resource_tdx_2_1nfve52p2wvx0kp0eq3xaznuvwakcu5a6aqtsjqq8x30zk4wkglxmlv".parse::<ResourceAddress>().unwrap(),
                            SimpleNonFungibleResourceBounds::new(certain_ids_sample.clone(), Some(SimpleCountedResourceBounds::between(2, 5))),
                        ),
                    ],
                    UnspecifiedResources::MayBePresent,
                ),
            )
        );
    }

    #[test]
    fn test_multiple_deposits() {
        let manifest_str = fixture_rtm!("open_subintent_mix_multiple_deposits");

        let network = NetworkID::Stokenet.network_definition();
        let man: ScryptoSubintentManifestV2 = scrypto_compile_manifest(
            manifest_str,
            &network,
            BlobProvider::new(),
        )
        .unwrap();

        let manifest: SUT = (man, NetworkID::Stokenet).try_into().unwrap();
        let summary = manifest.summary().unwrap();

        pretty_assertions::assert_eq!(
            summary.account_deposits,
            hashmap!(
                AccountAddress::from_str("account_tdx_2_129rfcz44zxflyaf6d65fdvaqtk5rlvdu8nzek2nz435zknhqure2xl").unwrap() =>
                    AccountDeposits::new_for_test(
                        vec![
                            SimpleResourceBounds::fungible("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::unknown_amount()),
                            SimpleResourceBounds::fungible("resource_tdx_2_1th4gzm9rk5s28yf5gud0a32m082g4x60d6na4ecsccte032y3xu785".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_least(6)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1tkd957yt3rwqze7elmzlphfjnmfyzkf9l5rau5ccsx9h2vs9nq3tzp".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_most(10)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5jcmwqx39ym7fw0hrlpnxwechr0mzlgulcfcye59qm9y9fa5uvdrd".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::between(100, 159)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::exact(3)),

                            SimpleResourceBounds::fungible("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::unknown_amount()),
                            SimpleResourceBounds::fungible("resource_tdx_2_1th4gzm9rk5s28yf5gud0a32m082g4x60d6na4ecsccte032y3xu785".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_least(6)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1tkd957yt3rwqze7elmzlphfjnmfyzkf9l5rau5ccsx9h2vs9nq3tzp".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_most(10)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5jcmwqx39ym7fw0hrlpnxwechr0mzlgulcfcye59qm9y9fa5uvdrd".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::between(100, 159)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::exact(3)),
                        ],
                        UnspecifiedResources::MayBePresent,
                    ),
                    AccountAddress::from_str("account_tdx_2_1288u4aka2dm8787texaeta8ruzhcr7dyckmnck5wt0llrm6x0ak7e4").unwrap() => AccountDeposits::new_for_test(
                        vec![
                            SimpleResourceBounds::fungible("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::unknown_amount()),
                            SimpleResourceBounds::fungible("resource_tdx_2_1th4gzm9rk5s28yf5gud0a32m082g4x60d6na4ecsccte032y3xu785".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_least(6)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1tkd957yt3rwqze7elmzlphfjnmfyzkf9l5rau5ccsx9h2vs9nq3tzp".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::at_most(10)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5jcmwqx39ym7fw0hrlpnxwechr0mzlgulcfcye59qm9y9fa5uvdrd".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::between(100, 159)),
                            SimpleResourceBounds::fungible("resource_tdx_2_1t5dapa24l4xvwqtqe2jrdphtn7ga46gw67wr9fwn4gp532myfjqpck".parse::<ResourceAddress>().unwrap(), SimpleCountedResourceBounds::exact(3)),
                        ],
                        UnspecifiedResources::NonePresent,
                    ),
            )
        )
    }
}
