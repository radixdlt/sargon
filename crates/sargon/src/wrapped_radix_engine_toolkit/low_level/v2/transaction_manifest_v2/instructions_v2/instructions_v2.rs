use crate::prelude::*;

use radix_common::prelude::MANIFEST_SBOR_V1_MAX_DEPTH;
use radix_transactions::manifest::CallMethod;

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display)]
#[display("{}", self.instructions_string())]
pub struct InstructionsV2 {
    pub instructions: Vec<ScryptoInstructionV2>, // MUST be first prop, else you break build.
    pub network_id: NetworkID,
}

impl Deref for InstructionsV2 {
    type Target = Vec<ScryptoInstructionV2>;

    fn deref(&self) -> &Self::Target {
        self.instructions()
    }
}

#[cfg(test)]
impl InstructionsV2 {
    /// For tests only, does not validate the SBOR depth of the instructions.
    pub(crate) fn new_unchecked(
        instructions: Vec<ScryptoInstructionV2>,
        network_id: NetworkID,
    ) -> Self {
        Self {
            instructions,
            network_id,
        }
    }
}

impl InstructionsV2 {
    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstructionV2> {
        self.deref()
    }
}

impl TryFrom<(&Vec<ScryptoInstructionV2>, NetworkID)> for InstructionsV2 {
    type Error = CommonError;

    fn try_from(
        value: (&Vec<ScryptoInstructionV2>, NetworkID),
    ) -> Result<Self, CommonError> {
        let scrypto = value.0;
        let network_id = value.1;

        // Verify that the instructions has acceptable depth and are compatible
        _ = instructions_string_from(scrypto, network_id)?;

        Ok(Self {
            instructions: ScryptoInstructionsV2(scrypto.to_owned().into()).0.to_vec(),
            network_id,
        })
    }
}

fn instructions_string_from(
    scrypto_instructions: &Vec<ScryptoInstructionV2>,
    network_id: NetworkID,
) -> Result<String, CommonError> {
    let scrypto_manifest = ScryptoTransactionManifestV2 {
        instructions: scrypto_instructions.clone(),
        blobs: Default::default(),
        children: Default::default(),
        object_names: Default::default(),
    };
    manifest_v2_string_from(scrypto_manifest, network_id)
}

impl InstructionsV2 {
    pub fn instructions_string(&self) -> String {
        instructions_string_from(self.instructions(), self.network_id).expect("Should never fail, because should never have allowed invalid instructions")
    }

    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
    ) -> Result<Self> {
        let network_definition = network_id.network_definition();
        let blob_provider = ScryptoMockBlobProvider::new();

        scrypto_compile_manifest(
            instructions_string.as_ref(),
            &network_definition,
            blob_provider,
        )
        .map_err(|e| CommonError::from_scrypto_compile_error(e, network_id))
        .and_then(|manifest: ScryptoTransactionManifestV2| {
            Self::try_from((manifest.instructions.as_ref(), network_id))
        })
    }
}

impl InstructionsV2 {
    pub fn new_from_byte_instructions(
        byte_instructions: Vec<u8>,
        network_id: NetworkID,
    ) -> Result<Self> {
        let instructions = RET_from_payload_bytes_instructions_v2(&byte_instructions)
            .map_err(|e| {
                let err_msg = format!("{:?}", e);
                error!("{}", err_msg);
                CommonError::FailedToDecodeBytesToManifestInstructions.into()
            })?;
        Ok(Self {
            instructions,
            network_id,
        })
    }

    pub fn instructions_as_bytes(&self) -> Vec<u8> {
        RET_to_payload_bytes_instructions_v2(self.instructions())
            .map(|b| b.into())
            .expect("to never fail")
    }
}

#[cfg(test)]
impl InstructionsV2 {
    /// Utility function which uses `InstructionsV2::new(<string>, <network_id>)`
    /// and SHOULD return `Err` if `depth > Instructions::MAX_SBOR_DEPTH`, which
    /// we can assert in unit tests.
    pub(crate) fn test_with_sbor_depth(
        depth: usize,
        network_id: NetworkID,
    ) -> Result<Self> {
        let nested_value = manifest_value_with_sbor_depth(depth);
        let dummy_address =
            ComponentAddress::with_node_id_bytes(&[0xffu8; 29], network_id);
        let instruction = ScryptoInstructionV2::CallMethod(CallMethod {
            address: TryInto::<ScryptoDynamicComponentAddress>::try_into(
                &dummy_address,
            )
            .unwrap()
            .into(),
            method_name: "dummy".to_owned(),
            args: nested_value,
        });
        instructions_string_from(&vec![instruction], network_id)
            .and_then(|x: String| Self::new(x, network_id))
    }

    pub(crate) const MAX_SBOR_DEPTH: usize = MANIFEST_SBOR_V1_MAX_DEPTH - 3;
}

impl HasSampleValues for InstructionsV2 {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_simulator_other()
    }
}

impl InstructionsV2 {
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            instructions: Vec::new(),
            network_id,
        }
    }
}

