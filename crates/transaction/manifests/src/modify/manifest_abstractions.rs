use crate::prelude::*;
use radix_engine_interface::blueprints::account::ACCOUNT_LOCK_FEE_IDENT;
use radix_rust::Resolve;
use radix_transactions::builder::{
    ReferencedManifestComponentAddress, ReferencedManifestGlobalAddress,
    ResolvableArguments,
};

/// Common representation of any Manifest like `TransactionManifest` or `SubintentManifest`
/// consumed or modified by the host.
pub(crate) trait IntoManifest<I>: Sized
where
    I: IntoInstruction + Clone,
{
    fn network_id(&self) -> NetworkID;

    fn instructions(&self) -> Vec<I>;
}

impl IntoManifest<ScryptoInstruction> for TransactionManifest {
    fn network_id(&self) -> NetworkID {
        self.network_id()
    }

    fn instructions(&self) -> Vec<ScryptoInstruction> {
        self.instructions.instructions.clone()
    }
}

impl IntoManifest<ScryptoInstructionV2> for SubintentManifest {
    fn network_id(&self) -> NetworkID {
        self.network_id()
    }

    fn instructions(&self) -> Vec<ScryptoInstructionV2> {
        self.instructions.instructions.clone()
    }
}

/// Common representation of an Instruction included in any `IntoManifest`.
pub(crate) trait IntoInstruction: Sized {
    fn is_lock_fee(&self) -> bool;
}

impl IntoInstruction for ScryptoInstruction {
    fn is_lock_fee(&self) -> bool {
        match self {
            ScryptoInstruction::CallMethod(call_method) => {
                call_method.method_name == ACCOUNT_LOCK_FEE_IDENT
            }
            _ => false,
        }
    }
}

impl IntoInstruction for ScryptoInstructionV2 {
    fn is_lock_fee(&self) -> bool {
        match self {
            ScryptoInstructionV2::CallMethod(call_method) => {
                call_method.method_name == ACCOUNT_LOCK_FEE_IDENT
            }
            _ => false,
        }
    }
}

/// Common representation of a manifest builder
pub(crate) trait IntoManifestBuilder<M, I>: Sized
where
    M: IntoManifest<I>,
    I: IntoInstruction + Clone,
{
    fn extend_builder_with_instructions(
        self,
        instructions: impl IntoIterator<Item = I>,
    ) -> Self;

    fn new_with_instructions(instructions: impl IntoIterator<Item = I>)
        -> Self;

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
        Self::extend_builder_with_instructions(
            self,
            manifest.instructions().clone(),
        )
    }

    fn new_with_manifest(manifest: M) -> Self {
        Self::new_with_instructions(manifest.instructions().clone())
    }
}

impl IntoManifestBuilder<TransactionManifest, ScryptoInstruction>
    for ScryptoTransactionManifestBuilder
{
    fn extend_builder_with_instructions(
        self,
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
    ) -> Self {
        instructions.into_iter().fold(self, |builder, instruction| {
            builder.add_instruction_advanced(instruction).0
        })
    }

    fn new_with_instructions(
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
    ) -> Self {
        Self::extend_builder_with_instructions(
            ScryptoTransactionManifestBuilder::new(),
            instructions,
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
    fn extend_builder_with_instructions(
        self,
        instructions: impl IntoIterator<Item = ScryptoInstructionV2>,
    ) -> Self {
        instructions.into_iter().fold(self, |builder, instruction| {
            builder.add_instruction_advanced(instruction).0
        })
    }

    fn new_with_instructions(
        instructions: impl IntoIterator<Item = ScryptoInstructionV2>,
    ) -> Self {
        Self::extend_builder_with_instructions(
            ScryptoSubintentManifestV2Builder::new_subintent_v2(),
            instructions,
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
