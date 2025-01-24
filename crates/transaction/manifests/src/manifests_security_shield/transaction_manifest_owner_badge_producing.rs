use std::borrow::Borrow;

use radix_engine_interface::blueprints::access_controller::ACCESS_CONTROLLER_CREATE_PROOF_IDENT;

use crate::prelude::*;

impl TransactionManifestOwnerBadgeProducing for TransactionManifest {}

pub trait TransactionManifestOwnerBadgeProducing {
    /// Produces and puts the owner badge in a Bucket and returns the bucket.
    fn put_owner_badge_in_bucket(
        builder: ScryptoTransactionManifestBuilder,
        owner: impl Borrow<AccountOrPersona>,
    ) -> (ScryptoTransactionManifestBuilder, ScryptoManifestBucket) {
        let owner = owner.borrow();
        let is_account = owner.is_account_entity();
        let owner_badge_bucket_name = "owner_badge_bucket";
        let owner_badge = if is_account {
            SCRYPTO_ACCOUNT_OWNER_BADGE
        } else {
            SCRYPTO_IDENTITY_OWNER_BADGE
        };
        let mut builder = Self::produce_owner_badge(builder, owner);
        // Create a bucket out of the entity owner badge.
        builder =
            builder.take_from_worktop(owner_badge, 1, owner_badge_bucket_name);

        let owner_badge_bucket = builder.bucket(owner_badge_bucket_name);

        (builder, owner_badge_bucket)
    }

    /// Produce the owner badge
    /// TODO: Ask Omar if this is correct for Securified entityes.
    fn produce_owner_badge(
        builder: ScryptoTransactionManifestBuilder,
        owner: impl Borrow<AccountOrPersona>,
    ) -> ScryptoTransactionManifestBuilder {
        let mut builder = builder;
        let owner = owner.borrow();

        let is_account = owner.is_account_entity();

        match owner.security_state() {
            EntitySecurityState::Securified { value } => {
                builder = builder.call_method(
                    value.access_controller_address.scrypto(),
                    ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                    (),
                );
            }
            EntitySecurityState::Unsecured { value: _ } => {
                let securify_entity_identifier = if is_account {
                    SCRYPTO_ACCOUNT_SECURIFY_IDENT
                } else {
                    SCRYPTO_IDENTITY_SECURIFY_IDENT
                };
                builder = builder.call_method(
                    owner.address().scrypto(),
                    securify_entity_identifier,
                    (),
                );
            }
        }

        builder
    }
}
