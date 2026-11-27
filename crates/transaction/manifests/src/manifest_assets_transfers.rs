use bucket_factory::BucketFactory;

use crate::prelude::*;

pub trait TransactionManifestAssetTransfers: Sized {
    /// The manifest of `transfers`, proving `required_badges` (the badges the withdraw rules of
    /// restricted resources ask for, see `TransferRequiredBadges`).
    ///
    /// When there are badges to prove, the proofs are created first, then every resource that is
    /// not one of the badges is withdrawn and deposited (the proofs cover a withdrawer or a
    /// depositor role asking for the badges). If badges are transferred too, the badge proofs are
    /// then popped from the auth zone and dropped, because a proof locks the badge in its vault,
    /// and the badges are withdrawn and deposited last. Only the proofs created here are dropped
    /// (the most recent ones, LIFO): the signatures and any proof put in the auth zone before
    /// them (e.g. the access controller proof of a securified account) still authorize the
    /// withdrawals of the badges. A depositor rule on a transferred badge itself is not covered
    /// by the badge proofs, which are gone by then.
    fn per_asset_transfers(
        transfers: PerAssetTransfers,
        required_badges: Vec<ResourceSpecifier>,
    ) -> TransactionManifest;

    /// Uses `per_asset_transfers` after having transposed the `PerRecipientAssetTransfers`
    /// into `PerAssetTransfers`. We always use `PerAssetTransfers` when building the manifest
    /// since it is more efficient (allows a single withdraw per resource) => fewer instruction =>
    /// cheaper TX fee for user.
    fn per_recipient_transfers(
        transfers: PerRecipientAssetTransfers,
        required_badges: Vec<ResourceSpecifier>,
    ) -> TransactionManifest {
        Self::per_asset_transfers(transfers.transpose(), required_badges)
    }
}

impl TransactionManifestAssetTransfers for TransactionManifest {
    fn per_asset_transfers(
        transfers: PerAssetTransfers,
        required_badges: Vec<ResourceSpecifier>,
    ) -> Self {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        let bucket_factory = BucketFactory::default();
        let from_account = &transfers.from_account;

        let badges = required_badges
            .iter()
            .map(ResourceSpecifier::get_address)
            .collect::<IndexSet<_>>();

        let badge_proofs = required_badges.len();
        for badge in required_badges {
            builder = match badge {
                ResourceSpecifier::Fungible {
                    resource_address,
                    amount,
                } => builder.create_proof_from_account_of_amount(
                    from_account,
                    ScryptoResourceAddress::from(resource_address),
                    ScryptoDecimal192::from(amount),
                ),
                ResourceSpecifier::NonFungible {
                    resource_address,
                    ids,
                } => builder.create_proof_from_account_of_non_fungibles(
                    from_account,
                    ScryptoResourceAddress::from(resource_address),
                    ids.into_iter().map(ScryptoNonFungibleLocalId::from),
                ),
            };
        }

        let (badge_fungibles, fungibles): (Vec<_>, Vec<_>) = transfers
            .fungible_resources
            .into_iter()
            .partition(|f| badges.contains(&f.resource.resource_address));
        let (badge_non_fungibles, non_fungibles): (Vec<_>, Vec<_>) = transfers
            .non_fungible_resources
            .into_iter()
            .partition(|nf| badges.contains(&nf.resource));

        let transfer_all = |mut builder: ScryptoTransactionManifestBuilder,
                            fungibles: Vec<
            PerAssetTransfersOfFungibleResource,
        >,
                            non_fungibles: Vec<
            PerAssetTransfersOfNonFungibleResource,
        >| {
            for fungible in fungibles {
                builder = transfer_fungible(
                    builder,
                    &bucket_factory,
                    from_account,
                    fungible,
                );
            }
            for non_fungible in non_fungibles {
                builder = transfer_non_fungible(
                    builder,
                    &bucket_factory,
                    from_account,
                    non_fungible,
                );
            }
            builder
        };

        builder = transfer_all(builder, fungibles, non_fungibles);
        if !badge_fungibles.is_empty() || !badge_non_fungibles.is_empty() {
            // A proof locks the badge in its vault: drop the badge proofs (the last ones put in
            // the auth zone, nothing in between creates proofs) so that the badges can be
            // withdrawn, keeping the earlier proofs, which may authorize those withdrawals.
            for index in (1..=badge_proofs).rev() {
                let proof = format!("badge_proof{index}");
                builder = builder.pop_from_auth_zone(proof.as_str());
                builder = builder.drop_proof(proof.as_str());
            }
            builder =
                transfer_all(builder, badge_fungibles, badge_non_fungibles);
        }

        TransactionManifest::sargon_built(builder, from_account.network_id())
    }
}

