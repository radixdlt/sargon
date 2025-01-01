use bucket_factory::BucketFactory;

use crate::prelude::*;

pub trait TransactionManifestFaucet: Sized {
    fn faucet(
        include_lock_fee_instruction: bool,
        address_of_receiving_account: &AccountAddress,
    ) -> TransactionManifest;
}

impl TransactionManifestFaucet for TransactionManifest {
    fn faucet(
        include_lock_fee_instruction: bool,
        address_of_receiving_account: &AccountAddress,
    ) -> TransactionManifest {
        let mut builder = ScryptoTransactionManifestBuilder::new();

        if include_lock_fee_instruction {
            builder = builder.lock_fee_from_faucet()
        }

        builder = builder
            .get_free_xrd_from_faucet()
            .try_deposit_entire_worktop_or_abort(
                address_of_receiving_account.scrypto(),
                None,
            );

        TransactionManifest::sargon_built(
            builder,
            address_of_receiving_account.network_id(),
        )
    }
}

pub trait TransactionManifestMetadataSetting: Sized {
    fn set_metadata<A>(
        address: &A,
        key: MetadataKey,
        value: impl ScryptoToMetadataEntry,
    ) -> TransactionManifest
    where
        A: IntoScryptoAddress;

    fn set_metadata_on_builder<A>(
        address: &A,
        key: MetadataKey,
        value: impl ScryptoToMetadataEntry,
        builder: ScryptoTransactionManifestBuilder,
    ) -> ScryptoTransactionManifestBuilder
    where
        A: IntoScryptoAddress,
    {
        builder.set_metadata(address.scrypto(), key, value)
    }

    fn marking_account_as_dapp_definition_type(
        account_address: &AccountAddress,
    ) -> TransactionManifest {
        Self::set_metadata(
            account_address,
            MetadataKey::AccountType,
            MetadataValueStr::DappDefinition,
        )
    }

    fn set_owner_keys_hashes(
        address_of_account_or_persona: &AddressOfAccountOrPersona,
        owner_key_hashes: Vec<PublicKeyHash>,
    ) -> TransactionManifest {
        let builder = Self::set_owner_keys_hashes_on_builder(
            address_of_account_or_persona,
            owner_key_hashes,
            ScryptoTransactionManifestBuilder::new(),
        );
        TransactionManifest::sargon_built(
            builder,
            address_of_account_or_persona.network_id(),
        )
    }

    fn set_owner_keys_hashes_on_builder(
        address_of_account_or_persona: &AddressOfAccountOrPersona,
        owner_key_hashes: Vec<PublicKeyHash>,
        builder: ScryptoTransactionManifestBuilder,
    ) -> ScryptoTransactionManifestBuilder {
        Self::set_metadata_on_builder(
            address_of_account_or_persona,
            MetadataKey::OwnerKeys,
            ScryptoMetadataValue::PublicKeyHashArray(
                owner_key_hashes.into_iter().map(|h| h.into()).collect_vec(),
            ),
            builder,
        )
    }

    fn account_withdraw_non_fungibles(
        builder: ScryptoTransactionManifestBuilder,
        owner: &AccountAddress,
        resource_address: &ResourceAddress,
        non_fungible_local_ids: &[NonFungibleLocalId],
    ) -> ScryptoTransactionManifestBuilder {
        builder.withdraw_non_fungibles_from_account(
            owner,
            resource_address,
            non_fungible_local_ids
                .iter()
                .cloned()
                .map(ScryptoNonFungibleLocalId::from),
        )
    }
}

impl TransactionManifestMetadataSetting for TransactionManifest {
    fn set_metadata<A>(
        address: &A,
        key: MetadataKey,
        value: impl ScryptoToMetadataEntry,
    ) -> TransactionManifest
    where
        A: IntoScryptoAddress,
    {
        let builder = Self::set_metadata_on_builder(
            address,
            key,
            value,
            ScryptoTransactionManifestBuilder::new(),
        );
        TransactionManifest::sargon_built(builder, address.network_id())
    }
}

pub trait TransactionManifestStakeClaim: Sized {
    fn stake_claims(
        owner: &AccountAddress,
        stake_claims: Vec<StakeClaim>,
    ) -> TransactionManifest;
}

