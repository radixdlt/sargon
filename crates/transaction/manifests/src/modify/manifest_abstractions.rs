use crate::prelude::*;
use radix_rust::Resolve;
use radix_transactions::builder::{
    ReferencedManifestComponentAddress, ReferencedManifestGlobalAddress,
    ResolvableArguments,
};

/// Common representation of any Manifest like `TransactionManifest` or `SubintentManifest`
/// consumed or modified by the host.
pub trait IntoManifest<I>: Sized
where
    I: IntoInstruction + Clone,
{
    fn network_id(&self) -> NetworkID;

    fn instructions(&self) -> Vec<I>;

    fn blobs(&self) -> Blobs;
}

impl IntoManifest<ScryptoInstruction> for TransactionManifest {
    fn network_id(&self) -> NetworkID {
        self.network_id()
    }

    fn instructions(&self) -> Vec<ScryptoInstruction> {
        self.instructions.instructions.clone()
    }

    fn blobs(&self) -> Blobs {
        self.blobs.clone()
    }
}

impl IntoManifest<ScryptoInstructionV2> for SubintentManifest {
    fn network_id(&self) -> NetworkID {
        self.network_id()
    }

    fn instructions(&self) -> Vec<ScryptoInstructionV2> {
        self.instructions.instructions.clone()
    }

    fn blobs(&self) -> Blobs {
        self.blobs.clone()
    }
}

impl IntoManifest<ScryptoInstructionV2> for TransactionManifestV2 {
    fn network_id(&self) -> NetworkID {
        self.network_id()
    }

    fn instructions(&self) -> Vec<ScryptoInstructionV2> {
        self.instructions.instructions.clone()
    }

    fn blobs(&self) -> Blobs {
        self.blobs.clone()
    }
}

/// Common representation of an Instruction included in any `IntoManifest`.
pub trait IntoInstruction: Sized {}

impl IntoInstruction for ScryptoInstruction {}

impl IntoInstruction for ScryptoInstructionV2 {}

/// Common representation of a manifest builder
pub trait IntoManifestBuilder<M, I>: Sized
where
    M: IntoManifest<I>,
    I: IntoInstruction + Clone,
{
    fn extend_builder_with_instructions_and_blobs(
        self,
        instructions: impl IntoIterator<Item = I>,
        blobs: Blobs,
    ) -> Self;

    fn new_with_instructions(
        instructions: impl IntoIterator<Item = I>,
        blobs: Blobs,
    ) -> Self;

    fn build(self, network_id: NetworkID) -> Result<M>;

    fn call_method(
        self,
        address: impl ReferencedManifestGlobalAddress,
        method_name: impl Into<String>,
        arguments: impl ResolvableArguments,
    ) -> Self;

    fn lock_fee(
        self,
        account_address: impl ReferencedManifestComponentAddress,
        amount: impl Resolve<radix_common::math::Decimal>,
    ) -> Self;

    fn extend_builder_with_manifest(self, manifest: M) -> Self {
        Self::extend_builder_with_instructions_and_blobs(
            self,
            manifest.instructions().clone(),
            manifest.blobs().clone(),
        )
    }

    fn new_with_manifest(manifest: M) -> Self {
        Self::new_with_instructions(
            manifest.instructions().clone(),
            manifest.blobs(),
        )
    }
}

impl IntoManifestBuilder<TransactionManifest, ScryptoInstruction>
    for ScryptoTransactionManifestBuilder
{
    fn extend_builder_with_instructions_and_blobs(
        self,
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
        blobs: Blobs,
    ) -> Self {
        instructions
            .into_iter()
            .fold(self, |builder, instruction| {
                builder.add_instruction_advanced(instruction).0
            })
            .then(|mut builder| {
                // Each blob on its own, as instructions refer to them by
                // hash: none if there are none, and never merged into one.
                for blob in blobs.blobs() {
                    builder.add_blob(blob.into());
                }
                builder
            })
    }

    fn new_with_instructions(
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
        blobs: Blobs,
    ) -> Self {
        Self::extend_builder_with_instructions_and_blobs(
            ScryptoTransactionManifestBuilder::new(),
            instructions,
            blobs,
        )
    }

    fn build(self, network_id: NetworkID) -> Result<TransactionManifest> {
        let scrypto_manifest = self.build();

        TransactionManifest::try_from((scrypto_manifest, network_id))
    }

    fn call_method(
        self,
        address: impl ReferencedManifestGlobalAddress,
        method_name: impl Into<String>,
        arguments: impl ScryptoResolvableArguments,
    ) -> Self {
        self.call_method(address, method_name, arguments)
    }

    fn lock_fee(
        self,
        account_address: impl ReferencedManifestComponentAddress,
        amount: impl Resolve<ScryptoDecimal192>,
    ) -> Self {
        self.lock_fee(account_address, amount)
    }
}

