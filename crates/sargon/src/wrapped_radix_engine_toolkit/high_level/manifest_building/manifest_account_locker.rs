use crate::prelude::*;
use radix_common::prelude::ManifestExpression;
use radix_engine_interface::blueprints::locker::{
    ACCOUNT_LOCKER_CLAIM_IDENT, ACCOUNT_LOCKER_CLAIM_NON_FUNGIBLES_IDENT,
};
use std::cmp::min;

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
        let mut builder = ScryptoTransactionManifestBuilder::new();
        let bucket_factory = BucketFactory::default();
        let claimant_arg: ScryptoComponentAddress = (*claimant).into();
        let claimable_resources =
            Self::build_claimable_batch(claimable_resources, 50);

        for claimable in claimable_resources.iter() {
            let (resource_arg, amount_arg) = match claimable.clone() {
                AccountLockerClaimableResource::Fungible {
                    resource_address,
                    amount,
                } => {
                    let resource_arg: ScryptoResourceAddress =
                        resource_address.into();
                    let amount_arg: ScryptoDecimal192 = amount.into();
                    (resource_arg, amount_arg)
                }
                AccountLockerClaimableResource::NonFungible {
                    resource_address,
                    number_of_items: count,
                } => {
                    let resource_arg: ScryptoResourceAddress =
                        resource_address.into();
                    let amount_arg: ScryptoDecimal192 = count.into();
                    (resource_arg, amount_arg)
                }
            };

            builder = builder.call_method(
                locker_address,
                ACCOUNT_LOCKER_CLAIM_IDENT,
                (claimant_arg, resource_arg, amount_arg),
            );

            let bucket = &bucket_factory.next();

            builder =
                builder.take_from_worktop(resource_arg, amount_arg, bucket);
            builder = builder.deposit(claimant_arg, bucket);
        }

        TransactionManifest::sargon_built(builder, claimant.network_id())
    }

    fn build_claimable_batch(
        claimable_resources: Vec<AccountLockerClaimableResource>,
        max_size: u64,
    ) -> IndexSet<AccountLockerClaimableResource> {
        let mut number_of_items_to_add = max_size;
        let mut result = IndexSet::<AccountLockerClaimableResource>::new();

        for claimable_resource in claimable_resources {
            let updated_resource = claimable_resource
                .coerce_number_of_items_at_most(number_of_items_to_add);
            result.insert(updated_resource.clone());
            number_of_items_to_add -= updated_resource.number_of_items();

            // can never be negative thanks to clamping performed in `coerce_number_of_items_at_most`
            if number_of_items_to_add == 0 {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radix_common::prelude::ResourceAddress;

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
                    number_of_items: 10,
                },
                AccountLockerClaimableResource::Fungible {
                    resource_address: "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd".into(),
                    amount: 1500.into(),
                },
                AccountLockerClaimableResource::NonFungible {
                    resource_address: "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd".into(),
                    number_of_items: 1,
                },
            ],
        );

        manifest_eq(manifest, expected_manifest)
    }

    #[test]
    fn claim_limited_to_required_batch_size() {
        let expected_manifest = include_str!(concat!(
            env!("FIXTURES_TX"),
            "account_locker_claim_max_nft_items.rtm"
        ));
        let manifest = SUT::account_locker_claim(
            &"locker_rdx1drn4q2zk6dvljehytnhfah330xk7emfznv59rqlps5ayy52d7xkzzz".into(),
            &"account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr".into(),
            vec![
                AccountLockerClaimableResource::NonFungible {
                    resource_address: "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd".into(),
                    number_of_items: 100,
                },
                AccountLockerClaimableResource::Fungible {
                    resource_address: "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd".into(),
                    amount: Decimal192::one(),
                }
            ],
        );

        manifest_eq(manifest, expected_manifest)
    }
}
