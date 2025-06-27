use radix_transactions::prelude::ManifestBuilder;

use crate::prelude::*;

pub trait BuilderExtendWithInstructions: Sized {
    fn extend_builder_with_instructions(
        self,
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
    ) -> ManifestBuilder;

    fn extend_builder_with_manifest(
        self,
        manifest: TransactionManifest,
    ) -> ManifestBuilder {
        Self::extend_builder_with_instructions(
            self,
            manifest.instructions().clone(),
        )
    }
}

impl BuilderExtendWithInstructions for ManifestBuilder {
    fn extend_builder_with_instructions(
        self,
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
    ) -> ManifestBuilder {
        instructions.into_iter().fold(self, |builder, instruction| {
            builder.add_instruction_advanced(instruction).0
        })
    }
}

pub trait BuilderFromManifest: BuilderExtendWithInstructions {
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
        Self::extend_builder_with_instructions(
            ManifestBuilder::new(),
            instructions,
        )
    }
}