impl TransactionManifestStakeClaim for TransactionManifest {
    fn stake_claims(
        owner: &AccountAddress,
        stake_claims: Vec<StakeClaim>,
    ) -> TransactionManifest {
        let account_address = owner;
        let network_id = account_address.network_id();
        if stake_claims
            .clone()
            .into_iter()
            .map(|c| c.validator_address.network_id())
            .any(|n| n != network_id)
        {
            warn!("ValidatorAddress of stake are not on the same network as 'owner' (AccountAddress), the ValidatorAddresses will automatically be switch to the network of owner: {network_id}")
        }
        let xrd_address = &ResourceAddress::xrd_on_network(network_id);

        let mut builder = ScryptoTransactionManifestBuilder::new();

        let bucket_factory = BucketFactory::default();

        for stake_claim in stake_claims.iter() {
            let claim_address = &stake_claim.resource_address;
            let validator_address = &stake_claim.validator_address;

            // Withdraw non fungibles from account
            builder = Self::account_withdraw_non_fungibles(
                builder,
                account_address,
                claim_address,
                &stake_claim.ids,
            );

            let bucket = &bucket_factory.next();
            builder = builder.take_all_from_worktop(claim_address, bucket);

            // Claim XRDs for the given nft ids.
            builder = builder.claim_xrd(validator_address, bucket);

            // Deposit the claimed amount
            let xrd_bucket = &bucket_factory.next();

            builder = builder.take_from_worktop(
                xrd_address,
                stake_claim.amount,
                xrd_bucket,
            );

            builder = builder.deposit(account_address, xrd_bucket)
        }

        TransactionManifest::sargon_built(builder, network_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn manifest_for_faucet() {
        manifest_eq(
            SUT::faucet(true, &AccountAddress::sample_mainnet()),
            r#"
            CALL_METHOD
                Address("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet")
                "lock_fee"
                Decimal("5000")
            ;
            CALL_METHOD
                Address("component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet")
                "free"
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "try_deposit_batch_or_abort"
                Expression("ENTIRE_WORKTOP")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn manifest_for_set_account_to_dapp_definition_address() {
        manifest_eq(
            SUT::marking_account_as_dapp_definition_type(
                &AccountAddress::sample_mainnet(),
            ),
            r#"
            SET_METADATA
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "account_type"
                Enum<0u8>(
                    "dapp definition"
                )
            ;
            "#,
        );
    }

    #[test]
    fn manifest_for_owner_keys() {
        manifest_eq(
            SUT::set_owner_keys_hashes(
                &AccountAddress::sample_mainnet().into(),
                vec![
                    PublicKeyHash::hash(Ed25519PublicKey::sample_alice()),
                    PublicKeyHash::hash(Secp256k1PublicKey::sample_bob()),
                ],
            ),
            r#"
            SET_METADATA
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
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
    fn stake_claims() {
        let stake_claims =
            vec![StakeClaim::sample(), StakeClaim::sample_other()];
        let manifest =
            SUT::stake_claims(&AccountAddress::sample_mainnet(), stake_claims);
        manifest_eq(
            manifest,
            r#"
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw_non_fungibles"
            Address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("{deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead-deaddeaddeaddead}"),
                NonFungibleLocalId("<foobar>")
            )
        ;
        TAKE_ALL_FROM_WORKTOP
            Address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa")
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0")
            "claim_xrd"
            Bucket("bucket1")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("1337")
            Bucket("bucket2")
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "deposit"
            Bucket("bucket2")
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "withdraw_non_fungibles"
            Address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd")
            Array<NonFungibleLocalId>(
                NonFungibleLocalId("<foobar>")
            )
        ;
        TAKE_ALL_FROM_WORKTOP
            Address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd")
            Bucket("bucket3")
        ;
        CALL_METHOD
            Address("validator_rdx1sw5rrhkxs65kl9xcxu7t9yu3k8ptscjwamum4phclk297j6r28g8kd")
            "claim_xrd"
            Bucket("bucket3")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("237")
            Bucket("bucket4")
        ;
        CALL_METHOD
            Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
            "deposit"
            Bucket("bucket4")
        ;
            "#,
        );
    }

    #[test]
    fn manifest_builder_switches_network_of_validators_to_that_of_account_address(
    ) {
        let stake_claim = StakeClaim::sample();
        let validator_address = stake_claim.validator_address;
        let owner_address = AccountAddress::sample_stokenet();

        // The network of Validator address and owner address are different!
        // However, when Scrypto builds manifest, it is network agnostic!
        // and we have setup our builder to always use the network of
        // the "owner" (AccountAddress)!
        assert_ne!(validator_address.network_id(), owner_address.network_id());

        let stake_claims = vec![stake_claim];
        let manifest = SUT::stake_claims(&owner_address, stake_claims);

        // This might be surprising! The built manifest does NOT contain
        // the specified validator address, it is because the network id
        // (of the ValidatorAddress) has changed from mainnet to stokenet,
        // since the account address is on stokenet!
        assert_eq!(
            manifest
                .manifest_string()
                .contains(&validator_address.address()),
            false
        );

        // However, if we map the validator_address -> same network as owner
        // THAT address should indeed be present!
        assert_eq!(
            manifest.manifest_string().contains(
                &validator_address
                    .map_to_network(owner_address.network_id())
                    .address()
            ),
            true,
        );
    }
}
