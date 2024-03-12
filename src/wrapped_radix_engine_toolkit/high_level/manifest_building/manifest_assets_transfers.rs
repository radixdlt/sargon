use crate::prelude::*;

impl TransactionManifest {
    pub fn assets_transfers(transfers: AssetsTransfers) -> Self {
        let mut builder = ScryptoManifestBuilder::new();
        let bucket_factory = BucketFactory::default();
        let from_account = &transfers.from_account;

        for fungible in transfers.fungible_resources {
            let divisibility = fungible.resource.divisibility;

            let resource_address = &fungible.resource.address;

            builder = builder.withdraw_from_account(
                from_account,
                resource_address,
                fungible.total_transfer_amount(),
            );

            for transfer in fungible.transfers {
                let bucket = &bucket_factory.next();

                builder = builder.take_from_worktop(
                    resource_address,
                    transfer.amount(divisibility),
                    bucket,
                );

                builder = transfer.deposit_instruction(builder, bucket);
            }
        }

        for non_fungible in transfers.non_fungible_resources {
            let resource_address = &non_fungible.resource;

            builder = builder.withdraw_non_fungibles_from_account(
                from_account,
                resource_address,
                non_fungible.all_ids(),
            );

            for transfer in non_fungible.transfers {
                let bucket = &bucket_factory.next();

                builder = builder.take_non_fungibles_from_worktop(
                    resource_address,
                    transfer.local_ids(),
                    bucket,
                );

                builder = transfer.deposit_instruction(builder, bucket);
            }
        }

        let scrypto_manifest = builder.build();

        TransactionManifest::from_scrypto(
            scrypto_manifest,
            from_account.network_id(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn trivial() {
        let sut = SUT::assets_transfers(AssetsTransfers::new(
            AccountAddress::sample(),
            [],
            [],
        ));
        manifest_eq(sut, ""); // empty!
    }

    #[test]
    fn multi_token_multi_recipient() {
        let sender: AccountAddress = "account_tdx_2_128rkfzdztjpgajucstydar2gz2vp9jj779k33jy3gect2rh5r28rgn".parse().unwrap();
        let recip0: AccountAddress = "account_tdx_2_129e9h6zp5z08qkc0q5tdqz9zc67gg2k7tergrj9erznmke6qeevmsv".parse().unwrap();
        let recip1: AccountAddress = "account_tdx_2_128a45a7hetjfpfqdlsp07eyrmhq7edldefgd7263jd58puzuq09qks".parse().unwrap();

        let nft_c0: NonFungibleResourceAddress = "resource_tdx_2_1n2sjxxtk6vm6pvk8dxr798e8zpxqz50q5wlmldlat0qhh04u2mwmy8".parse().unwrap();
        let nft_c1: NonFungibleResourceAddress = "resource_tdx_2_1ntuaekqexa73m9en04jj3vdt3fk9u9kdk8q9su4efldun2y7nd3cga".parse().unwrap();

        let fung_0: ResourceAddress = ResourceAddress::sample_stokenet_xrd();
        let fung_1: ResourceAddress =
            ResourceAddress::sample_stokenet_gc_tokens();

        let transfers = AssetsTransfers::new(
            sender,
            [
                TransfersOfFungibleResource::new(
                    FungibleResource::new(fung_0.clone(), 18),
                    [
                        FungibleTransfer::new(recip0.clone(), true, 30),
                        FungibleTransfer::new(recip1.clone(), true, 50),
                    ],
                ),
                TransfersOfFungibleResource::new(
                    FungibleResource::new(fung_1.clone(), 18),
                    [
                        FungibleTransfer::new(recip0.clone(), true, 3),
                        FungibleTransfer::new(recip1.clone(), true, 5),
                    ],
                ),
            ],
            [
                TransfersOfNonFungibleResource::new(
                    nft_c0.clone(),
                    [
                        NonFungibleTransfer::new(
                            recip0.clone(),
                            true,
                            [
                                NonFungibleLocalId::integer(40),
                                NonFungibleLocalId::integer(48),
                            ],
                        ),
                        NonFungibleTransfer::new(
                            recip1.clone(),
                            true,
                            [
                                NonFungibleLocalId::integer(34),
                                NonFungibleLocalId::integer(22),
                            ],
                        ),
                    ],
                ),
                TransfersOfNonFungibleResource::new(
                    nft_c1.clone(),
                    [
                        NonFungibleTransfer::new(
                            recip0.clone(),
                            true,
                            [
                                NonFungibleLocalId::integer(21),
                                NonFungibleLocalId::integer(3),
                            ],
                        ),
                        NonFungibleTransfer::new(
                            recip1.clone(),
                            true,
                            [
                                NonFungibleLocalId::integer(15),
                                NonFungibleLocalId::integer(9),
                                NonFungibleLocalId::integer(13),
                            ],
                        ),
                    ],
                ),
            ],
        );

        let sut = SUT::assets_transfers(transfers);
        manifest_eq(
            sut,
            r##"
        CALL_METHOD
            Address("account_tdx_2_128rkfzdztjpgajucstydar2gz2vp9jj779k33jy3gect2rh5r28rgn")
            "withdraw"
            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
            Decimal("80")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
            Decimal("30")
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_tdx_2_129e9h6zp5z08qkc0q5tdqz9zc67gg2k7tergrj9erznmke6qeevmsv")
            "try_deposit_or_abort"
            Bucket("bucket1")
            Enum<0u8>()
        ;
        TAKE_FROM_WORKTOP
            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
            Decimal("50")
            Bucket("bucket2")
        ;
        CALL_METHOD
            Address("account_tdx_2_128a45a7hetjfpfqdlsp07eyrmhq7edldefgd7263jd58puzuq09qks")
            "try_deposit_or_abort"
            Bucket("bucket2")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_tdx_2_128rkfzdztjpgajucstydar2gz2vp9jj779k33jy3gect2rh5r28rgn")
            "withdraw"
            Address("resource_tdx_2_1thqcgjw37fjgycpvqr52nx4jcsdeuq75mf2nywme07kzsuds9a4psp")
            Decimal("8")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_tdx_2_1thqcgjw37fjgycpvqr52nx4jcsdeuq75mf2nywme07kzsuds9a4psp")
            Decimal("3")
            Bucket("bucket3")
        ;
        CALL_METHOD
            Address("account_tdx_2_129e9h6zp5z08qkc0q5tdqz9zc67gg2k7tergrj9erznmke6qeevmsv")
            "try_deposit_or_abort"
            Bucket("bucket3")
            Enum<0u8>()
        ;
        TAKE_FROM_WORKTOP
            Address("resource_tdx_2_1thqcgjw37fjgycpvqr52nx4jcsdeuq75mf2nywme07kzsuds9a4psp")
            Decimal("5")
            Bucket("bucket4")
        ;
        CALL_METHOD
            Address("account_tdx_2_128a45a7hetjfpfqdlsp07eyrmhq7edldefgd7263jd58puzuq09qks")
            "try_deposit_or_abort"
            Bucket("bucket4")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_tdx_2_128rkfzdztjpgajucstydar2gz2vp9jj779k33jy3gect2rh5r28rgn")
            "withdraw_non_fungibles"
            Address("resource_tdx_2_1n2sjxxtk6vm6pvk8dxr798e8zpxqz50q5wlmldlat0qhh04u2mwmy8")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#40#"),
                NonFungibleLocalId("#48#"),
                NonFungibleLocalId("#34#"),
                NonFungibleLocalId("#22#")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_tdx_2_1n2sjxxtk6vm6pvk8dxr798e8zpxqz50q5wlmldlat0qhh04u2mwmy8")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#40#"),
                NonFungibleLocalId("#48#")
            )
            Bucket("bucket5")
        ;
        CALL_METHOD
            Address("account_tdx_2_129e9h6zp5z08qkc0q5tdqz9zc67gg2k7tergrj9erznmke6qeevmsv")
            "try_deposit_or_abort"
            Bucket("bucket5")
            Enum<0u8>()
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_tdx_2_1n2sjxxtk6vm6pvk8dxr798e8zpxqz50q5wlmldlat0qhh04u2mwmy8")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#34#"),
                NonFungibleLocalId("#22#")
            )
            Bucket("bucket6")
        ;
        CALL_METHOD
            Address("account_tdx_2_128a45a7hetjfpfqdlsp07eyrmhq7edldefgd7263jd58puzuq09qks")
            "try_deposit_or_abort"
            Bucket("bucket6")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_tdx_2_128rkfzdztjpgajucstydar2gz2vp9jj779k33jy3gect2rh5r28rgn")
            "withdraw_non_fungibles"
            Address("resource_tdx_2_1ntuaekqexa73m9en04jj3vdt3fk9u9kdk8q9su4efldun2y7nd3cga")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#21#"),
                NonFungibleLocalId("#3#"),
                NonFungibleLocalId("#15#"),
                NonFungibleLocalId("#9#"),
                NonFungibleLocalId("#13#")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_tdx_2_1ntuaekqexa73m9en04jj3vdt3fk9u9kdk8q9su4efldun2y7nd3cga")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#21#"),
                NonFungibleLocalId("#3#")
            )
            Bucket("bucket7")
        ;
        CALL_METHOD
            Address("account_tdx_2_129e9h6zp5z08qkc0q5tdqz9zc67gg2k7tergrj9erznmke6qeevmsv")
            "try_deposit_or_abort"
            Bucket("bucket7")
            Enum<0u8>()
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_tdx_2_1ntuaekqexa73m9en04jj3vdt3fk9u9kdk8q9su4efldun2y7nd3cga")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#15#"),
                NonFungibleLocalId("#9#"),
                NonFungibleLocalId("#13#")
            )
            Bucket("bucket8")
        ;
        CALL_METHOD
            Address("account_tdx_2_128a45a7hetjfpfqdlsp07eyrmhq7edldefgd7263jd58puzuq09qks")
            "try_deposit_or_abort"
            Bucket("bucket8")
            Enum<0u8>()
        ;        
            "##,
        );
    }

    #[test]
    fn simple() {
        let sut = SUT::assets_transfers(AssetsTransfers::sample());
        manifest_eq(
            sut,
            r##"
        CALL_METHOD
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
            "withdraw"
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("987891.25712718281828")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("237.13372718281828")
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_rdx12xvg2sssh0rpca6e8xyqv5vf4nqu928083yzf0fdrnvjdz2pvc000x")
            "try_deposit_or_abort"
            Bucket("bucket1")
            Enum<0u8>()
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("987654.1234")
            Bucket("bucket2")
        ;
        CALL_METHOD
            Address("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69")
            "try_deposit_or_abort"
            Bucket("bucket2")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
            "withdraw"
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Decimal("987654.1234")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Decimal("987654.1234")
            Bucket("bucket3")
        ;
        CALL_METHOD
            Address("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69")
            "try_deposit_or_abort"
            Bucket("bucket3")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
            "withdraw_non_fungibles"
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("{deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead}"),
                NonFungibleLocalId("<foobar>")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("{deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead}"),
                NonFungibleLocalId("<foobar>")
            )
            Bucket("bucket4")
        ;
        CALL_METHOD
            Address("account_rdx12xvg2sssh0rpca6e8xyqv5vf4nqu928083yzf0fdrnvjdz2pvc000x")
            "try_deposit_or_abort"
            Bucket("bucket4")
            Enum<0u8>()
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("<foobar>")
            )
            Bucket("bucket5")
        ;
        CALL_METHOD
            Address("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69")
            "try_deposit_or_abort"
            Bucket("bucket5")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
            "withdraw_non_fungibles"
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("<foobar>")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("<foobar>")
            )
            Bucket("bucket6")
        ;
        CALL_METHOD
            Address("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69")
            "try_deposit_or_abort"
            Bucket("bucket6")
            Enum<0u8>()
        ;
        "##,
        );
    }
}
