use radix_transactions::prelude::ManifestBuilder;

use crate::prelude::*;

pub trait BuilderFromManifest {
    fn with_instructions(
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
    ) -> ManifestBuilder;

    fn with_manifest(manifest: TransactionManifest) -> ManifestBuilder {
        Self::with_instructions(manifest.instructions().clone())
    }
}

impl BuilderFromManifest for ManifestBuilder {
    fn with_instructions(
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
    ) -> ManifestBuilder {
        instructions.into_iter().fold(
            ManifestBuilder::new(),
            |builder, instruction| {
                builder.add_instruction_advanced(instruction).0
            },
        )
    }
}
