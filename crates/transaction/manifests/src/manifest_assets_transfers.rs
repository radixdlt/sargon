use bucket_factory::BucketFactory;

use crate::prelude::*;

pub trait TransactionManifestAssetTransfers: Sized {
    fn per_asset_transfers(transfers: PerAssetTransfers)
        -> TransactionManifest;

    /// Uses `per_asset_transfers` after having transposed the `PerRecipientAssetTransfers`
    /// into `PerAssetTransfers`. We always use `PerAssetTransfers` when building the manifest
    /// since it is more efficient (allows a single withdraw per resource) => fewer instruction =>
    /// cheaper TX fee for user.
    fn per_recipient_transfers(
        transfers: PerRecipientAssetTransfers,
    ) -> TransactionManifest {
        Self::per_asset_transfers(transfers.transpose())
    }
}

impl TransactionManifestAssetTransfers for TransactionManifest {
    fn per_asset_transfers(transfers: PerAssetTransfers) -> Self {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        let bucket_factory = BucketFactory::default();
        let from_account = &transfers.from_account;

        for fungible in transfers.fungible_resources {
            let divisibility = fungible.resource.divisibility;

            let resource_address = &fungible.resource.resource_address;

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

        TransactionManifest::sargon_built(builder, from_account.network_id())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn trivial() {
        let sut = SUT::per_asset_transfers(PerAssetTransfers::new(
            AccountAddress::sample(),
            [],
            [],
        ));
        manifest_eq(sut, ""); // empty!
    }

    #[test]
    fn use_try_deposit_or_abort_is_false() {
        let transfers = PerRecipientAssetTransfers::new(
            AccountAddress::sample_mainnet(),
            [PerRecipientAssetTransfer::new(
                TransferRecipient::AddressOfExternalAccount {
                    value: AccountAddress::sample_mainnet_other(),
                },
                [PerRecipientFungibleTransfer::new(
                    ResourceAddress::sample_mainnet_candy(),
                    1337,
                    false,
                    None,
                )],
                [PerRecipientNonFungibleTransfer::new(
                    NonFungibleResourceAddress::sample_mainnet_other(),
                    false,
                    [NonFungibleLocalId::integer(237)],
                )],
            )],
        );
        let manifest = SUT::per_recipient_transfers(transfers);
        manifest_eq(
            manifest,
            r##"
            CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw"
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Decimal("1337")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Decimal("1337")
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
            "deposit"
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw_non_fungibles"
            Address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#237#")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#237#")
            )
            Bucket("bucket2")
        ;
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
            "deposit"
            Bucket("bucket2")
        ;
        "##,
        );
    }

    #[test]
    fn multi_token_multi_recipient() {
        let sut = SUT::per_recipient_transfers(
            PerRecipientAssetTransfers::sample_other(),
        );

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
        let sut = SUT::per_asset_transfers(PerAssetTransfers::sample());
        manifest_eq(
            sut,
            r##"
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
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
            Address("account_rdx129akrrsd9ctuphe99lesa8cf6auc5vqwdd2lu0ej6csncnuw9eedgv")
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
            Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
            "try_deposit_or_abort"
            Bucket("bucket2")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
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
            Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
            "try_deposit_or_abort"
            Bucket("bucket3")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw_non_fungibles"
            Address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("{deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead}"),
                NonFungibleLocalId("<foobar>")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("{deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead}"),
                NonFungibleLocalId("<foobar>")
            )
            Bucket("bucket4")
        ;
        CALL_METHOD
            Address("account_rdx129akrrsd9ctuphe99lesa8cf6auc5vqwdd2lu0ej6csncnuw9eedgv")
            "try_deposit_or_abort"
            Bucket("bucket4")
            Enum<0u8>()
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("<foobar>")
            )
            Bucket("bucket5")
        ;
        CALL_METHOD
            Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
            "try_deposit_or_abort"
            Bucket("bucket5")
            Enum<0u8>()
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw_non_fungibles"
            Address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("<foobar>")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("<foobar>")
            )
            Bucket("bucket6")
        ;
        CALL_METHOD
            Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
            "try_deposit_or_abort"
            Bucket("bucket6")
            Enum<0u8>()
        ;
        "##,
        );
    }
}
