use profile_supporting_types::AnyUnsecurifiedEntity;

use crate::prelude::*;

impl TransactionManifestUnsecurifiedEntityOwnerBadgeIntoBucketPutting
    for TransactionManifest
{
}

pub trait TransactionManifestUnsecurifiedEntityOwnerBadgeIntoBucketPutting {
    /// Produces and puts the owner badge of an Unsecurified Entity by calling
    /// "securify" and put said owner badge in a Bucket and returns the bucket.
    fn put_owner_badge_in_bucket(
        builder: ScryptoTransactionManifestBuilder,
        unsecurified_entity: &AnyUnsecurifiedEntity,
    ) -> (ScryptoTransactionManifestBuilder, ScryptoManifestBucket) {
        let owner_badge_bucket_name = "owner_badge_bucket";
        let owner_badge = match unsecurified_entity.entity.get_entity_kind() {
            CAP26EntityKind::Account => SCRYPTO_ACCOUNT_OWNER_BADGE,
            CAP26EntityKind::Identity => SCRYPTO_IDENTITY_OWNER_BADGE,
        };
        let mut builder = Self::call_securify_for_unsecurified_entity(
            builder,
            unsecurified_entity,
        );
        // Create a bucket out of the entity owner badge.
        builder =
            builder.take_from_worktop(owner_badge, 1, owner_badge_bucket_name);

        let owner_badge_bucket = builder.bucket(owner_badge_bucket_name);

        (builder, owner_badge_bucket)
    }

    /// Calls "securify" for an unsecurified entity which places the owner badge
    /// on the worktop.
    fn call_securify_for_unsecurified_entity(
        builder: ScryptoTransactionManifestBuilder,
        unsecurified_entity: &AnyUnsecurifiedEntity,
    ) -> ScryptoTransactionManifestBuilder {
        let mut builder = builder;

        let is_account = unsecurified_entity.entity.is_account_entity();

        let securify_entity_identifier = if is_account {
            SCRYPTO_ACCOUNT_SECURIFY_IDENT
        } else {
            SCRYPTO_IDENTITY_SECURIFY_IDENT
        };
        builder = builder.call_method(
            unsecurified_entity.address().scrypto(),
            securify_entity_identifier,
            (),
        );

        builder
    }
}
