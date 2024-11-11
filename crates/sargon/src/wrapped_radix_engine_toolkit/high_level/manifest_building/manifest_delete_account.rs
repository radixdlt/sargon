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
        resource_preferences_to_be_removed: Vec<
            AccountResourcePreferencesResponseItem,
        >,
        authorized_depositors_to_be_removed: Vec<
            AccountAuthorizedDepositorsResponseItem,
        >,
    ) -> Result<Self> {
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

        // Remove all the resource preferences from the account
        for resource_address in resource_preferences_to_be_removed {
            builder = builder.call_method(
                account_address,
                ACCOUNT_REMOVE_RESOURCE_PREFERENCE_IDENT,
                ScryptoAccountRemoveResourcePreferenceInput::from(
                    resource_address,
                ),
            )
        }

        // Remove all the authorized depositros from the account
        for authorized_depositor in authorized_depositors_to_be_removed {
            let input = ScryptoAccountRemoveAuthorizedDepositorInput::try_from(
                authorized_depositor,
            )?;
            builder = builder.call_method(
                account_address,
                ACCOUNT_REMOVE_AUTHORIZED_DEPOSITOR_IDENT,
                input,
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

        Ok(TransactionManifest::sargon_built(
            builder,
            account_address.network_id(),
        ))
    }
}

impl From<AccountResourcePreferencesResponseItem>
    for ScryptoAccountRemoveResourcePreferenceInput
{
    fn from(value: AccountResourcePreferencesResponseItem) -> Self {
        Self {
            resource_address: value.resource_address.into(),
        }
    }
}

impl TryFrom<AccountAuthorizedDepositorsResponseItem>
    for ScryptoAccountRemoveAuthorizedDepositorInput
{
    type Error = CommonError;
    fn try_from(
        value: AccountAuthorizedDepositorsResponseItem,
    ) -> Result<Self> {
        let resource_or_non_fungible = ResourceOrNonFungible::try_from(value)?;
        Ok(resource_or_non_fungible.into())
    }
}

impl TryFrom<AccountAuthorizedDepositorsResponseItem>
    for ResourceOrNonFungible
{
    type Error = CommonError;
    fn try_from(
        value: AccountAuthorizedDepositorsResponseItem,
    ) -> Result<Self> {
        match value {
            AccountAuthorizedDepositorsResponseItem::ResourceBadge {
                resource_address,
            } => Ok(Self::Resource {
                value: resource_address,
            }),
            AccountAuthorizedDepositorsResponseItem::NonFungibleBadge {
                resource_address,
                non_fungible_id,
            } => {
                if let Ok(non_fungible_id) =
                    NonFungibleLocalId::from_str(&non_fungible_id)
                {
                    Ok(Self::NonFungible {
                        value: NonFungibleGlobalId::new_unchecked(
                            resource_address,
                            non_fungible_id,
                        ),
                    })
                } else {
                    return Err(CommonError::InvalidNonFungibleLocalIDString);
                }
            }
        }
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
                vec![
                    AccountResourcePreferencesResponseItem::sample_other(),
                ],
                vec![AccountAuthorizedDepositorsResponseItem::sample_other()],
            ).unwrap(),
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
}
