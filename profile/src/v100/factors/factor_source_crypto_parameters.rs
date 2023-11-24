use hierarchical_deterministic::derivation::{
    derivation_path_scheme::DerivationPathScheme, slip10_curve::SLIP10Curve,
};
use serde::{Deserialize, Serialize};
use wallet_kit_common::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FactorSourceCryptoParameters {
    /// SHOULD be non empty: Either `curve25519`` or `secp256k1`
    supported_curves: Vec<SLIP10Curve>,

    /// either BIP44 or CAP26 (SLIP10), empty if this factor source does not support HD derivation.
    supported_derivation_path_schemes: Vec<DerivationPathScheme>,
}

impl FactorSourceCryptoParameters {
    pub fn new<I, J>(curves: I, schemes: J) -> Result<Self, Error>
    where
        I: IntoIterator<Item = SLIP10Curve>,
        J: IntoIterator<Item = DerivationPathScheme>,
    {
        let mut supported_curves: Vec<SLIP10Curve> = curves.into_iter().collect();
        if supported_curves.is_empty() {
            return Err(Error::FactorSourceCryptoParametersSupportedCurvesInvalidSize);
        }
        supported_curves.dedup();
        let mut supported_derivation_path_schemes: Vec<DerivationPathScheme> =
            schemes.into_iter().collect();
        supported_derivation_path_schemes.dedup();

        Ok(Self {
            supported_curves,
            supported_derivation_path_schemes,
        })
    }

    pub fn babylon() -> Self {
        Self::new([SLIP10Curve::Curve25519], [DerivationPathScheme::Cap26])
            .expect("Valid Babylon parameters")
    }

    pub fn olympia_compatible() -> Self {
        Self::new(
            [SLIP10Curve::Secp256k1, SLIP10Curve::Curve25519],
            [
                DerivationPathScheme::Bip44Olympia,
                DerivationPathScheme::Cap26,
            ],
        )
        .expect("Valid Babylon parameters")
    }
}

#[cfg(test)]
mod tests {
    use hierarchical_deterministic::derivation::{
        derivation_path_scheme::DerivationPathScheme, slip10_curve::SLIP10Curve,
    };
    use wallet_kit_common::{error::Error, json::assert_eq_after_json_roundtrip};

    use super::FactorSourceCryptoParameters;

    #[test]
    fn babylon_has_curve25519_as_first_curve() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon()
                .supported_curves
                .first()
                .unwrap(),
            &SLIP10Curve::Curve25519
        );
    }

    #[test]
    fn babylon_has_cap26_as_first_derivation_path_scheme() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon()
                .supported_derivation_path_schemes
                .first()
                .unwrap(),
            &DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn olympia_compat_has_secp256k1_as_first_curve() {
        assert_eq!(
            FactorSourceCryptoParameters::olympia_compatible()
                .supported_curves
                .first()
                .unwrap(),
            &SLIP10Curve::Secp256k1
        );
    }

    #[test]
    fn olympia_compat_has_bip44_as_first_derivation_path_scheme() {
        assert_eq!(
            FactorSourceCryptoParameters::olympia_compatible()
                .supported_derivation_path_schemes
                .first()
                .unwrap(),
            &DerivationPathScheme::Bip44Olympia
        );
    }

    #[test]
    fn babylon_does_not_support_secp256k1() {
        assert!(!FactorSourceCryptoParameters::babylon()
            .supported_curves
            .contains(&SLIP10Curve::Secp256k1));
    }

    #[test]
    fn babylon_does_not_support_bip44() {
        assert!(!FactorSourceCryptoParameters::babylon()
            .supported_derivation_path_schemes
            .contains(&DerivationPathScheme::Bip44Olympia));
    }

    #[test]
    fn olympia_compat_has_supports_curve25519() {
        assert!(FactorSourceCryptoParameters::olympia_compatible()
            .supported_curves
            .contains(&SLIP10Curve::Curve25519));
    }

    #[test]
    fn olympia_compat_supports_cap26() {
        assert!(FactorSourceCryptoParameters::olympia_compatible()
            .supported_derivation_path_schemes
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
            .supported_curves
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
            .supported_derivation_path_schemes
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
        let model = FactorSourceCryptoParameters::olympia_compatible();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
            "supportedCurves": ["secp256k1", "curve25519"],
            "supportedDerivationPathSchemes": ["bip44Olympia", "cap26"]
        }
        "#,
        );
    }
}
