use crate::prelude::*;

use crate::modify::manifest_abstractions::{
    IntoInstruction, IntoManifest, IntoManifestBuilder,
};
use radix_engine_interface::blueprints::access_controller::ACCESS_CONTROLLER_CREATE_PROOF_IDENT;

/// Used by development, in production we SHOULD use the fee given by analyzing
/// the manifest.
fn default_fee() -> Decimal192 {
    Decimal192::from(25)
}

/// Creates a single manifest Instruction using the `ScryptoTransactionManifestBuilder`,
///
/// # Panics
/// You MUST NOT chain calls to the manifest builder, only call a single method
/// on it, thus creating just a single instruction.
fn single_instruction<F>(by: F) -> ScryptoInstruction
where
    F: Fn(
        ScryptoTransactionManifestBuilder,
    ) -> ScryptoTransactionManifestBuilder,
{
    let instruction = by(ScryptoTransactionManifestBuilder::new())
        .build()
        .instructions;

    // This might be a silly assertion since it seems that ScryptoManifestBuilder
    // in fact always adds just a single instruction
    if instruction.len() != 1 {
        panic!("Expected single instruction. You MUST NOT chain calls with the manifest builder.")
    }
    instruction[0].clone()
}

/// Creates a single manifest Instruction using the `ScryptoSubintentManifestV2Builder`,
///
/// # Panics
/// You MUST NOT chain calls to the manifest builder, only call a single method
/// on it, thus creating just a single instruction.
fn single_instruction_v2<F>(by: F) -> ScryptoInstructionV2
where
    F: Fn(
        ScryptoTransactionManifestV2Builder,
    ) -> ScryptoTransactionManifestV2Builder,
{
    let instruction = by(ScryptoTransactionManifestV2Builder::new_v2())
        .build()
        .instructions;

    // This might be a silly assertion since it seems that ScryptoManifestBuilder
    // in fact always adds just a single instruction
    if instruction.len() != 1 {
        panic!("Expected single instruction. You MUST NOT chain calls with the manifest builder.")
    }
    instruction[0].clone()
}

pub(crate) struct LockFeeData {
    fee_payer_address: AccountAddress,
    fee: Option<Decimal192>,
}

impl LockFeeData {
    pub(crate) fn new_with_fee(
        fee_payer_address: AccountAddress,
        fee: Decimal192,
    ) -> Self {
        Self {
            fee_payer_address,
            fee: Some(fee),
        }
    }

    pub(crate) fn new_with_fee_payer(
        fee_payer_address: AccountAddress,
    ) -> Self {
        Self {
            fee_payer_address,
            fee: None,
        }
    }
}

pub(crate) trait ModifiableManifest<B, M, I>
where
    B: IntoManifestBuilder<M, I>,
    M: IntoManifest<I>,
    I: IntoInstruction + Clone,
    Self: Sized,
{
    fn manifest(&self) -> M;

    fn insert_guarantee_assertion_at_position(
        &self,
        position: InstructionPosition,
        guarantee: TransactionGuarantee,
    ) -> Result<Self>;

    /// Modifies the current manifest to add `lock_fee` instruction. Also adds all the proofs
    /// provided in `entities_with_access_controllers` by prepending the `create_proof` method at
    /// the top.
    ///
    /// Beware that if the fee payer is controlled by an `AccessControllerAddress` provided by
    /// `entities_with_access_controllers` then that `create_proof` instruction will be placed just
    /// before the `lock_fee` instruction.
    fn modify_add_proofs_and_lock_fee(
        self,
        lock_fee_data: Option<LockFeeData>,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
    ) -> Result<M> {
        let mut access_controllers = entities_with_access_controllers
            .iter()
            .map(|(_, ac)| *ac)
            .collect::<IndexSet<_>>();

        let mut builder = B::new_with_instructions([]);

        if let Some(lock_fee_data) = lock_fee_data {
            let lock_fee_entity_address = AddressOfAccountOrPersona::Account(
                lock_fee_data.fee_payer_address,
            );

            let fee = lock_fee_data.fee.unwrap_or(default_fee());
            if let Some(access_controller_of_fee_payer) =
                entities_with_access_controllers.get(&lock_fee_entity_address)
            {
                access_controllers.shift_remove(access_controller_of_fee_payer);

                // Add proof for lock fee payer, who happens to be securified.
                builder = builder.call_method(
                    ScryptoGlobalAddress::from(*access_controller_of_fee_payer),
                    ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                    (),
                );
            }

            // Add lock fee
            builder = builder.lock_fee(&lock_fee_data.fee_payer_address, fee)
        }

        // Put the remaining proofs of the Access Controller addresses
        for access_controller in access_controllers {
            builder = builder.call_method(
                ScryptoGlobalAddress::from(access_controller),
                ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                (),
            );
        }

        builder = builder.extend_builder_with_manifest(self.manifest());

        builder.build(self.manifest().network_id())
    }

    // Modifies `manifest` by inserting transaction "guarantees", which is the wallet
    // term for `assert_worktop_contains`.
    fn modify_add_guarantees<G>(self, guarantees: G) -> Result<M>
    where
        G: IntoIterator<Item = TransactionGuarantee>,
    {
        let guarantees = guarantees.into_iter().collect_vec();
        if guarantees.is_empty() {
            return Ok(self.manifest());
        };

        let instructions = self.manifest().instructions();
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
        if instructions.iter().any(|i| i.is_lock_fee()) {
            offset = 1;
        }

        let mut modifiable_manifest = self;

        for guarantee in guarantees {
            modifiable_manifest = modifiable_manifest
                .insert_guarantee_assertion_at_position(
                    InstructionPosition(guarantee.instruction_index + offset),
                    guarantee,
                )?;

            offset.add_assign(1);
        }

        Ok(modifiable_manifest.manifest())
    }
}

