use crate::prelude::*;
use radix_engine_interface::blueprints::access_controller::ACCESS_CONTROLLER_CREATE_PROOF_IDENT;

pub trait ModifyingManifest<M, I>
where
    M: IntoManifest<I> + Clone,
    I: IntoInstruction + Clone,
    Self: Sized,
{
    fn modifying_manifest(&self) -> M;
}

pub struct InstructionPosition(pub u64);

pub trait AddingGuaranteesModifyingManifest<M, I>:
    ModifyingManifest<M, I>
where
    M: IntoManifest<I> + Clone,
    I: IntoInstruction + Clone,
{
    fn insert_guarantee_assertion_at_position(
        self,
        position: InstructionPosition,
        guarantee: TransactionGuarantee,
    ) -> Result<Self>;

    // Modifies `manifest` by inserting transaction "guarantees", which is the wallet
    // term for `assert_worktop_contains`.
    fn modify_add_guarantees<G>(self, guarantees: G) -> Result<M>
    where
        G: IntoIterator<Item = TransactionGuarantee>,
    {
        let guarantees = guarantees.into_iter().collect_vec();
        if guarantees.is_empty() {
            return Ok(self.modifying_manifest());
        };

        let instructions = self.modifying_manifest().instructions();
        let instruction_count = instructions.len() as u64;

        if let Some(oob) = guarantees
            .clone()
            .into_iter()
            .find(|g| g.instruction_index >= instruction_count)
        {
            return Err(CommonError::TXGuaranteeIndexOutOfBounds {
                index: oob.instruction_index,
                count: instruction_count,
            });
        }

        // Will be increased with each added guarantee to account for the
        // difference in indexes from the initial manifest.
        let mut offset = 0;
        let mut modifiable_manifest = self;

        for guarantee in guarantees {
            modifiable_manifest = modifiable_manifest
                .insert_guarantee_assertion_at_position(
                    InstructionPosition(guarantee.instruction_index + offset),
                    guarantee,
                )?;

            offset.add_assign(1);
        }

        Ok(modifiable_manifest.modifying_manifest())
    }
}

/// Used by development, in production we SHOULD use the fee given by analyzing
/// the manifest.
fn default_fee() -> Decimal192 {
    Decimal192::from(25)
}

pub struct LockFeeData {
    pub fee_payer_address: AccountAddress,
    pub access_controller_address: Option<AccessControllerAddress>,
    fee: Option<Decimal192>,
}

impl LockFeeData {
    pub fn new_with_unsecurified_fee_payer(
        fee_payer_address: AccountAddress,
        fee: Decimal192,
    ) -> Self {
        Self {
            fee_payer_address,
            access_controller_address: None,
            fee: Some(fee),
        }
    }

    pub fn new_with_securified_fee_payer(
        fee_payer_address: AccountAddress,
        access_controller_address: AccessControllerAddress,
        fee: Decimal192,
    ) -> Self {
        Self {
            fee_payer_address,
            access_controller_address: Some(access_controller_address),
            fee: Some(fee),
        }
    }

    #[cfg(test)]
    pub fn new_with_unsecurified_fee_payer_no_fee(
        fee_payer_address: AccountAddress,
    ) -> Self {
        Self {
            fee_payer_address,
            access_controller_address: None,
            fee: None,
        }
    }

    pub fn fee(&self) -> Decimal192 {
        self.fee.unwrap_or(default_fee())
    }
}

pub trait AddingLockFeeAndProofsModifyingManifest<M, B, I>:
    ModifyingManifest<M, I>
