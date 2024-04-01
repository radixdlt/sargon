use crate::prelude::*;

use radix_engine::types::MANIFEST_SBOR_V1_MAX_DEPTH;
use radix_engine_toolkit::functions::address::decode as RET_decode_address;

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display, uniffi::Record)]
#[display("{}", self.instructions_string())]
pub struct Instructions {
    secret_magic: InstructionsSecretMagic, // MUST be first prop, else you break build.
    pub network_id: NetworkID,
}

impl Deref for Instructions {
    type Target = Vec<ScryptoInstruction>;

    fn deref(&self) -> &Self::Target {
        self.secret_magic.instructions()
    }
}

#[cfg(test)]
impl Instructions {
    /// For tests only, does not validate the SBOR depth of the instructions.
    pub(crate) fn new_unchecked(
        instructions: Vec<ScryptoInstruction>,
        network_id: NetworkID,
    ) -> Self {
        Self {
            secret_magic: InstructionsSecretMagic::new(instructions),
            network_id,
        }
    }
}

impl Instructions {
    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstruction> {
        self.deref()
    }
}

impl TryFrom<(&[ScryptoInstruction], NetworkID)> for Instructions {
    type Error = CommonError;

    fn try_from(
        value: (&[ScryptoInstruction], NetworkID),
    ) -> Result<Self, CommonError> {
        let scrypto = value.0;
        let network_id = value.1;

        // Verify that the instructions has acceptable depth and are compatible
        _ = instructions_string_from(scrypto, network_id)?;

        Ok(Self {
            secret_magic: InstructionsSecretMagic::from(ScryptoInstructions(
                scrypto.to_owned(),
            )),
            network_id,
        })
    }
}

fn instructions_string_from(
    scrypto_instructions: &[ScryptoInstruction],
    network_id: NetworkID,
) -> Result<String, CommonError> {
    let network_definition = network_id.network_definition();
    scrypto_decompile(scrypto_instructions, &network_definition).map_err(|e| {
        CommonError::InvalidInstructionsFailedToDecompile {
            underlying: format!("{:?}", e),
        }
    })
}

impl Instructions {
    pub fn instructions_string(&self) -> String {
        instructions_string_from(self.secret_magic.instructions().as_ref(), self.network_id).expect("Should never fail, because should never have allowed invalid instructions")
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
        .and_then(|manifest| {
            Self::try_from((manifest.instructions.as_ref(), network_id))
        })
    }
}

#[cfg(test)]
impl Instructions {
    pub fn with_great_sbor_depth(depth: usize) -> Result<Self> {
        let network_id = NetworkID::Stokenet;
        let nested_value = manifest_value_with_sbor_depth(depth);
        let dummy_address = ComponentAddress::sample_stokenet();
        assert_eq!(dummy_address.network_id(), network_id);
        let instruction = ScryptoInstruction::CallMethod {
            address: TryInto::<ScryptoDynamicComponentAddress>::try_into(
                &dummy_address,
            )
            .unwrap()
            .into(),
            method_name: "dummy".to_owned(),
            args: nested_value,
        };
        scrypto_decompile(&[instruction], &network_id.network_definition())
            .map_err(|_| CommonError::Unknown)
            .and_then(|x: String| Self::new(x, network_id))
    }
}

#[cfg(test)]
#[test]
fn test_max() {
    assert!(Instructions::with_great_sbor_depth(
        MANIFEST_SBOR_V1_MAX_DEPTH - 3
    )
    .is_ok());
}

#[cfg(test)]
#[test]
fn test_err() {
    assert!(Instructions::with_great_sbor_depth(
        MANIFEST_SBOR_V1_MAX_DEPTH - 2
    )
    .is_err());
}

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
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            secret_magic: InstructionsSecretMagic::new(Vec::new()),
            network_id,
        }
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

    pub fn sample_mainnet_without_lock_fee() -> Self {
        Self::new(
            r#"
            CALL_METHOD
            Address("account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8")
            "withdraw"
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("1337")
        ;
        TAKE_FROM_WORKTOP
            Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
            Decimal("1337")
            Bucket("bucket1")
        ;
        CALL_METHOD
            Address("account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69")
            "try_deposit_or_abort"
            Bucket("bucket1")
            Enum<0u8>()
        ;
            "#,
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
    use super::*;

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
    fn empty() {
        let sut = SUT::empty(NetworkID::Simulator);
        assert_eq!(sut.network_id, NetworkID::Simulator);
        assert_eq!(sut.instructions_string(), "");
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

    #[test]
    fn extract_error_from_addr_fallbacks_to_invalid_ins_err() {
        assert_eq!(
            extract_error_from_addr("foo".to_owned(), NetworkID::Simulator),
            CommonError::InvalidInstructionsString
        );
    }
    #[test]
    fn extract_error_from_addr_uses_invalid_instructions_string_if_same_network(
    ) {
        assert_eq!(
            extract_error_from_addr("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease".to_owned(), NetworkID::Mainnet),
            CommonError::InvalidInstructionsString
        );
    }

    #[test]
    fn extract_error_from_error_non_gen_err() {
        assert_eq!(
            extract_error_from_error(
                ScryptoCompileError::LexerError(
                    transaction::manifest::lexer::LexerError::UnexpectedEof
                ),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsString
        );
    }

    #[test]
    fn from_scrypto() {
        let network_id = NetworkID::Mainnet;
        let instructions: &[ScryptoInstruction] = &[
            ScryptoInstruction::DropAuthZoneProofs,
            ScryptoInstruction::DropAuthZoneRegularProofs,
        ];
        assert_eq!(
            SUT {
                secret_magic: InstructionsSecretMagic::sample(),
                network_id
            },
            SUT::try_from((instructions, network_id)).unwrap()
        );
    }

    #[test]
    fn extract_error_from_error_gen_non_addr_err() {
        assert_eq!(
            extract_error_from_error(
                ScryptoCompileError::GeneratorError(transaction::manifest::generator::GeneratorError::BlobNotFound("dead".to_owned())),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsString
        );
    }

    #[test]
    fn extract_error_from_error_gen_err_package_addr() {
        assert_eq!(
            extract_error_from_error(
                ScryptoCompileError::GeneratorError(transaction::manifest::generator::GeneratorError::InvalidPackageAddress(PackageAddress::sample().to_string())),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsWrongNetwork { found_in_instructions: NetworkID::Mainnet, specified_to_instructions_ctor: NetworkID::Simulator }
        );
    }

    #[test]
    fn extract_error_from_error_gen_err_component_addr() {
        assert_eq!(
            extract_error_from_error(
                ScryptoCompileError::GeneratorError(transaction::manifest::generator::GeneratorError::InvalidComponentAddress(ComponentAddress::sample().to_string())),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsWrongNetwork { found_in_instructions: NetworkID::Mainnet, specified_to_instructions_ctor: NetworkID::Simulator }
        );
    }

    #[test]
    fn extract_error_from_error_gen_err_resource_addr() {
        assert_eq!(
            extract_error_from_error(
                ScryptoCompileError::GeneratorError(transaction::manifest::generator::GeneratorError::InvalidResourceAddress(ResourceAddress::sample().to_string())),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsWrongNetwork { found_in_instructions: NetworkID::Mainnet, specified_to_instructions_ctor: NetworkID::Simulator }
        );
    }
}