impl
    ModifiableManifest<
        ScryptoTransactionManifestBuilder,
        TransactionManifest,
        ScryptoInstruction,
    > for TransactionManifest
{
    fn manifest(&self) -> TransactionManifest {
        self.clone()
    }

    fn insert_guarantee_assertion_at_position(
        &self,
        position: InstructionPosition,
        guarantee: TransactionGuarantee,
    ) -> Result<Self> {
        let rounded_amount = guarantee.rounded_amount();

        let instruction = single_instruction(|b| {
            b.assert_worktop_contains(
                &guarantee.resource_address,
                rounded_amount,
            )
        });

        let mut instructions = self.instructions().clone();
        instructions.insert(position.0 as usize, instruction);

        let instructions =
            Instructions::try_from((instructions.as_ref(), self.network_id()))?;

        Ok(TransactionManifest::with_instructions_and_blobs(
            instructions,
            self.blobs().clone(),
        ))
    }
}

impl
    ModifiableManifest<
        ScryptoSubintentManifestV2Builder,
        SubintentManifest,
        ScryptoInstructionV2,
    > for SubintentManifest
{
    fn manifest(&self) -> SubintentManifest {
        self.clone()
    }

    fn insert_guarantee_assertion_at_position(
        &self,
        position: InstructionPosition,
        guarantee: TransactionGuarantee,
    ) -> Result<Self> {
        let rounded_amount = guarantee.rounded_amount();

        let instruction = single_instruction_v2(|b| {
            b.assert_worktop_contains(
                &guarantee.resource_address,
                rounded_amount,
            )
        });

        let mut instructions = self.instructions().clone();
        instructions.insert(position.0 as usize, instruction);

        let instructions = InstructionsV2::try_from((
            instructions.as_ref(),
            self.network_id(),
        ))?;

        Ok(SubintentManifest::with_instructions_and_blobs_and_children(
            instructions,
            self.blobs().clone(),
            self.children().clone(),
        ))
    }
}

impl
    ModifiableManifest<
        ScryptoTransactionManifestV2Builder,
        TransactionManifestV2,
        ScryptoInstructionV2,
    > for TransactionManifestV2
{
    fn manifest(&self) -> TransactionManifestV2 {
        self.clone()
    }

    fn insert_guarantee_assertion_at_position(
        &self,
        _position: InstructionPosition,
        _guarantee: TransactionGuarantee,
    ) -> Result<Self> {
        panic!("Should not happen")
    }
}

pub(crate) struct InstructionPosition(u64);

#[cfg(test)]
mod tests {
    use radix_transactions::manifest::{AssertWorktopContains, DropAllProofs};

    use super::*;

    #[test]
    fn default_dev_fee_is() {
        assert_eq!(default_fee(), 25.into());
    }

