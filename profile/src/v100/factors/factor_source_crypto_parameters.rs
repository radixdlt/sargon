use std::cell::RefCell;

use hd::derivation::derivation_path_scheme::DerivationPathScheme;
use identified_vec::{IsIdentifiedVec, IsIdentifiedVecOf, ItemsCloned};
use serde::{Deserialize, Serialize};
use wallet_kit_common::error::common_error::CommonError as Error;
use wallet_kit_common::types::keys::slip10_curve::SLIP10Curve;

use crate::identified_vec_via::IdentifiedVecVia;

/// Cryptographic parameters a certain FactorSource supports, e.g. which Elliptic Curves
/// it supports and which Hierarchical Deterministic (HD) derivations schemes it supports,
/// if any.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FactorSourceCryptoParameters {
    /// Describes with which Elliptic Curves a Factor Source can be used, e.g. a
    /// "Babylon" `DeviceFactorSource` is not capable of deriving keys on the curve
    /// `secp256k1` - only Olympia imported FactorSources can do that.
    ///
    /// Either `[curve25519]` or `[secp256k1, curve25519]`
    ///
    /// Must not be empty.
    supported_curves: RefCell<IdentifiedVecVia<SLIP10Curve>>,

    /// If not empty: Describes which kind of Hierarchical Deterministic (HD)
    /// derivations a FactorSource is capable of doing - if empty: the
    /// FactorSource does not support HD derivation.
    ///
    /// Either BIP44 or CAP26 (SLIP10)
    supported_derivation_path_schemes: RefCell<IdentifiedVecVia<DerivationPathScheme>>,
}

impl FactorSourceCryptoParameters {
    pub fn supported_curves(&self) -> Vec<SLIP10Curve> {
        self.supported_curves.borrow().items()
    }

    pub fn supported_derivation_path_schemes(&self) -> Vec<DerivationPathScheme> {
        self.supported_derivation_path_schemes.borrow().items()
    }
}

impl FactorSourceCryptoParameters {
    pub fn new<I, J>(curves: I, schemes: J) -> Result<Self, Error>
    where
        I: IntoIterator<Item = SLIP10Curve>,
        J: IntoIterator<Item = DerivationPathScheme>,
    {
        let supported_curves = IdentifiedVecVia::from_iter(curves);
        if supported_curves.len() == 0 {
            return Err(Error::FactorSourceCryptoParametersSupportedCurvesInvalidSize);
        }
        let supported_derivation_path_schemes = IdentifiedVecVia::from_iter(schemes);

        Ok(Self {
            supported_curves: RefCell::new(supported_curves),
            supported_derivation_path_schemes: RefCell::new(supported_derivation_path_schemes),
        })
    }

    pub fn babylon() -> Self {
        Self::new([SLIP10Curve::Curve25519], [DerivationPathScheme::Cap26])
            .expect("Valid Babylon parameters")
    }

    pub fn olympia() -> Self {
        Self::new(
            [SLIP10Curve::Secp256k1],
            [DerivationPathScheme::Bip44Olympia],
        )
        .expect("Valid Olympia parameters")
    }

    pub fn babylon_olympia_compatible() -> Self {
        Self::new(
            [SLIP10Curve::Curve25519, SLIP10Curve::Secp256k1],
            [
                DerivationPathScheme::Cap26,
                DerivationPathScheme::Bip44Olympia,
            ],
        )
        .expect("Valid Babylon and Olympia parameters")
    }
}

impl Default for FactorSourceCryptoParameters {
    fn default() -> Self {
        Self::babylon()
    }
}

#[cfg(test)]
mod tests {
    use hd::derivation::derivation_path_scheme::DerivationPathScheme;
    use wallet_kit_common::{
        json::assert_eq_after_json_roundtrip, types::keys::slip10_curve::SLIP10Curve,
    };

    use super::FactorSourceCryptoParameters;
    use wallet_kit_common::error::common_error::CommonError as Error;

