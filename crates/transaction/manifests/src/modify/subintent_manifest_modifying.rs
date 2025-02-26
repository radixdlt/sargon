use crate::prelude::*;
use radix_transactions::builder::SubintentManifestV2Builder;

impl ModifyingManifest<SubintentManifest, ScryptoInstructionV2>
    for SubintentManifest
{
    fn modifying_manifest(&self) -> SubintentManifest {
        self.clone()
    }
}

impl AddingGuaranteesModifyingManifest<SubintentManifest, ScryptoInstructionV2>
    for SubintentManifest
{
    fn insert_guarantee_assertion_at_position(
        self,
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
    AddingLockFeeAndProofsModifyingManifest<
        SubintentManifest,
        SubintentManifestV2Builder,
        ScryptoInstructionV2,
    > for SubintentManifest
{
}