    #[test]
    fn is_lock_fee() {
        assert!(!ScryptoInstruction::DropAllProofs(DropAllProofs).is_lock_fee());
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
        let mut manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();
        manifest = manifest
            .modify_add_guarantees([TransactionGuarantee::new(
                added_guaranteed_amount,
                percentage,
                index,
                resource,
                divisibility,
            )])
            .unwrap();
        let instructions = manifest.instructions().to_owned();
        let instruction = instructions[index as usize + 1].clone();
        assert_eq!(
            instruction,
            ScryptoInstruction::AssertWorktopContains(AssertWorktopContains {
                resource_address: resource.into(),
                amount: rounded_guaranteed_amount.into(),
            })
        );
    }

    #[test]
    #[should_panic(
        expected = "Expected single instruction. You MUST NOT chain calls with the manifest builder."
    )]
    fn single_when_more_than_one_panic() {
        _ = single_instruction(|b| b.drop_all_proofs().drop_auth_zone_proofs())
    }

    #[test]
    #[should_panic(
        expected = "Expected single instruction. You MUST NOT chain calls with the manifest builder."
    )]
    fn single_v2_when_more_than_one_panic() {
        _ = single_instruction_v2(|b| {
            b.drop_all_proofs().drop_auth_zone_proofs()
        })
    }

    #[test]
    fn test_modify_manifest_lock_fee() {
        let instructions_string = r#"
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
        "#;

        let manifest = TransactionManifest::new(
            instructions_string,
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();

        manifest_eq(
            manifest.modify_add_lock_fee(
                &"account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264".parse().unwrap(),
            Some(42.into())
            ).unwrap(),
            r#"
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
            "lock_fee"
            Decimal("42")
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
    fn test_modify_manifest_lock_fee_default_added_if_none_provided() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();

        manifest_eq(
        manifest.modify_add_lock_fee(
            &"account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264".parse().unwrap(),
            None,
        ).unwrap(),
        r#"
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
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
    fn test_modify_manifest_add_guarantees_to_manifest_without_lock_fee() {
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
    fn test_modify_manifest_add_guarantees_to_manifest_with_lock_fee() {
        let manifest = TransactionManifest::sample();

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
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("0.61")
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
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
                Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
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

    #[test]
    fn test_modify_add_proofs_and_lock_fee_when_no_proofs_provided() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();

        manifest_eq(
            manifest.modify_add_proofs_and_lock_fee(
                Some(LockFeeData::new_with_fee_payer(
                    AccountAddress::try_from_bech32(
                        "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
                    ).unwrap()
                )),
                radix_rust::indexmap!()
            ).unwrap(),
            r#"
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
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
    fn test_modify_add_proofs_and_lock_fee_when_proofs_provided() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();

        // This account will lock the fee, but is also securified...
        let account_address = AccountAddress::try_from_bech32(
            "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
        ).unwrap();
        // ...by this access controller
        let acc1 = AccessControllerAddress::try_from_bech32(
            "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a"
        ).unwrap();

        let acc2 = AccessControllerAddress::try_from_bech32(
            "accesscontroller_rdx1cw68j9ca4fye09mz3hshp4qydjnxhsahm68hvmz9cjhftcz9gnn375"
        ).unwrap();
        let acc3 = AccessControllerAddress::try_from_bech32(
            "accesscontroller_rdx1cwdxqrwpx9hrng5e6l6qfyhp9wfem03ls9z9kvutqpmfk665pqa3y5"
        ).unwrap();

        manifest_eq(
            manifest
                .modify_add_proofs_and_lock_fee(
                    Some(LockFeeData::new_with_fee_payer(account_address)),
                    IndexMap::from([
                        (
                            AddressOfAccountOrPersona::Account(account_address),
                            acc1,
                        ),
                        (AddressOfAccountOrPersona::sample_mainnet(), acc2),
                        (
                            AddressOfAccountOrPersona::sample_mainnet_other(),
                            acc3,
                        ),
                    ]),
                )
                .unwrap(),
            r#"
        CALL_METHOD
            Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
            "create_proof"
        ;
        CALL_METHOD
            Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
            "lock_fee"
            Decimal("25")
        ;
        CALL_METHOD
            Address("accesscontroller_rdx1cw68j9ca4fye09mz3hshp4qydjnxhsahm68hvmz9cjhftcz9gnn375")
            "create_proof"
        ;
        CALL_METHOD
            Address("accesscontroller_rdx1cwdxqrwpx9hrng5e6l6qfyhp9wfem03ls9z9kvutqpmfk665pqa3y5")
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
}
