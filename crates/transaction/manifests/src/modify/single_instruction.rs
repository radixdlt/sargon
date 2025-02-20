use crate::prelude::*;

/// Creates a single manifest Instruction using the `ScryptoTransactionManifestBuilder`,
///
/// # Panics
/// You MUST NOT chain calls to the manifest builder, only call a single method
/// on it, thus creating just a single instruction.
pub(crate) fn single_instruction<F>(by: F) -> ScryptoInstruction
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
pub(crate) fn single_instruction_v2<F>(by: F) -> ScryptoInstructionV2
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