/// Withdraws the total of `fungible`, then deposits each of its transfers.
fn transfer_fungible(
    mut builder: ScryptoTransactionManifestBuilder,
    bucket_factory: &BucketFactory,
    from_account: &AccountAddress,
    fungible: PerAssetTransfersOfFungibleResource,
) -> ScryptoTransactionManifestBuilder {
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
    builder
}

/// Withdraws all the ids of `non_fungible`, then deposits each of its transfers.
fn transfer_non_fungible(
    mut builder: ScryptoTransactionManifestBuilder,
    bucket_factory: &BucketFactory,
    from_account: &AccountAddress,
    non_fungible: PerAssetTransfersOfNonFungibleResource,
) -> ScryptoTransactionManifestBuilder {
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
    builder
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn trivial() {
        let sut = SUT::per_asset_transfers(
            PerAssetTransfers::new(AccountAddress::sample(), [], []),
            Vec::new(),
        );
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
        let manifest = SUT::per_recipient_transfers(transfers, Vec::new());
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
            Vec::new(),
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
        let sut =
            SUT::per_asset_transfers(PerAssetTransfers::sample(), Vec::new());
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

    fn recipient() -> TransferRecipient {
        TransferRecipient::AddressOfExternalAccount {
            value: AccountAddress::sample_mainnet_other(),
        }
    }

    /// XRD stands for a restricted token here, Candy for the fungible badge its rule asks for.
    fn restricted_token_with_badge(
        badge_transferred_too: bool,
    ) -> (PerAssetTransfers, Vec<ResourceSpecifier>) {
        let restricted = PerAssetTransfersOfFungibleResource::new(
            PerAssetFungibleResource::new(
                ResourceAddress::sample_mainnet_xrd(),
                None,
            ),
            [PerAssetFungibleTransfer::new(
                recipient(),
                true,
                Decimal::ten(),
            )],
        );
        let badge = PerAssetTransfersOfFungibleResource::new(
            PerAssetFungibleResource::new(
                ResourceAddress::sample_mainnet_candy(),
                None,
            ),
            [PerAssetFungibleTransfer::new(
                recipient(),
                true,
                Decimal::one(),
            )],
        );
        let fungibles = if badge_transferred_too {
            vec![restricted, badge]
        } else {
            vec![restricted]
        };
        (
            PerAssetTransfers::new(
                AccountAddress::sample_mainnet(),
                fungibles,
                [],
            ),
            vec![ResourceSpecifier::fungible(
                ResourceAddress::sample_mainnet_candy(),
                Decimal::one(),
            )],
        )
    }

    #[test]
    fn required_badges_are_proven_and_not_dropped_when_no_badge_is_sent() {
        let (transfers, required_badges) = restricted_token_with_badge(false);
        let sut = SUT::per_asset_transfers(transfers, required_badges);
        manifest_eq(
            sut,
            r##"
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "create_proof_of_amount"
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Decimal("1")
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw"
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("10")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("10")
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
            "try_deposit_or_abort"
            Bucket("bucket1")
            Enum<0u8>()
        ;
            "##,
        );
    }

    #[test]
    fn a_badge_transferred_with_the_restricted_token_is_withdrawn_after_the_proofs_are_dropped(
    ) {
        let (transfers, required_badges) = restricted_token_with_badge(true);
        let sut = SUT::per_asset_transfers(transfers, required_badges);
        manifest_eq(
            sut,
            r##"
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "create_proof_of_amount"
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Decimal("1")
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw"
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("10")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("10")
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
            "try_deposit_or_abort"
            Bucket("bucket1")
            Enum<0u8>()
        ;
        POP_FROM_AUTH_ZONE
            Proof("proof1")
        ;
        DROP_PROOF
            Proof("proof1")
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw"
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Decimal("1")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
            Decimal("1")
            Bucket("bucket2")
        ;
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
            "try_deposit_or_abort"
            Bucket("bucket2")
            Enum<0u8>()
        ;
            "##,
        );
    }

    #[test]
    fn every_badge_proof_is_dropped_before_the_badges_are_sent() {
        // Two badges proven, one of them sent: exactly the two proofs created here are popped.
        let (transfers, mut required_badges) =
            restricted_token_with_badge(true);
        required_badges.push(ResourceSpecifier::fungible(
            ResourceAddress::sample_mainnet_other(),
            Decimal::one(),
        ));
        let instructions = SUT::per_asset_transfers(transfers, required_badges)
            .instructions_string();
        assert_eq!(instructions.matches("create_proof_of_amount").count(), 2);
        assert_eq!(instructions.matches("POP_FROM_AUTH_ZONE").count(), 2);
        assert_eq!(instructions.matches("DROP_PROOF").count(), 2);
        // Only the badge moves after the drops.
        let after_drops =
            &instructions[instructions.rfind("DROP_PROOF").unwrap()..];
        assert!(after_drops
            .contains(&ResourceAddress::sample_mainnet_candy().to_string()));
        assert!(!after_drops
            .contains(&ResourceAddress::sample_mainnet_xrd().to_string()));
    }

    #[test]
    fn non_fungible_badges_are_proven_by_id() {
        let nft_badge = ResourceAddress::sample_mainnet_nft_other();
        let transfers = PerAssetTransfers::new(
            AccountAddress::sample_mainnet(),
            [],
            [
                PerAssetTransfersOfNonFungibleResource::new(
                    NonFungibleResourceAddress::sample_mainnet(),
                    [PerAssetNonFungibleTransfer::new(
                        recipient(),
                        true,
                        [NonFungibleLocalId::integer(7)],
                    )],
                ),
                PerAssetTransfersOfNonFungibleResource::new(
                    nft_badge,
                    [PerAssetNonFungibleTransfer::new(
                        recipient(),
                        true,
                        [NonFungibleLocalId::integer(1)],
                    )],
                ),
            ],
        );
        let sut = SUT::per_asset_transfers(
            transfers,
            vec![ResourceSpecifier::non_fungible(
                nft_badge,
                vec![NonFungibleLocalId::integer(1)],
            )],
        );
        manifest_eq(
            sut,
            r##"
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "create_proof_of_non_fungibles"
            Address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#1#")
            )
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw_non_fungibles"
            Address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#7#")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#7#")
            )
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
            "try_deposit_or_abort"
            Bucket("bucket1")
            Enum<0u8>()
        ;
        POP_FROM_AUTH_ZONE
            Proof("proof1")
        ;
        DROP_PROOF
            Proof("proof1")
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw_non_fungibles"
            Address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#1#")
            )
        ;
        TAKE_NON_FUNGIBLES_FROM_WORKTOP
            Address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("#1#")
            )
            Bucket("bucket2")
        ;
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
            "try_deposit_or_abort"
            Bucket("bucket2")
            Enum<0u8>()
        ;
            "##,
        );
    }

    #[test]
    fn per_recipient_transfers_get_the_required_badges_too() {
        let (transfers, required_badges) = restricted_token_with_badge(true);
        let per_recipient = PerRecipientAssetTransfers::new(
            transfers.from_account,
            [PerRecipientAssetTransfer::new(
                recipient(),
                [
                    PerRecipientFungibleTransfer::new(
                        ResourceAddress::sample_mainnet_xrd(),
                        Decimal::ten(),
                        true,
                        None,
                    ),
                    PerRecipientFungibleTransfer::new(
                        ResourceAddress::sample_mainnet_candy(),
                        Decimal::one(),
                        true,
                        None,
                    ),
                ],
                [],
            )],
        );
        assert_eq!(
            SUT::per_recipient_transfers(
                per_recipient,
                required_badges.clone()
            ),
            SUT::per_asset_transfers(transfers, required_badges)
        );
    }

    #[test]
    fn a_transfer_with_required_badges_is_still_classified_as_a_transfer() {
        let (transfers, required_badges) = restricted_token_with_badge(true);
        let sut = SUT::per_asset_transfers(transfers, required_badges);
        let summary = sut.summary().unwrap();
        assert!(summary
            .classification
            .contains(&ManifestClassification::Transfer));
    }
}

