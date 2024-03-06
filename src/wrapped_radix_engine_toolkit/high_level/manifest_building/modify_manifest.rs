use crate::prelude::*;

use transaction::model::{
    InstructionV1 as ScryptoInstruction, InstructionsV1 as ScryptoInstructions,
};

use transaction::prelude::ManifestBuilder as ScryptoManifestBuilder;

/// Used by development, in production we SHOULD use the fee given by analyzing
/// the manifest.
fn default_fee() -> Decimal192 {
    Decimal192::from(25)
}

fn single<F>(by: F) -> ScryptoInstruction
where
    F: Fn(ScryptoManifestBuilder) -> ScryptoManifestBuilder,
{
    let instruction = by(ScryptoManifestBuilder::new()).build().instructions;
    if instruction.len() != 1 {
        panic!("expected single instruction")
    }
    instruction[0].clone()
}

impl TransactionManifest {
    pub(crate) fn modify_add_lock_fee(
        self,
        address_of_fee_payer: &AccountAddress,
        fee: Option<Decimal192>,
    ) -> Self {
        let fee = fee.unwrap_or(default_fee());
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
    ) -> Self {
        let mut instructions = self.instructions().clone();

        match position {
            InstructionPosition::First => instructions.insert(0, instruction),
        };

        let instructions = Instructions::from_scrypto(
            ScryptoInstructions(instructions),
            self.network_id(),
        );

        TransactionManifest::with_instructions_and_blobs(
            instructions,
            self.blobs().clone(),
        )
    }
}

enum InstructionPosition {
    First,
}

#[cfg(test)]
mod tests {
    use radix_engine_interface::blueprints::account::AccountLockFeeInput;

    use super::*;

    #[test]
    fn default_dev_fee_is() {
        assert_eq!(default_fee().to_string(), "25");
    }

    #[test]
    fn test_modify_manifest_lock_fee() {
        let instructions_string = r#"
CALL_METHOD
    Address("account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8")
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
    Address("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69")
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
            modify_manifest_lock_fee(
                manifest,
                &"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master".parse().unwrap(),
                Some(42.into()),
            ),
            r#"
        CALL_METHOD
            Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
            "lock_fee"
            Decimal("42")
        ;
        CALL_METHOD
            Address("account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8")
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
            Address("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69")
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
                    modify_manifest_lock_fee(
                        manifest,
                        &"account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master".parse().unwrap(),
                        None,
                    ),
                    r#"
                CALL_METHOD
                    Address("account_rdx16yf8jxxpdtcf4afpj5ddeuazp2evep7quuhgtq28vjznee08master")
                    "lock_fee"
                    Decimal("25")
                ;
                CALL_METHOD
                    Address("account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8")
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
                    Address("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69")
                    "try_deposit_or_abort"
                    Bucket("bucket1")
                    Enum<0u8>()
                ;
                    "#,
                );
    }
}