impl InstructionsV2 {
    pub(crate) fn sample_mainnet_instructions_string() -> String {
        include_str!(concat!(env!("FIXTURES_TX"), "resource_transfer.rtm"))
            .to_owned()
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
            Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
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
            Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
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
        include_str!(concat!(
            env!("FIXTURES_TX"),
            "multi_account_resource_transfer.rtm"
        ))
        .to_owned()
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
    use radix_transactions::manifest::{
        DropAuthZoneProofs, DropAuthZoneRegularProofs,
    };

    #[allow(clippy::upper_case_acronyms)]
    type SUT = InstructionsV2;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
        assert_eq!(
            SUT::sample_mainnet_without_lock_fee(),
            SUT::sample_mainnet_without_lock_fee()
        );
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
    fn new_from_instructions_string() {
        assert!(SUT::new(
            SUT::sample_mainnet_instructions_string(),
            NetworkID::Mainnet
        )
        .is_ok());
    }

    #[test]
    fn extract_error_from_addr_fallbacks_to_invalid_ins_err() {
        assert_eq!(
            CommonError::from_address_error(
                "foo".to_owned(),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsString {
                underlying: "Failed to get NetworkID from address".to_owned()
            }
        );
    }
    #[test]
    fn extract_error_from_addr_uses_invalid_instructions_string_if_same_network(
    ) {
        assert_eq!(
            CommonError::from_address_error("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr".to_owned(), NetworkID::Mainnet),
            CommonError::InvalidInstructionsString { underlying: "Failed to determine why an address was invalid".to_owned() }
        );
    }

    #[test]
    fn extract_error_from_error_non_gen_err() {
        assert_eq!(
            CommonError::from_scrypto_compile_error(
                ScryptoCompileError::LexerError(LexerError {
                    error_kind: LexerErrorKind::UnexpectedEof,
                    span: Span {
                        start: Position {
                            full_index: 0,
                            line_idx: 0,
                            line_char_index: 0
                        },
                        end: Position {
                            full_index: 0,
                            line_idx: 0,
                            line_char_index: 0
                        }
                    }
                }),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsString {
                underlying: "LexerError(LexerError { error_kind: UnexpectedEof, span: Span { start: Position { full_index: 0, line_idx: 0, line_char_index: 0 }, end: Position { full_index: 0, line_idx: 0, line_char_index: 0 } } })".to_owned()
            }
        );
    }

    #[test]
    fn from_scrypto() {
        let network_id = NetworkID::Mainnet;
        let instructions: &Vec<ScryptoInstructionV2> = &vec![
            ScryptoInstructionV2::DropAuthZoneProofs(DropAuthZoneProofs),
            ScryptoInstructionV2::DropAuthZoneRegularProofs(
                DropAuthZoneRegularProofs,
            ),
        ];
        assert_eq!(
            SUT {
                instructions: instructions.clone(),
                network_id
            },
            SUT::try_from((instructions, network_id)).unwrap()
        );
    }

    #[test]
    fn extract_error_from_error_gen_non_addr_err() {
        assert_eq!(
            CommonError::from_scrypto_compile_error(
                ScryptoCompileError::GeneratorError(GeneratorError {
                    error_kind: GeneratorErrorKind::BlobNotFound(
                        "dead".to_owned()
                    ),
                    span: Span {
                        start: Position {
                            full_index: 0,
                            line_idx: 0,
                            line_char_index: 0
                        },
                        end: Position {
                            full_index: 0,
                            line_idx: 0,
                            line_char_index: 0
                        }
                    }
                }),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsString {
                underlying: "GeneratorError: BlobNotFound(\"dead\")".to_owned()
            }
        );
    }

    #[test]
    fn extract_error_from_error_gen_err_package_addr() {
        assert_eq!(
            CommonError::from_scrypto_compile_error(
                ScryptoCompileError::GeneratorError(GeneratorError {
                    error_kind: GeneratorErrorKind::InvalidPackageAddress(
                        PackageAddress::sample().to_string()
                    ),
                    span: Span {
                        start: Position {
                            full_index: 0,
                            line_idx: 0,
                            line_char_index: 0
                        },
                        end: Position {
                            full_index: 0,
                            line_idx: 0,
                            line_char_index: 0
                        }
                    }
                }),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Mainnet,
                specified_to_instructions_ctor: NetworkID::Simulator
            }
        );
    }

    #[test]
    fn extract_error_from_error_gen_err_resource_addr() {
        assert_eq!(
            CommonError::from_scrypto_compile_error(
                ScryptoCompileError::GeneratorError(GeneratorError {
                    error_kind: GeneratorErrorKind::InvalidResourceAddress(
                        ResourceAddress::sample().to_string()
                    ),
                    span: Span {
                        start: Position {
                            full_index: 0,
                            line_idx: 0,
                            line_char_index: 0
                        },
                        end: Position {
                            full_index: 0,
                            line_idx: 0,
                            line_char_index: 0
                        }
                    }
                }),
                NetworkID::Simulator
            ),
            CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Mainnet,
                specified_to_instructions_ctor: NetworkID::Simulator
            }
        );
    }

    #[test]
    fn instructions_with_max_sbor_depth_is_ok() {
        assert!(SUT::test_with_sbor_depth(
            SUT::MAX_SBOR_DEPTH,
            NetworkID::Stokenet
        )
        .is_ok());
    }

    #[test]
    fn instructions_with_sbor_depth_greater_than_max_is_err() {
        assert_eq!(
            SUT::test_with_sbor_depth(
                SUT::MAX_SBOR_DEPTH + 1,
                NetworkID::Stokenet
            ),
            Err(CommonError::InvalidTransactionMaxSBORDepthExceeded {
                max: 20_u16
            })
        );
    }
}
