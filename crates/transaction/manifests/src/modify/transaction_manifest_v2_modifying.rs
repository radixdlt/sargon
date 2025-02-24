use crate::prelude::*;
use radix_transactions::model::InstructionV2;

impl ModifyingManifest<TransactionManifestV2, ScryptoInstructionV2>
    for TransactionManifestV2
{
    fn modifying_manifest(&self) -> TransactionManifestV2 {
        self.clone()
    }
}

impl
    AddingLockFeeAndProofsModifyingManifest<
        TransactionManifestV2,
        ScryptoTransactionManifestV2Builder,
        InstructionV2,
    > for TransactionManifestV2
{
}
