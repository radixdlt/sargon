use std::ops::AddAssign;

use crate::prelude::*;

use radix_engine::{
    prelude::ToMetadataEntry as ScryptoToMetadataEntry,
    types::ManifestBucket as ScryptoManifestBucket,
};
use transaction::{
    builder::{
        ExistingManifestBucket as ScryptoExistingManifestBucket,
        ManifestNameRegistrar as ScryptoManifestNameRegistrar,
        NewManifestBucket as ScryptoNewManifestBucket,
    },
    prelude::{
        ManifestBuilder as ScryptoManifestBuilder,
        MetadataValue as ScryptoMetadataValue,
    },
};

use radix_engine_common::prelude::NonFungibleLocalId as ScryptoNonFungibleLocalId;

impl TransactionManifest {
    pub fn faucet(
        include_lock_fee_instruction: bool,
        address_of_receiving_account: &AccountAddress,
    ) -> Self {
        let mut builder = ScryptoManifestBuilder::new();

        if include_lock_fee_instruction {
            builder = builder.lock_fee_from_faucet()
        }

        let scrypto_manifest = builder
            .get_free_xrd_from_faucet()
            .try_deposit_entire_worktop_or_abort(
                address_of_receiving_account.scrypto(),
                None,
            )
            .build();

        TransactionManifest::from_scrypto(
            scrypto_manifest,
            address_of_receiving_account.network_id(),
        )
    }

    pub fn marking_account_as_dapp_definition_type(
        account_address: &AccountAddress,
    ) -> Self {
        Self::set_metadata(
            account_address,
            MetadataKey::AccountType,
            MetadataValueStr::DappDefinition,
        )
    }

    pub fn set_owner_keys_hashes(
        address_of_account_or_persona: &AddressOfAccountOrPersona,
        owner_key_hashes: Vec<PublicKeyHash>,
    ) -> Self {
        Self::set_metadata(
            address_of_account_or_persona,
            MetadataKey::OwnerKeys,
            ScryptoMetadataValue::PublicKeyHashArray(
                owner_key_hashes.into_iter().map(|h| h.into()).collect_vec(),
            ),
        )
    }

    fn account_withdraw_non_fungibles(
        builder: ScryptoManifestBuilder,
        owner: &AccountAddress,
        resource_address: &ResourceAddress,
        non_fungible_local_ids: &[NonFungibleLocalId],
    ) -> ScryptoManifestBuilder {
        builder.withdraw_non_fungibles_from_account(
            owner,
            resource_address,
            non_fungible_local_ids
                .iter()
                .cloned()
                .map(Into::<ScryptoNonFungibleLocalId>::into),
        )
    }

    pub fn stake_claims(
        owner: &AccountAddress,
        stake_claims: Vec<StakeClaim>,
    ) -> Self {
        let account_address = owner;
        let network_id = account_address.network_id();
        let xrd_address = &ResourceAddress::xrd_on_network(network_id);

        let mut builder = ScryptoManifestBuilder::new();

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

        let scrypto_manifest = builder.build();

        TransactionManifest::from_scrypto(scrypto_manifest, network_id)
    }
}

#[derive(Default)]
pub(crate) struct BucketFactory {
    next_id: std::cell::Cell<u64>,
}
impl BucketFactory {
    pub(crate) fn next(&self) -> Bucket {
        let next = self.next_id.get();
        let bucket = Bucket {
            name: format!("bucket_{}", next),
        };
        self.next_id.set(next + 1);
        bucket
    }
}

#[derive(Clone)]
pub(crate) struct Bucket {
    pub(crate) name: String,
}
// impl Bucket {
//     pub(crate) fn unique() -> Self {
//         Self {
//             name: id().to_string(),
//         }
//     }
// }
impl AsRef<str> for Bucket {
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}
impl ScryptoNewManifestBucket for &Bucket {
    fn register(self, registrar: &ScryptoManifestNameRegistrar) {
        registrar.register_bucket(registrar.new_bucket(self.name.clone()));
    }
}

impl ScryptoExistingManifestBucket for &Bucket {
    fn resolve(
        self,
        registrar: &ScryptoManifestNameRegistrar,
    ) -> ScryptoManifestBucket {
        registrar.name_lookup().bucket(self)
    }
}

impl TransactionManifest {
    fn set_metadata<A>(
        address: &A,
        key: MetadataKey,
        value: impl ScryptoToMetadataEntry,
    ) -> Self
    where
        A: IntoScryptoAddress,
    {
        let scrypto_manifest = ScryptoManifestBuilder::new()
            .set_metadata(address.scrypto(), key, value)
            .build();

        TransactionManifest::from_scrypto(
            scrypto_manifest,
            address.network_id(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
pub enum MetadataKey {
    #[display("account_type")]
    AccountType,

    #[display("owner_keys")]
    OwnerKeys,
}

impl From<MetadataKey> for String {
    fn from(value: MetadataKey) -> Self {
        value.to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MetadataValue {
    Str(MetadataValueStr),
}
impl MetadataValue {
    pub const DAPP_DEFINITION: Self =
        Self::Str(MetadataValueStr::DappDefinition);
}

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
pub enum MetadataValueStr {
    #[display("dapp definition")]
    DappDefinition,
}

impl ScryptoToMetadataEntry for MetadataValueStr {
    fn to_metadata_entry(self) -> Option<ScryptoMetadataValue> {
        Some(ScryptoMetadataValue::String(self.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use rand::Rng;

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
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
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
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
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
                Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
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
    fn bucket_factory() {
        let sut = BucketFactory::default();
        assert_eq!(sut.next().name, "bucket_0");
        assert_eq!(sut.next().name, "bucket_1");
        assert_eq!(sut.next().name, "bucket_2");
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
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
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
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
            "deposit"
            Bucket("bucket2")
        ;
        CALL_METHOD
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
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
            Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
            "deposit"
            Bucket("bucket4")
        ;
            "#,
        );
    }
}
