use std::ops::Deref;

use crate::prelude::*;

use radix_engine::types::GlobalAddress;
use transaction::{
    manifest::{
        compile as scrypto_compile, decompile as scrypto_decompile,
        CompileError as ScryptoCompileError,
        MockBlobProvider as ScryptoMockBlobProvider,
    },
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
        .map_err(|e| extract_error_from_error(e, network_id))
        .map(|manifest| Self {
            secret_magic: InstructionsSecretMagic(manifest.instructions),
            network_id,
        })
    }
}

use radix_engine_toolkit::functions::address::decode as RET_decode_address;

fn extract_error_from_addr(
    s: String,
    expected_network: NetworkID,
) -> CommonError {
    let Some(Some(network_id)) = RET_decode_address(&s)
        .map(|t| t.0)
        .map(NetworkID::from_repr)
    else {
        return CommonError::InvalidInstructionsString;
    };
    if network_id != expected_network {
        CommonError::InvalidInstructionsWrongNetwork {
            found_in_instructions: network_id,
            specified_to_instructions_ctor: expected_network,
        }
    } else {
        CommonError::InvalidInstructionsString
    }
}

fn extract_error_from_error(
    err: ScryptoCompileError,
    expected_network: NetworkID,
) -> CommonError {
    use transaction::manifest::generator::GeneratorError::*;
    let n = expected_network;
    match err {
        ScryptoCompileError::GeneratorError(gen_err) => match gen_err {
            InvalidPackageAddress(a) => extract_error_from_addr(a, n),
            InvalidComponentAddress(a) => extract_error_from_addr(a, n),
            InvalidResourceAddress(a) => extract_error_from_addr(a, n),
            InvalidGlobalAddress(a) => extract_error_from_addr(a, n),
            _ => CommonError::InvalidInstructionsString,
        },
        _ => CommonError::InvalidInstructionsString,
    }
}

impl HasSampleValues for Instructions {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_simulator_other()
    }
}

impl Instructions {
    pub(crate) fn sample_mainnet_instructions_string() -> String {
        include_str!("resource_transfer.rtm").to_owned()
    }

    pub fn sample_mainnet() -> Self {
        Self::new(
            Self::sample_mainnet_instructions_string(),
            NetworkID::Mainnet,
        )
        .expect("Valid sample value")
    }

    // https://github.com/radixdlt/radix-engine-toolkit/blob/cf2f4b4d6de56233872e11959861fbf12db8ddf6/crates/radix-engine-toolkit/tests/manifests/account/multi_account_resource_transfer.rtm
    // but modified, changed `None` -> `Enum<0u8>()`, also changed `"account_a_bucket"` -> `"bucket1"`, `"account_b_bucket"` -> `"bucket2"`, etc.
    pub(crate) fn sample_other_simulator_instructions_string() -> String {
        include_str!("multi_account_resource_transfer.rtm").to_owned()
    }

    pub fn sample_simulator_other() -> Self {
        Self::new(
            Self::sample_other_simulator_instructions_string(),
            NetworkID::Simulator,
        )
        .expect("Valid sample value")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Instructions;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample_mainnet().network_id, NetworkID::Mainnet);
        assert_eq!(
            SUT::sample_simulator_other().network_id,
            NetworkID::Simulator
        );
    }

    #[test]
    fn new_from_instructions_string_wrong_network_id() {
        assert_eq!(
            SUT::new(
                SUT::sample_mainnet_instructions_string(),
                NetworkID::Stokenet
            ),
            Err(CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Mainnet,
                specified_to_instructions_ctor: NetworkID::Stokenet
            })
        );
    }
}
