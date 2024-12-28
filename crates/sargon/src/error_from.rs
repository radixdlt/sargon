use crate::prelude::*;
pub trait FromAddressError {
    fn from_address_error(
        s: String,
        expected_network: NetworkID,
        fallback_underlying: String,
    ) -> Self;
}

impl FromAddressError for CommonError {
    fn from_address_error(
        s: String,
        expected_network: NetworkID,
        fallback_underlying: String,
    ) -> Self {
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
                found_in_instructions: network_id.to_string(),
                specified_to_instructions_ctor: expected_network.to_string(),
            }
        } else {
            CommonError::InvalidInstructionsString {
                underlying: fallback_underlying,
            }
        }
    }
}

pub trait FromScryptoCompileError {
    fn from_scrypto_compile_error(
        manifest_string: &str,
        err: ScryptoCompileError,
        expected_network: NetworkID,
    ) -> Self;
}

impl FromScryptoCompileError for CommonError {
    fn from_scrypto_compile_error(
        manifest_string: &str,
        err: ScryptoCompileError,
        expected_network: NetworkID,
    ) -> Self {
        use radix_transactions::manifest::parser::ParserError;
        use radix_transactions::manifest::parser::ParserErrorKind::*;
        use GeneratorError;
        use GeneratorErrorKind::*;
        let n = expected_network;

        let pretty_diagnostics = scrypto_compile_error_diagnostics(
            manifest_string,
            err.clone(),
            ScryptoCompileErrorDiagnosticsStyle::PlainText,
        );
        match err {
            ScryptoCompileError::GeneratorError(GeneratorError {
                error_kind: gen_err,
                ..
            }) => match gen_err {
                InvalidPackageAddress(a) => {
                    Self::from_address_error(a, n, pretty_diagnostics)
                }
                InvalidResourceAddress(a) => {
                    Self::from_address_error(a, n, pretty_diagnostics)
                }
                InvalidGlobalAddress(a) => {
                    Self::from_address_error(a, n, pretty_diagnostics)
                }
                _ => CommonError::InvalidInstructionsString {
                    underlying: pretty_diagnostics,
                },
            },
            ScryptoCompileError::ParserError(ParserError {
                error_kind: MaxDepthExceeded { max, .. },
                ..
            }) => CommonError::InvalidTransactionMaxSBORDepthExceeded {
                max: max as u16,
            },
            _ => CommonError::InvalidInstructionsString {
                underlying: pretty_diagnostics,
            },
        }
    }
}
