use crate::prelude::*;
use sargon::FactorSourceCryptoParameters as InternalFactorSourceCryptoParameters;

decl_identified_vec_of!(
    /// A collection of [`SLIP10Curve`]s that a factor source supports.
    /// MUST never be empty.
    SupportedCurves,
    SLIP10Curve
);

/// Cryptographic parameters a certain FactorSource supports, e.g. which Elliptic Curves
/// it supports and which Hierarchical Deterministic (HD) derivations schemes it supports,
/// if any.
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct FactorSourceCryptoParameters {
    /// Describes with which Elliptic Curves a Factor Source can be used, e.g. a
    /// "Babylon" `DeviceFactorSource` is not capable of deriving keys on the curve
    /// `secp256k1` - only Olympia imported FactorSources can do that.
    ///
    /// Either `[curve25519]` or `[secp256k1, curve25519]`
    ///
    /// Must not be empty.
    pub supported_curves: SupportedCurves,

    /// If not empty: Describes which kind of Hierarchical Deterministic (HD)
    /// derivations a FactorSource is capable of doing - if empty: the
    /// FactorSource does not support HD derivation.
    ///
    /// Either BIP44 or CAP26 (SLIP10)
    pub supported_derivation_path_schemes: Vec<DerivationPathScheme>,
}

impl From<InternalFactorSourceCryptoParameters> for FactorSourceCryptoParameters {
    fn from(value: InternalFactorSourceCryptoParameters) -> Self {
        Self {
            supported_curves: value.supported_curves.into(),
            supported_derivation_path_schemes: value.supported_derivation_path_schemes.into(),
        }
    }
}

impl Into<InternalFactorSourceCryptoParameters> for FactorSourceCryptoParameters {
    fn into(self) -> InternalFactorSourceCryptoParameters {
        InternalFactorSourceCryptoParameters {
            supported_curves: self.supported_curves.into(),
            supported_derivation_path_schemes: self.supported_derivation_path_schemes.into(),
        }
    }
}

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_sample(
) -> FactorSourceCryptoParameters {
    InternalFactorSourceCryptoParameters::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_sample_other(
) -> FactorSourceCryptoParameters {
    InternalFactorSourceCryptoParameters::sample_other().into()
}

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_preset_babylon_only(
) -> FactorSourceCryptoParameters {
    InternalFactorSourceCryptoParameters::babylon().into()
}

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_preset_olympia_only(
) -> FactorSourceCryptoParameters {
    InternalFactorSourceCryptoParameters::olympia().into()
}

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_preset_babylon_olympia_compatible(
) -> FactorSourceCryptoParameters {
    InternalFactorSourceCryptoParameters::babylon_olympia_compatible().into()
}

#[uniffi::export]
pub fn factor_source_crypto_parameters_supports_olympia(
    parameters: &FactorSourceCryptoParameters,
) -> bool {
    parameters.into_internal().supports_olympia()
}

#[uniffi::export]
pub fn factor_source_crypto_parameters_supports_babylon(
    parameters: &FactorSourceCryptoParameters,
) -> bool {
    parameters.into_internal().supports_babylon()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceCryptoParameters;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_crypto_parameters_sample(),
                new_factor_source_crypto_parameters_sample_other(),
                new_factor_source_crypto_parameters_preset_babylon_olympia_compatible(),
                // duplicates should get removed
                new_factor_source_crypto_parameters_preset_babylon_only(), // same as sample
                new_factor_source_crypto_parameters_preset_olympia_only(), // same as sample_other
            ])
            .len(),
            3
        );
    }

    #[test]
    fn test_supports_babylon() {
        assert!(factor_source_crypto_parameters_supports_babylon(
            &new_factor_source_crypto_parameters_preset_babylon_only()
        ))
    }

    #[test]
    fn test_supports_olympia() {
        assert!(factor_source_crypto_parameters_supports_olympia(
            &new_factor_source_crypto_parameters_preset_olympia_only()
        ))
    }
}
