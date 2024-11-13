use crate::prelude::*;
use radix_engine_interface::blueprints::account::{
    AccountRemoveAuthorizedDepositorInput as ScryptoAccountRemoveAuthorizedDepositorInput,
    AccountRemoveResourcePreferenceInput as ScryptoAccountRemoveResourcePreferenceInput,
    AccountSetResourcePreferenceInput as ScryptoAccountSetResourcePreferenceInput,
    ResourcePreference as ScryptoResourcePreference,
    ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
    ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT, ACCOUNT_SECURIFY_IDENT,
    ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
};

impl TransactionManifest {
    pub fn delete_account(
        account_address: &AccountAddress,
        account_transfers: impl Into<Option<DeleteAccountTransfers>>,
        resource_preferences_to_be_removed: Vec<
            ScryptoAccountRemoveResourcePreferenceInput,
        >,
        authorized_depositors_to_be_removed: Vec<
            ScryptoAccountRemoveAuthorizedDepositorInput,
        >,
    ) -> Self {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        let bucket_factory = BucketFactory::default();

        // Transfer all the resources to the recipient address.
        if let Some(transfers) = account_transfers.into() {
            let recipient_address = &transfers.recipient;
            // Transfer each fungible
            for transfer in transfers.fungibles {
                let resource_address = transfer.resource_address.scrypto();
                let amount: ScryptoDecimal192 = transfer.amount.into();

                builder = builder.withdraw_from_account(
                    account_address,
                    resource_address,
                    amount,
                );

                let bucket = &bucket_factory.next();
                builder =
                    builder.take_from_worktop(resource_address, amount, bucket);

                builder = builder.try_deposit_or_abort(
                    recipient_address,
                    None,
                    bucket,
                );
            }

            // Transfer each non-fungible
            for transfer in transfers.non_fungibles {
                let resource_address = transfer.resource_address.scrypto();
                let amount: ScryptoDecimal192 = transfer.amount.into();

                // TODO: Confirm the following logic is correct for withdrawing non-fungibles by amount

                builder = builder.withdraw_from_account(
                    account_address,
                    resource_address,
                    amount,
                );

                let bucket = &bucket_factory.next();
                builder = builder.take_from_worktop(resource_address, amount, bucket);

                builder = builder.try_deposit_or_abort(
                    recipient_address,
                    None,
                    bucket,
                );
            }
        }

        // Securify the account which will return an account owner badge onto the worktop.
        builder =
            builder.call_method(account_address, ACCOUNT_SECURIFY_IDENT, ());

        // Create a bucket out of the account owner badge.
        let owner_badge = SCRYPTO_ACCOUNT_OWNER_BADGE;
        let owner_badge_bucket = &bucket_factory.next();
        builder =
            builder.take_all_from_worktop(owner_badge, owner_badge_bucket);

        // Create a proof from the account owner badge
        let owner_badge_proof = "owner_badge_proof";
        builder = builder.create_proof_from_bucket_of_all(
            owner_badge_bucket,
            owner_badge_proof,
        );

        // Push the proof to the auth zone
        builder = builder.push_to_auth_zone(owner_badge_proof);

        // Remove all the resource preferences from the account
        for resource_address in resource_preferences_to_be_removed {
            builder = builder.call_method(
                account_address,
                ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
                resource_address,
            )
        }

        // Remove all the authorized depositors from the account
        for authorized_depositor in authorized_depositors_to_be_removed {
            builder = builder.call_method(
                account_address,
                ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
                authorized_depositor,
            )
        }

        // Make the deposit of the account owner badge allowed into the account
        let asset_exception = AssetException::new(
            ResourceAddress::new(owner_badge, account_address.network_id())
                .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        builder = builder.call_method(
            account_address,
            ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
            ScryptoAccountSetResourcePreferenceInput::from(asset_exception),
        );

        // We've changed the account deposit rules, so now we can drop all the proofs in the auth zone (which
        // is pretty much a single proof of the account owner badge).
        builder = builder.drop_auth_zone_proofs();

        // We deposit the account's owner badge into it which locks it forever.
        builder = builder.try_deposit_or_abort(
            account_address,
            None,
            owner_badge_bucket,
        );

        TransactionManifest::sargon_built(builder, account_address.network_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn manifest_without_transfers() {
        manifest_eq(
            SUT::delete_account(
                &"account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg".into(),
                None,
                vec![
                    AccountResourcePreference::sample_other().into(),
                ],
                vec![
                    AccountAuthorizedDepositor::sample_other().try_into().unwrap(),
                ],
            ),
            r#"
CALL_METHOD
    Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
    "securify"
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1nfxxxxxxxxxxaccwnrxxxxxxxxx006664022062xxxxxxxxx4vczzk")
    Bucket("bucket1")
;
CREATE_PROOF_FROM_BUCKET_OF_ALL
    Bucket("bucket1")
    Proof("proof1")
;
PUSH_TO_AUTHZONE
    Proof("proof1")
;
CALL_METHOD
    Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
    "remove_resource_preference"
    Address("resource_tdx_2_1tk30vj4ene95e3vhymtf2p35fzl29rv4us36capu2rz0vretw9gzr3")
;
CALL_METHOD
    Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
    "remove_authorized_depositor"
    Enum<0u8>(
        NonFungibleGlobalId("resource_tdx_2_1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9frs528x:#1#")
    )
;
CALL_METHOD
    Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
    "set_resource_preference"
    Address("resource_tdx_2_1nfxxxxxxxxxxaccwnrxxxxxxxxx006664022062xxxxxxxxx4vczzk")
    Enum<0u8>()
;
DROP_AUTH_ZONE_PROOFS;
CALL_METHOD
    Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
    "try_deposit_or_abort"
    Bucket("bucket1")
    Enum<0u8>()
;
            "#,
        );
    }

    #[test]
    fn manifest_with_transfers() {
        let transfers = DeleteAccountTransfers::new(
            AccountAddress::sample_other(),
            vec![FungibleResourcesCollectionItemGloballyAggregated::sample()],
            vec![
                NonFungibleResourcesCollectionItemGloballyAggregated::sample(),
            ],
        );
        let manifest = SUT::delete_account(
                &"account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg".into(),
                transfers,
                vec![
                    AccountResourcePreference::sample_other().into(),
                ],
                vec![
                    AccountAuthorizedDepositor::sample_other().try_into().unwrap(),
                ],
            );

        println!("{}", manifest.to_string());
    }
}