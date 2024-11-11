use crate::prelude::*;
use radix_engine_interface::blueprints::account::{
    AccountSetResourcePreferenceInput as ScryptoAccountSetResourcePreferenceInput,
    ResourcePreference as ScryptoResourcePreference, ACCOUNT_SECURIFY_IDENT,
    ACCOUNT_SET_RESOURCE_PREFERENCE_IDENT,
};

impl TransactionManifest {
    pub fn delete_account(account_address: &AccountAddress) -> Self {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        let bucket_factory = BucketFactory::default();

        // We securify the account which will return an account owner badge onto the worktop.
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

        // Make the deposit of the account owner badge allowed into the account just in case somebody
        // had changed the account deposit rules in the past.
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
    fn manifest() {
        manifest_eq(
            SUT::delete_account(
                &"account_tdx_2_16yll6clntk9za0wvrw0nat848uazduyqy635m8ms77md99q7yf9fzg".into(),
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
}
