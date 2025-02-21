use crate::prelude::*;

impl ModifyingManifest<TransactionManifest, ScryptoInstruction>
    for TransactionManifest
{
    fn modifying_manifest(&self) -> TransactionManifest {
        self.clone()
    }
}

impl AddingGuaranteesModifyingManifest<TransactionManifest, ScryptoInstruction>
    for TransactionManifest
{
    fn insert_guarantee_assertion_at_position(
        self,
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
    AddingLockFeeAndProofsModifyingManifest<
        TransactionManifest,
        ScryptoTransactionManifestBuilder,
        ScryptoInstruction,
    > for TransactionManifest
{
}
