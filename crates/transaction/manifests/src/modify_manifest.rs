use crate::prelude::*;

use radix_engine_interface::blueprints::account::ACCOUNT_LOCK_FEE_IDENT;

pub trait InspectInstruction {
    fn is_lock_fee(&self) -> bool;
    fn is_assert_worktop_contains(&self) -> bool;
}

impl InspectInstruction for ScryptoInstruction {
    fn is_lock_fee(&self) -> bool {
        match self {
            ScryptoInstruction::CallMethod(call_method) => {
                call_method.method_name == ACCOUNT_LOCK_FEE_IDENT
            }
            _ => false,
        }
    }

    // FIXME: this will be simpler once we get EnumAsInner on `ScryptoInstruction`
    fn is_assert_worktop_contains(&self) -> bool {
        matches!(self, ScryptoInstruction::AssertWorktopContains(_))
    }
}

/// Used by development, in production we SHOULD use the fee given by analyzing
/// the manifest.
fn default_fee() -> Decimal192 {
    Decimal192::from(25)
}

/// Creates a single manifest Instruction using the `ScryptoManifestBuilder`,
///
/// # Panics
/// You MUST NOT chain calls to the manifest builder, only call a single method
/// on it, thus creating just a single instruction.
fn single<F>(by: F) -> ScryptoInstruction
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

pub trait TransactionManifestModifying: Sized {
    fn modify_add_guarantees<I>(
        self,
        guarantees: I,
    ) -> Result<TransactionManifest>
    where
        I: IntoIterator<Item = TransactionGuarantee>;

    fn modify_add_guarantees_vec(
        self,
        guarantees: Vec<TransactionGuarantee>,
    ) -> Result<TransactionManifest> {
        self.modify_add_guarantees(guarantees)
    }

    fn modify_add_lock_fee(
        self,
        address_of_fee_payer: &AccountAddress,
        fee: impl Into<Option<Decimal192>>,
    ) -> Self {
        let fee = fee.into().unwrap_or(default_fee());
        let instruction = single(|b| b.lock_fee(address_of_fee_payer, fee));
        self.prepend_instruction(instruction)
    }

    fn prepend_instruction(self, instruction: ScryptoInstruction) -> Self {
        self.insert_instruction(InstructionPosition::First, instruction)
    }

    fn insert_instruction(
        self,
        position: InstructionPosition,
        instruction: ScryptoInstruction,
    ) -> Self;
}

impl TransactionManifestModifying for TransactionManifest {
    /// Modifies `manifest` by inserting transaction "guarantees", which is the wallet
    /// term for `assert_worktop_contains`.
    fn modify_add_guarantees<I>(
        self,
        guarantees: I,
    ) -> Result<TransactionManifest>
    where
        I: IntoIterator<Item = TransactionGuarantee>,
    {
        let guarantees = guarantees.into_iter().collect_vec();
        if guarantees.is_empty() {
            return Ok(self);
        };

        let instruction_count = self.instructions().len() as u64;

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

        let first = self.instructions().first().unwrap();
        if first.is_lock_fee() {
            offset = 1;
        }

        let mut manifest = self;

        for guarantee in guarantees {
            let rounded_amount = guarantee.rounded_amount();

            let guarantee_instruction = single(|b| {
                b.assert_worktop_contains(
                    &guarantee.resource_address,
                    rounded_amount,
                )
            });

            manifest = manifest.insert_instruction(
                InstructionPosition::At(guarantee.instruction_index + offset),
                guarantee_instruction,
            );

            offset.add_assign(1);
        }

        Ok(manifest)
    }

    fn insert_instruction(
        self,
        position: InstructionPosition,
        instruction: ScryptoInstruction,
    ) -> Self {
        let mut instructions = self.instructions().clone();

        match position {
            InstructionPosition::First => instructions.insert(0, instruction),
            InstructionPosition::At(index) => {
                instructions.insert(index as usize, instruction)
            }
        };

        let instructions = Instructions::try_from((
            instructions.as_ref(),
            self.network_id(),
        )).expect("Should not have changed depth of SBOR value, thus inserting an instruction should never fail.");

        TransactionManifest::with_instructions_and_blobs(
            instructions,
            self.blobs().clone(),
        )
    }
}

pub enum InstructionPosition {
    First,
    At(u64),
}

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
        _ = single(|b| b.drop_all_proofs().drop_auth_zone_proofs())
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
            ),
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
        ),
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
}