    #[test]
    fn babylon_has_curve25519_as_first_curve() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon()
                .supported_curves()
                .first()
                .unwrap(),
            &SLIP10Curve::Curve25519
        );
    }

    #[test]
    fn default_is_babylon() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon(),
            FactorSourceCryptoParameters::default()
        );
    }

    #[test]
    fn babylon_has_cap26_as_first_derivation_path_scheme() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon()
                .supported_derivation_path_schemes()
                .first()
                .unwrap(),
            &DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn babylon_olympia_compat_has_curve25519_as_first_curve() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon_olympia_compatible()
                .supported_curves()
                .first()
                .unwrap(),
            &SLIP10Curve::Curve25519
        );
    }

    #[test]
    fn babylon_olympia_compat_has_cap26_as_first_derivation_path_scheme() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon_olympia_compatible()
                .supported_derivation_path_schemes()
                .first()
                .unwrap(),
            &DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn babylon_does_not_support_secp256k1() {
        assert!(!FactorSourceCryptoParameters::babylon()
            .supported_curves()
            .contains(&SLIP10Curve::Secp256k1));
    }

    #[test]
    fn babylon_does_not_support_bip44() {
        assert!(!FactorSourceCryptoParameters::babylon()
            .supported_derivation_path_schemes()
            .contains(&DerivationPathScheme::Bip44Olympia));
    }

    #[test]
    fn olympia_does_not_support_curve25519() {
        assert!(!FactorSourceCryptoParameters::olympia()
            .supported_curves()
            .contains(&SLIP10Curve::Curve25519));
    }

    #[test]
    fn olympia_does_not_support_cap26() {
        assert!(!FactorSourceCryptoParameters::olympia()
            .supported_derivation_path_schemes()
            .contains(&DerivationPathScheme::Cap26));
    }

    #[test]
    fn babylon_olympia_compat_has_supports_curve25519() {
        assert!(FactorSourceCryptoParameters::babylon_olympia_compatible()
            .supported_curves()
            .contains(&SLIP10Curve::Curve25519));
    }

    #[test]
    fn babylon_olympia_compat_supports_cap26() {
        assert!(FactorSourceCryptoParameters::babylon_olympia_compatible()
            .supported_derivation_path_schemes()
            .contains(&DerivationPathScheme::Cap26));
    }

    #[test]
    fn curves_must_not_be_empty() {
        assert_eq!(
            FactorSourceCryptoParameters::new([], []),
            Err(Error::FactorSourceCryptoParametersSupportedCurvesInvalidSize)
        );
    }

    #[test]
    fn duplicate_curves_are_removed() {
        assert_eq!(
            FactorSourceCryptoParameters::new(
                [SLIP10Curve::Curve25519, SLIP10Curve::Curve25519],
                []
            )
            .unwrap()
            .supported_curves()
            .len(),
            1
        );
    }

    #[test]
    fn duplicate_derivation_paths_are_removed() {
        assert_eq!(
            FactorSourceCryptoParameters::new(
                [SLIP10Curve::Curve25519],
                [DerivationPathScheme::Cap26, DerivationPathScheme::Cap26]
            )
            .unwrap()
            .supported_derivation_path_schemes()
            .len(),
            1
        );
    }

    #[test]
    fn json_babylon() {
        let model = FactorSourceCryptoParameters::babylon();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
            "supportedCurves": ["curve25519"],
            "supportedDerivationPathSchemes": ["cap26"]
        }
        "#,
        );
    }

    #[test]
    fn json_olympia_compat() {
        let model = FactorSourceCryptoParameters::olympia();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
            "supportedCurves": ["secp256k1"],
            "supportedDerivationPathSchemes": ["bip44Olympia"]
        }
        "#,
        );
    }

    #[test]
    fn json_babylon_olympia_compat() {
        let model = FactorSourceCryptoParameters::babylon_olympia_compatible();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
            "supportedCurves": ["curve25519", "secp256k1"],
            "supportedDerivationPathSchemes": ["cap26", "bip44Olympia"]
        }
        "#,
        );
    }
}
