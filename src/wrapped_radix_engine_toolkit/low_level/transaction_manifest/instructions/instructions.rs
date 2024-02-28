use std::ops::Deref;

use crate::prelude::*;

use transaction::{
    manifest::compile as scrypto_compile,
    manifest::decompile as scrypto_decompile,
    manifest::MockBlobProvider as ScryptoMockBlobProvider,
    prelude::InstructionV1 as ScryptoInstruction,
};

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display, uniffi::Record)]
#[display("{}", self.instructions_string())]
pub struct Instructions {
    pub(crate) secret_magic: InstructionsSecretMagic, // MUST be first prop, else you break build.
    pub network_id: NetworkID,
}

impl Deref for Instructions {
    type Target = Vec<ScryptoInstruction>;

    fn deref(&self) -> &Self::Target {
        &self.secret_magic.0
    }
}

impl Instructions {
    pub fn instructions_string(&self) -> String {
        let network_definition = self.network_id.network_definition();
        scrypto_decompile(self, &network_definition).expect("Should never fail, because should never have allowed invalid instructions")
    }

    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
    ) -> Result<Self> {
        let network_definition = network_id.network_definition();
        let blob_provider = ScryptoMockBlobProvider::new();
        scrypto_compile(
            instructions_string.as_ref(),
            &network_definition,
            blob_provider,
        )
        .map_err(|_e| CommonError::InvalidInstructionsString)
        .map(|manifest| Self {
            secret_magic: InstructionsSecretMagic(manifest.instructions),
            network_id,
        })
    }
}

impl HasPlaceholder for Instructions {
    fn placeholder() -> Self {
        Self::placeholder_simulator()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_simulator_other()
    }
}

impl Instructions {
    // https://github.com/radixdlt/radix-engine-toolkit/blob/cf2f4b4d6de56233872e11959861fbf12db8ddf6/crates/radix-engine-toolkit/tests/manifests/account/resource_transfer.rtm
    // but modified, changed `None` -> `Enum<0u8>()`
    pub(crate) fn placeholder_simulator_instructions_string() -> String {
        include_str!("resource_transfer.rtm").to_owned()
    }

    pub fn placeholder_simulator() -> Self {
        Self::new(
            Self::placeholder_simulator_instructions_string(),
            NetworkID::Simulator,
        )
        .expect("Valid placeholder value")
    }

    // https://github.com/radixdlt/radix-engine-toolkit/blob/cf2f4b4d6de56233872e11959861fbf12db8ddf6/crates/radix-engine-toolkit/tests/manifests/account/multi_account_resource_transfer.rtm
    // but modified, changed `None` -> `Enum<0u8>()`, also changed `"account_a_bucket"` -> `"bucket1"`, `"account_b_bucket"` -> `"bucket2"`, etc.
    pub(crate) fn placeholder_other_simulator_instructions_string() -> String {
        include_str!("multi_account_resource_transfer.rtm").to_owned()
    }

    pub fn placeholder_simulator_other() -> Self {
        Self::new(
            Self::placeholder_other_simulator_instructions_string(),
            NetworkID::Simulator,
        )
        .expect("Valid placeholder value")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Instructions;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn network_id() {
        assert_eq!(
            SUT::placeholder_simulator().network_id,
            NetworkID::Simulator
        );
    }
}
