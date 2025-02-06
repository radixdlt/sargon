use crate::prelude::*;
use bucket_factory::BucketFactory;
use radix_engine_interface::blueprints::account::{
    AccountRemoveAuthorizedDepositorInput as ScryptoAccountRemoveAuthorizedDepositorInput,
    AccountRemoveResourcePreferenceInput as ScryptoAccountRemoveResourcePreferenceInput,
    AccountSetResourcePreferenceInput as ScryptoAccountSetResourcePreferenceInput,
    ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
    ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT, ACCOUNT_SECURIFY_IDENT,
    ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
    ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
};

pub trait ManifestForAccountDeletion {
    fn delete_account(
        account_address: &AccountAddress,
        account_transfers: impl Into<Option<DeleteAccountTransfers>>,
        resource_preferences_to_be_removed: Vec<
            ScryptoAccountRemoveResourcePreferenceInput,
        >,
        authorized_depositors_to_be_removed: Vec<
            ScryptoAccountRemoveAuthorizedDepositorInput,
        >,
    ) -> Self;
}

impl ManifestForAccountDeletion for TransactionManifest {
    fn delete_account(
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
        if let Some(account_transfers) = account_transfers.into() {
            let recipient_address = &account_transfers.recipient;

            for transfer in account_transfers.transfers {
                // Withdraw from account
                builder = builder.withdraw_from_account(
                    account_address,
                    transfer.resource_address,
                    transfer.amount,
                );

                let bucket = &bucket_factory.next();
                // Take from worktop
                builder = builder.take_from_worktop(
                    transfer.resource_address,
                    transfer.amount,
                    bucket,
                );

                // Try deposit or abort
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
            ResourceAddress::new_from_node_id(
                owner_badge,
                account_address.network_id(),
            )
            .unwrap(),
            DepositAddressExceptionRule::Allow,
        );
        builder = builder.call_method(
            account_address,
            ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
            ScryptoAccountSetResourcePreferenceInput::from(asset_exception),
        );

        // Set the default deposit rule to reject all deposits
        builder = builder.call_method(
            account_address,
            ACCOUNT_SET_DEFAULT_DEPOSIT_RULE_IDENT,
            (ScryptoDefaultDepositRule::Reject,),
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
                &"account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg".parse::<AccountAddress>().unwrap(),
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
PUSH_TO_AUTH_ZONE
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
CALL_METHOD
Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
"set_default_deposit_rule"
Enum<1u8>()
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
            vec![
                DeleteAccountTransfer::sample(),
                DeleteAccountTransfer::sample_other(),
            ],
            vec![],
        );
        let manifest = SUT::delete_account(
                &"account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg".parse::<AccountAddress>().unwrap(),
                transfers,
                vec![
                    AccountResourcePreference::sample_other().into(),
                ],
                vec![
                    AccountAuthorizedDepositor::sample_other().try_into().unwrap(),
                ],
            );

        manifest_eq(
            manifest,
            r#"
CALL_METHOD
    Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
    "withdraw"
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("123.456")
;
TAKE_FROM_WORKTOP
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("123.456")
    Bucket("bucket1")
;
CALL_METHOD
    Address("account_tdx_2_12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlwkwcf0")
    "try_deposit_or_abort"
    Bucket("bucket1")
    Enum<0u8>()
;
CALL_METHOD
    Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
    "withdraw"
    Address("resource_tdx_2_1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9frs528x")
    Decimal("5")
;
TAKE_FROM_WORKTOP
    Address("resource_tdx_2_1ng6aanl0nw98dgqxtja3mx4kpa8rzwhyt4q22sy9uul0vf9frs528x")
    Decimal("5")
    Bucket("bucket2")
;
CALL_METHOD
    Address("account_tdx_2_12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlwkwcf0")
    "try_deposit_or_abort"
    Bucket("bucket2")
    Enum<0u8>()
;
CALL_METHOD
    Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
    "securify"
;
TAKE_ALL_FROM_WORKTOP
    Address("resource_tdx_2_1nfxxxxxxxxxxaccwnrxxxxxxxxx006664022062xxxxxxxxx4vczzk")
    Bucket("bucket3")
;
CREATE_PROOF_FROM_BUCKET_OF_ALL
    Bucket("bucket3")
    Proof("proof1")
;
PUSH_TO_AUTH_ZONE
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
CALL_METHOD
Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
"set_default_deposit_rule"
Enum<1u8>()
;
DROP_AUTH_ZONE_PROOFS;
CALL_METHOD
    Address("account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg")
    "try_deposit_or_abort"
    Bucket("bucket3")
    Enum<0u8>()
;
            "#,
        )
    }
}