where
    M: IntoManifest<I> + Clone,
    I: IntoInstruction + Clone,
    B: IntoManifestBuilder<M, I>,
{
    fn modify_add_proofs(
        &self,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
    ) -> Result<M> {
        self.modify_add_lock_fee_and_proofs(
            None,
            entities_with_access_controllers,
        )
    }

    fn modify_add_lock_fee(&self, lock_fee_data: LockFeeData) -> Result<M> {
        self.modify_add_lock_fee_and_proofs(
            Some(lock_fee_data),
            IndexMap::new(),
        )
    }

    fn modify_add_lock_fee_and_proofs(
        &self,
        lock_fee_data: impl Into<Option<LockFeeData>>,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
    ) -> Result<M> {
        let mut access_controllers = entities_with_access_controllers
            .iter()
            .map(|(_, ac)| *ac)
            .collect::<IndexSet<_>>();

        let mut builder = B::new_with_instructions([], Blobs::new(vec![]));

        if let Some(lock_fee_data) = lock_fee_data.into() {
            if let Some(access_controller_of_fee_payer) =
                lock_fee_data.access_controller_address
            {
                access_controllers
                    .shift_remove(&access_controller_of_fee_payer);

                // Add proof for lock fee payer, who happens to be securified.
                builder = builder.call_method(
                    ScryptoGlobalAddress::from(access_controller_of_fee_payer),
                    ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                    (),
                );
            }

            // Add lock fee
            builder = builder
                .lock_fee(&lock_fee_data.fee_payer_address, lock_fee_data.fee())
        }

        // Put the remaining proofs of the Access Controller addresses
        for access_controller in access_controllers {
            builder = builder.call_method(
                ScryptoGlobalAddress::from(access_controller),
                ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                (),
            );
        }

        let modifying_manifest = self.modifying_manifest();

        builder =
            builder.extend_builder_with_manifest(modifying_manifest.clone());
        builder.build(modifying_manifest.network_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_add_lock_fee_without_fee_amount() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();

        manifest_eq(
            manifest
                .modify_add_lock_fee(
                    LockFeeData::new_with_unsecurified_fee_payer_no_fee(
                        fee_payer,
                    ),
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("25")
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_add_lock_fee_with_fee_amount() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();

        manifest_eq(
            manifest
                .modify_add_lock_fee(
                    LockFeeData::new_with_unsecurified_fee_payer(
                        fee_payer,
                        Decimal192::five(),
                    ),
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_add_lock_fee_with_fee_amount_and_access_controller() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();

        manifest_eq(
            manifest
                .modify_add_lock_fee(
                    LockFeeData::new_with_securified_fee_payer(
                        fee_payer,
                        AccessControllerAddress::sample_mainnet(),
                        Decimal192::five(),
                    ),
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_add_lock_fee_with_fee_amount_and_proofs() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();
        let proofs = IndexMap::just((
            AddressOfAccountOrPersona::sample_mainnet_other(),
            AccessControllerAddress::sample_mainnet(),
        ));

        manifest_eq(
            manifest
                .modify_add_lock_fee_and_proofs(
                    LockFeeData::new_with_unsecurified_fee_payer(
                        fee_payer,
                        Decimal192::five(),
                    ),
                    proofs,
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_add_lock_fee_with_fee_amount_and_proofs_fee_payer_in_proofs()
    {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();
        let proofs = IndexMap::from([
            (
                AddressOfAccountOrPersona::sample_mainnet_other(),
                AccessControllerAddress::sample_mainnet(),
            ),
            (
                AddressOfAccountOrPersona::from(fee_payer),
                AccessControllerAddress::sample_mainnet_other(),
            ),
        ]);

        manifest_eq(
            manifest
                .modify_add_lock_fee_and_proofs(
                    LockFeeData::new_with_securified_fee_payer(
                        fee_payer,
                        AccessControllerAddress::sample_mainnet(),
                        Decimal192::five(),
                    ),
                    proofs,
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn add_guarantees_divisibility_rounding() {
        let instructions_string = r#"
CALL_METHOD
    Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
    "lock_fee"
    Decimal("0.61")
;
CALL_METHOD
    Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
    "withdraw"
    Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
    Decimal("0.12344")
;
TAKE_FROM_WORKTOP
    Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
    Decimal("0.12344")
    Bucket("bucket1")
;
CALL_METHOD
    Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
    "try_deposit_or_abort"
    Bucket("bucket1")
    Enum<0u8>()
;
        "#;
        let index = 2;
        let resource = ResourceAddress::sample_mainnet_candy();
        let added_guaranteed_amount: Decimal = "0.12344".parse().unwrap();
        let percentage: Decimal = "0.95".parse().unwrap();
        let divisibility = 4;
        let rounded_guaranteed_amount: Decimal = "0.1234".parse().unwrap();
        assert_eq!(
            added_guaranteed_amount.clone().round(divisibility),
            rounded_guaranteed_amount.clone()
        );
        let manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();

        manifest_eq(
            manifest
                .modify_add_guarantees([TransactionGuarantee::new(
                    added_guaranteed_amount,
                    percentage,
                    index,
                    resource,
                    divisibility,
                )])
                .unwrap(),
            r#"
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("0.61")
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "withdraw"
                Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
                Decimal("0.12344")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
                Decimal("0.1234")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
                Decimal("0.12344")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_manifest_add_guarantees_to_manifest() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();

        manifest_eq(
            manifest
                .modify_add_guarantees([TransactionGuarantee::new(
                    1337,
                    0,
                    1,
                    ResourceAddress::sample(),
                    10,
                )])
                .unwrap(),
            r#"
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_manifest_add_many_guarantees() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();

        manifest_eq(
            manifest
                .modify_add_guarantees([
                    TransactionGuarantee::new(
                        1337,
                        0,
                        1,
                        ResourceAddress::sample(),
                        10,
                    ),
                    TransactionGuarantee::new(
                        1338,
                        0,
                        1,
                        ResourceAddress::sample(),
                        10,
                    ),
                    TransactionGuarantee::new(
                        1339,
                        0,
                        1,
                        ResourceAddress::sample(),
                        10,
                    ),
                    TransactionGuarantee::new(
                        1340,
                        0,
                        1,
                        ResourceAddress::sample(),
                        10,
                    ),
                    TransactionGuarantee::new(
                        1341,
                        0,
                        1,
                        ResourceAddress::sample(),
                        10,
                    ),
                    TransactionGuarantee::new(
                        1342,
                        0,
                        1,
                        ResourceAddress::sample(),
                        10,
                    ),
                ])
                .unwrap(),
            r#"
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1338")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1339")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1340")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1341")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1342")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_manifest_add_guarantees_unchanged_if_no_guarantees() {
        let manifest = TransactionManifest::sample();
        assert_eq!(
            manifest.clone().modify_add_guarantees([]).unwrap(),
            manifest
        );
    }

    #[test]
    fn test_modify_manifest_add_guarantees_unchanged_if_instructions_empty() {
        let manifest = TransactionManifest::empty(NetworkID::Mainnet);
        assert_eq!(
            manifest.clone().modify_add_guarantees([]).unwrap(),
            manifest
        );
    }

    #[test]
    fn test_modify_manifest_add_guarantees_returns_error_index_equal_to_instruction_count(
    ) {
        let manifest = TransactionManifest::sample();
        assert_eq!(
            manifest.modify_add_guarantees([TransactionGuarantee::new(
                0,
                0,
                4,
                ResourceAddress::sample(),
                None
            )]),
            Err(CommonError::TXGuaranteeIndexOutOfBounds {
                index: 4,
                count: 4
            })
        );
    }

    #[test]
    fn test_modify_manifest_add_guarantees_returns_error_index_larger_than_instruction_count(
    ) {
        let manifest = TransactionManifest::sample();
        assert_eq!(
            manifest.modify_add_guarantees(vec![TransactionGuarantee::new(
                0,
                0,
                5,
                ResourceAddress::sample(),
                None
            )]),
            Err(CommonError::TXGuaranteeIndexOutOfBounds {
                index: 5,
                count: 4
            })
        );
    }
}