/// Executes the manifests on a ledger simulator: they must succeed, and still be classified as a
/// transfer by the execution summary the wallets show.
#[cfg(test)]
mod ledger_tests {
    use super::*;
    use radix_engine::transaction::{ExecutionConfig, TransactionReceipt};
    use radix_engine_interface::prelude::*;
    use radix_engine_interface::prelude::{
        ComponentAddress, NonFungibleGlobalId,
    };
    use radix_transactions::prelude::ManifestBuilder;

    use crate::prelude::{
        AccountAddress, Decimal192, ExecutionSummary, NetworkID,
        PerAssetFungibleResource, PerAssetFungibleTransfer, PerAssetTransfers,
        PerAssetTransfersOfFungibleResource, ResourceAddress,
        ResourceSpecifier, TransactionManifest, TransferRecipient,
    };

    type ScryptoAddress = radix_engine_interface::prelude::ResourceAddress;

    struct Setup {
        ledger: scrypto_test::ledger_simulator::DefaultLedgerSimulator,
        signer: NonFungibleGlobalId,
        /// Pays the fees.
        fee_payer: ComponentAddress,
        /// A proof the sender's owner role needs, put in the auth zone first, like the access
        /// controller proof of a securified account: (holder, owner badge).
        owner_proof: Option<(ComponentAddress, ScryptoAddress)>,
        sender: ComponentAddress,
        recipient: ComponentAddress,
        badge: ScryptoAddress,
        token: ScryptoAddress,
    }