impl IntoManifestBuilder<SubintentManifest, ScryptoInstructionV2>
    for ScryptoSubintentManifestV2Builder
{
    fn extend_builder_with_instructions_and_blobs(
        self,
        instructions: impl IntoIterator<Item = ScryptoInstructionV2>,
        blobs: Blobs,
    ) -> Self {
        instructions
            .into_iter()
            .fold(self, |builder, instruction| {
                builder.add_instruction_advanced(instruction).0
            })
            .then(|mut builder| {
                // Each blob on its own, as instructions refer to them by
                // hash: none if there are none, and never merged into one.
                for blob in blobs.blobs() {
                    builder.add_blob(blob.into());
                }
                builder
            })
    }

    fn new_with_instructions(
        instructions: impl IntoIterator<Item = ScryptoInstructionV2>,
        blobs: Blobs,
    ) -> Self {
        Self::extend_builder_with_instructions_and_blobs(
            ScryptoSubintentManifestV2Builder::new_subintent_v2(),
            instructions,
            blobs,
        )
    }

    fn build(self, network_id: NetworkID) -> Result<SubintentManifest> {
        let scrypto_manifest = self.build();

        SubintentManifest::try_from((scrypto_manifest, network_id))
    }

    fn call_method(
        self,
        address: impl ReferencedManifestGlobalAddress,
        method_name: impl Into<String>,
        arguments: impl ScryptoResolvableArguments,
    ) -> Self {
        self.call_method(address, method_name, arguments)
    }

    fn lock_fee(
        self,
        account_address: impl ReferencedManifestComponentAddress,
        amount: impl Resolve<ScryptoDecimal192>,
    ) -> Self {
        self.lock_fee(account_address, amount)
    }
}

impl IntoManifestBuilder<TransactionManifestV2, ScryptoInstructionV2>
    for ScryptoTransactionManifestV2Builder
{
    fn extend_builder_with_instructions_and_blobs(
        self,
        instructions: impl IntoIterator<Item = ScryptoInstructionV2>,
        blobs: Blobs,
    ) -> Self {
        instructions
            .into_iter()
            .fold(self, |builder, instruction| {
                builder.add_instruction_advanced(instruction).0
            })
            .then(|mut builder| {
                // Each blob on its own, as instructions refer to them by
                // hash: none if there are none, and never merged into one.
                for blob in blobs.blobs() {
                    builder.add_blob(blob.into());
                }
                builder
            })
    }

    fn new_with_instructions(
        instructions: impl IntoIterator<Item = ScryptoInstructionV2>,
        blobs: Blobs,
    ) -> Self {
        Self::extend_builder_with_instructions_and_blobs(
            ScryptoTransactionManifestV2Builder::new_v2(),
            instructions,
            blobs,
        )
    }

    fn build(self, network_id: NetworkID) -> Result<TransactionManifestV2> {
        let scrypto_manifest = self.build();

        TransactionManifestV2::try_from((scrypto_manifest, network_id))
    }

    fn call_method(
        self,
        address: impl ReferencedManifestGlobalAddress,
        method_name: impl Into<String>,
        arguments: impl ScryptoResolvableArguments,
    ) -> Self {
        self.call_method(address, method_name, arguments)
    }

    fn lock_fee(
        self,
        account_address: impl ReferencedManifestComponentAddress,
        amount: impl Resolve<ScryptoDecimal192>,
    ) -> Self {
        self.lock_fee(account_address, amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_blobs() -> Blobs {
        Blobs::new([
            Blob::from(BagOfBytes::from(vec![1, 2, 3])),
            Blob::from(BagOfBytes::from(vec![4, 5])),
        ])
    }

    /// Rebuilds `manifest` from its instructions and blobs, as modifying it
    /// does.
    fn rebuilt<M, I, B>(manifest: &M) -> M
    where
        M: IntoManifest<I>,
        I: IntoInstruction + Clone,
        B: IntoManifestBuilder<M, I>,
    {
        IntoManifestBuilder::build(
            B::new_with_instructions(
                IntoManifest::instructions(manifest),
                IntoManifest::blobs(manifest),
            ),
            IntoManifest::network_id(manifest),
        )
        .unwrap()
    }

    #[test]
    fn subintent_without_blobs_is_rebuilt_without_blobs() {
        let manifest = SubintentManifest::sample();
        assert!(manifest.blobs().blobs().is_empty());

        let rebuilt =
            rebuilt::<_, _, ScryptoSubintentManifestV2Builder>(&manifest);

        assert!(rebuilt.blobs().blobs().is_empty());
        assert_eq!(rebuilt, manifest);
    }

    #[test]
    fn subintent_blobs_are_rebuilt_separately() {
        let manifest =
            SubintentManifest::with_instructions_and_blobs_and_children(
                SubintentManifest::sample().instructions,
                two_blobs(),
                ChildSubintentSpecifiers::default(),
            );

        let rebuilt =
            rebuilt::<_, _, ScryptoSubintentManifestV2Builder>(&manifest);

        assert_eq!(rebuilt.blobs(), &two_blobs());
    }

    #[test]
    fn transaction_v2_without_blobs_is_rebuilt_without_blobs() {
        let manifest = TransactionManifestV2::sample();
        assert!(manifest.blobs.blobs().is_empty());

        let rebuilt =
            rebuilt::<_, _, ScryptoTransactionManifestV2Builder>(&manifest);

        assert!(rebuilt.blobs.blobs().is_empty());
    }

    #[test]
    fn transaction_v1_blobs_are_rebuilt_separately() {
        let manifest = TransactionManifest::with_instructions_and_blobs(
            TransactionManifest::sample().instructions,
            two_blobs(),
        );

        let rebuilt =
            rebuilt::<_, _, ScryptoTransactionManifestBuilder>(&manifest);

        assert_eq!(rebuilt.blobs, two_blobs());
    }

    /// A pre-authorization's subintent must hash as the dApp built it, e.g.
    /// so that others can sign it first: modifying it with nothing to add
    /// must leave it as it was.
    #[test]
    fn modifying_a_subintent_with_nothing_to_add_leaves_it_unchanged() {
        let manifest = SubintentManifest::sample();

        let modified = manifest
            .clone()
            .modify_add_guarantees(Vec::<TransactionGuarantee>::new())
            .unwrap();

        assert_eq!(modified, manifest);
    }
}
