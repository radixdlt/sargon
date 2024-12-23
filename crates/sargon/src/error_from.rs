use crate::prelude::*;
pub trait FromAddressError {
    fn from_address_error(s: String, expected_network: NetworkID) -> Self;
}

impl FromAddressError for CommonError {
    fn from_address_error(s: String, expected_network: NetworkID) -> Self {
        use radix_engine_toolkit::functions::address::decode as RET_decode_address;
        let Some(Some(network_id)) = RET_decode_address(&s)
            .map(|t| t.0)
            .map(NetworkID::from_repr)
        else {
            return CommonError::InvalidInstructionsString {
                underlying: "Failed to get NetworkID from address".to_owned(),
            };
        };
        if network_id != expected_network {
            CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: network_id,
                specified_to_instructions_ctor: expected_network,
            }
        } else {
            CommonError::InvalidInstructionsString {
                underlying: "Failed to determine why an address was invalid"
                    .to_owned(),
            }
        }
    }
}

pub trait FromScryptoCompileError {
    fn from_scrypto_compile_error(
        err: ScryptoCompileError,
        expected_network: NetworkID,
    ) -> Self;
}

impl FromScryptoCompileError for CommonError {
    fn from_scrypto_compile_error(
        err: ScryptoCompileError,
        expected_network: NetworkID,
    ) -> Self {
        use radix_transactions::manifest::parser::ParserError;
        use radix_transactions::manifest::parser::ParserErrorKind::*;
        use GeneratorError;
        use GeneratorErrorKind::*;
        let n = expected_network;
        match err {
            ScryptoCompileError::GeneratorError(GeneratorError {
                error_kind: gen_err,
                ..
            }) => match gen_err {
                InvalidPackageAddress(a) => Self::from_address_error(a, n),
                InvalidResourceAddress(a) => Self::from_address_error(a, n),
                InvalidGlobalAddress(a) => Self::from_address_error(a, n),
                _ => CommonError::InvalidInstructionsString {
                    underlying: format!("GeneratorError: {:?}", gen_err),
                },
            },
            ScryptoCompileError::ParserError(ParserError {
                error_kind: MaxDepthExceeded { max, .. },
                ..
            }) => CommonError::InvalidTransactionMaxSBORDepthExceeded {
                max: max as u16,
            },
            _ => CommonError::InvalidInstructionsString {
                underlying: format!("{:?}", err),
            },
        }
    }
}