    /// A sender holding 10 badges and 100 tokens that only holders of the badge may withdraw
    /// and deposit. With `owned_by_a_badge`, the sender's owner role asks for an owner badge held
    /// by another account (like a securified account and its access controller), instead of
    /// the signature.
    fn setup(owned_by_a_badge: bool) -> Setup {
        let mut ledger =
            LedgerSimulatorBuilder::new().without_kernel_trace().build();
        let (key, _, signing_account) = ledger.new_allocated_account();
        let (_, _, recipient) = ledger.new_allocated_account();
        let signer = NonFungibleGlobalId::from_public_key(key);
        let (sender, owner_proof) = if owned_by_a_badge {
            let owner_badge =
                ledger.create_non_fungible_resource(signing_account);
            let sender = ledger.new_account_advanced(OwnerRole::Fixed(rule!(
                require(owner_badge)
            )));
            (sender, Some((signing_account, owner_badge)))
        } else {
            (signing_account, None)
        };
        let badge = ledger.create_fungible_resource(dec!(10), 0, sender);

        let restricted = rule!(require(badge));
        let mut builder = ManifestBuilder::new().lock_fee_from_faucet();
        if let Some((holder, owner_badge)) = owner_proof {
            builder = builder.create_proof_from_account_of_non_fungibles(
                holder,
                owner_badge,
                [ScryptoNonFungibleLocalId::integer(1)],
            );
        }
        let manifest = builder
            .create_proof_from_account_of_amount(sender, badge, dec!(1))
            .create_fungible_resource(
                OwnerRole::None,
                true,
                0,
                FungibleResourceRoles {
                    withdraw_roles: withdraw_roles! {
                        withdrawer => restricted.clone();
                        withdrawer_updater => rule!(deny_all);
                    },
                    deposit_roles: deposit_roles! {
                        depositor => restricted;
                        depositor_updater => rule!(deny_all);
                    },
                    ..Default::default()
                },
                metadata!(),
                Some(dec!(100)),
            )
            .try_deposit_entire_worktop_or_abort(sender, None)
            .build();
        let token = ledger
            .execute_manifest(manifest, vec![signer.clone()])
            .expect_commit(true)
            .new_resource_addresses()[0];

        Setup {
            ledger,
            signer,
            fee_payer: signing_account,
            owner_proof,
            sender,
            recipient,
            badge,
            token,
        }
    }

    fn sargon<A, B: From<(A, NetworkID)>>(address: A) -> B {
        B::from((address, NetworkID::Simulator))
    }

    /// Transfers `amounts` of each resource from the sender to the recipient, proving one badge,
    /// after the fee lock and the owner proof (if any), as the wallets put them in front.
    fn transfer(
        setup: &mut Setup,
        amounts: Vec<(ScryptoAddress, u64)>,
    ) -> (TransactionReceipt, TransactionManifest) {
        let badge = ResourceSpecifier::fungible(
            sargon::<_, ResourceAddress>(setup.badge),
            Decimal192::one(),
        );
        transfer_proving(setup, amounts, vec![badge])
    }

