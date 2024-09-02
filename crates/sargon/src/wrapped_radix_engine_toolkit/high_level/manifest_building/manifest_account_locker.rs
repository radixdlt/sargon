use crate::prelude::*;
use radix_common::prelude::ManifestExpression;
use radix_engine_interface::blueprints::locker::{
    ACCOUNT_LOCKER_CLAIM_IDENT, ACCOUNT_LOCKER_CLAIM_NON_FUNGIBLES_IDENT,
};

impl From<AccountAddress> for ScryptoComponentAddress {
    fn from(value: AccountAddress) -> ScryptoComponentAddress {
        ScryptoComponentAddress::new_or_panic(value.node_id().0)
    }
}

impl TransactionManifest {
    pub fn account_locker_claim(
        locker_address: &LockerAddress,
        claimant: &AccountAddress,
        claimable_resources: Vec<AccountLockerClaimableResource>,
    ) -> Self {
        let mut builder = ScryptoManifestBuilder::new();
        let bucket_factory = BucketFactory::default();
        let claimant_arg: ScryptoComponentAddress = (*claimant).into();
        let claimable_resources =
            Self::build_claimable_batch(claimable_resources, 50);

        for claimable in claimable_resources.iter() {
            match claimable.clone() {
                AccountLockerClaimableResource::Fungible {
                    resource_address,
                    amount,
                } => {
                    let resource_arg: ScryptoResourceAddress =
                        resource_address.into();
                    let amount_arg: ScryptoDecimal192 = amount.into();

                    builder = builder.call_method(
                        locker_address,
                        ACCOUNT_LOCKER_CLAIM_IDENT,
                        (claimant_arg, resource_arg, amount_arg),
                    );

                    let bucket = &bucket_factory.next();

                    builder = builder.take_from_worktop(
                        resource_arg,
                        amount_arg,
                        bucket,
                    );
                    builder = builder.deposit(claimant_arg, bucket);
                }
                AccountLockerClaimableResource::NonFungible {
                    resource_address,
                    ids,
                } => {
                    let resource_arg: ScryptoResourceAddress =
                        resource_address.into();
                    let ids_arg: Vec<ScryptoNonFungibleLocalId> =
                        ids.iter().map(|id| id.clone().into()).collect();

                    builder = builder.call_method(
                        locker_address,
                        ACCOUNT_LOCKER_CLAIM_NON_FUNGIBLES_IDENT,
                        (claimant_arg, resource_arg, ids_arg.clone()),
                    );

                    let bucket = &bucket_factory.next();

                    builder = builder.take_non_fungibles_from_worktop(
                        resource_arg,
                        ids_arg,
                        bucket,
                    );

                    builder = builder.deposit(claimant_arg, bucket);
                }
            }
        }

        TransactionManifest::sargon_built(builder, claimant.network_id())
    }

    fn build_claimable_batch(
        claimable_resources: Vec<AccountLockerClaimableResource>,
        size: usize,
    ) -> IndexSet<AccountLockerClaimableResource> {
        let mut current_batch_size = 0;
        claimable_resources
            .into_iter()
            .take_while(|claimable| {
                current_batch_size += claimable.resource_count();
                current_batch_size <= size
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_transactions::manifest::ast::ValueKind::ResourceAddress;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn empty() {
        let manifest = SUT::account_locker_claim(
            &"locker_rdx1drn4q2zk6dvljehytnhfah330xk7emfznv59rqlps5ayy52d7xkzzz".into(),
            &"account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr".into(),
            vec![],
        );
        manifest_eq(manifest, "")
    }

    #[test]
    fn claim_fungibles_and_non_fungibles() {
        let expected_manifest = include_str!(concat!(
            env!("FIXTURES_TX"),
            "account_locker_claim.rtm"
        ));

        let manifest = SUT::account_locker_claim(
            &"locker_rdx1drn4q2zk6dvljehytnhfah330xk7emfznv59rqlps5ayy52d7xkzzz".into(),
            &"account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr".into(),
            vec![
                AccountLockerClaimableResource::Fungible {
                    resource_address: "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j".into(),
                    amount: 123.into(),
                },
                AccountLockerClaimableResource::NonFungible {
                    resource_address: "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa".into(),
                    ids: vec![NonFungibleLocalId::integer(1)],
                },
                AccountLockerClaimableResource::Fungible {
                    resource_address: "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd".into(),
                    amount: 1500.into(),
                },
                AccountLockerClaimableResource::NonFungible {
                    resource_address: "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd".into(),
                    ids: vec![NonFungibleLocalId::string("foobar").unwrap()],
                },
            ],
        );

        manifest_eq(manifest, expected_manifest)
    }

    #[test]
    fn claim_limited_to_required_batch_size() {
        let manifest = SUT::account_locker_claim(
            &"locker_rdx1drn4q2zk6dvljehytnhfah330xk7emfznv59rqlps5ayy52d7xkzzz".into(),
            &"account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr".into(),
            (0..100).map(|i| AccountLockerClaimableResource::NonFungible {
                resource_address: "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd".into(),
                ids: vec![NonFungibleLocalId::integer(i)]
            }).collect(),
        );

        assert_eq!(manifest.instructions().len(), 150)
    }
}