    fn transfer_proving(
        setup: &mut Setup,
        amounts: Vec<(ScryptoAddress, u64)>,
        required_badges: Vec<ResourceSpecifier>,
    ) -> (TransactionReceipt, TransactionManifest) {
        let sender: AccountAddress = sargon(setup.sender);
        let recipient = TransferRecipient::AddressOfExternalAccount {
            value: sargon(setup.recipient),
        };
        let fungibles = amounts.into_iter().map(|(resource, amount)| {
            PerAssetTransfersOfFungibleResource::new(
                PerAssetFungibleResource::new(sargon(resource), 0),
                [PerAssetFungibleTransfer::new(
                    recipient.clone(),
                    true,
                    Decimal192::from(amount),
                )],
            )
        });
        let transfer = TransactionManifest::per_asset_transfers(
            PerAssetTransfers::new(sender, fungibles, []),
            required_badges,
        );

        let fee_payer: AccountAddress = sargon(setup.fee_payer);
        let mut prefix = format!(
            "CALL_METHOD Address(\"{fee_payer}\") \"lock_fee\" Decimal(\"10\");\n"
        );
        if let Some((holder, owner_badge)) = setup.owner_proof {
            let holder: AccountAddress = sargon(holder);
            let owner_badge: ResourceAddress = sargon(owner_badge);
            prefix += &format!(
                "CALL_METHOD Address(\"{holder}\") \"create_proof_of_non_fungibles\" \
                 Address(\"{owner_badge}\") Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));\n"
            );
        }
        let manifest = TransactionManifest::new(
            prefix + &transfer.instructions_string(),
            NetworkID::Simulator,
            Blobs::default(),
        )
        .unwrap();

        let receipt = setup.ledger.execute_manifest_with_execution_config(
            manifest.scrypto_manifest(),
            vec![setup.signer.clone()],
            // The execution summary needs the execution trace.
            ExecutionConfig::for_test_transaction()
                .with_execution_trace(Some(15)),
        );
        (receipt, manifest)
    }

    fn execution_summary(
        receipt: TransactionReceipt,
        manifest: &TransactionManifest,
    ) -> ExecutionSummary {
        ExecutionSummary::from((
            manifest
                .ret_dynamically_analyze(
                    ScryptoRuntimeToolkitTransactionReceipt::try_from(receipt)
                        .unwrap(),
                )
                .unwrap(),
            NetworkID::Simulator,
        ))
    }

    #[test]
    fn without_the_badge_proof_the_restricted_token_is_not_transferred() {
        let mut setup = setup(false);
        let token = setup.token;
        let (receipt, _) =
            transfer_proving(&mut setup, vec![(token, 10)], vec![]);
        receipt.expect_commit_failure();
    }

    #[test]
    fn a_withdraw_and_deposit_restricted_token_is_transferred() {
        let mut setup = setup(false);
        let token = setup.token;
        let (receipt, manifest) = transfer(&mut setup, vec![(token, 10)]);
        receipt.expect_commit_success();
        assert_eq!(
            setup.ledger.get_component_balance(setup.recipient, token),
            dec!(10)
        );
        assert!(matches!(
            execution_summary(receipt, &manifest).detailed_classification,
            Some(DetailedManifestClass::Transfer { .. })
        ));
    }

    #[test]
    fn all_the_badges_are_transferred_with_the_restricted_token() {
        let mut setup = setup(false);
        let (token, badge) = (setup.token, setup.badge);
        // All 10 badges: the one locked by the proof too.
        let (receipt, manifest) =
            transfer(&mut setup, vec![(token, 10), (badge, 10)]);
        receipt.expect_commit_success();
        assert_eq!(
            setup.ledger.get_component_balance(setup.recipient, token),
            dec!(10)
        );
        assert_eq!(
            setup.ledger.get_component_balance(setup.recipient, badge),
            dec!(10)
        );
        assert!(matches!(
            execution_summary(receipt, &manifest).detailed_classification,
            Some(DetailedManifestClass::Transfer { .. })
        ));
    }

    #[test]
    fn the_proofs_put_in_the_auth_zone_before_the_badge_proofs_are_kept() {
        // The sender's owner role asks for an owner badge proven first (like the access
        // controller proof of a securified account): it must survive the dropping of the badge
        // proofs, to authorize the withdrawal of the badges.
        let mut setup = setup(true);
        let (token, badge) = (setup.token, setup.badge);
        let (receipt, _) = transfer(&mut setup, vec![(token, 10), (badge, 10)]);
        receipt.expect_commit_success();
        assert_eq!(
            setup.ledger.get_component_balance(setup.recipient, token),
            dec!(10)
        );
        assert_eq!(
            setup.ledger.get_component_balance(setup.recipient, badge),
            dec!(10)
        );
    }
}
