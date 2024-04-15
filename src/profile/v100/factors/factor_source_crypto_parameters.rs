use crate::prelude::*;

/// Cryptographic parameters a certain FactorSource supports, e.g. which Elliptic Curves
/// it supports and which Hierarchical Deterministic (HD) derivations schemes it supports,
/// if any.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct FactorSourceCryptoParameters {
    /// Describes with which Elliptic Curves a Factor Source can be used, e.g. a
    /// "Babylon" `DeviceFactorSource` is not capable of deriving keys on the curve
    /// `secp256k1` - only Olympia imported FactorSources can do that.
    ///
    /// Either `[curve25519]` or `[secp256k1, curve25519]`
    ///
    /// Must not be empty.
    pub supported_curves: IdentifiedVecVia<SLIP10Curve>,

    /// If not empty: Describes which kind of Hierarchical Deterministic (HD)
    /// derivations a FactorSource is capable of doing - if empty: the
    /// FactorSource does not support HD derivation.
    ///
    /// Either BIP44 or CAP26 (SLIP10)
    pub supported_derivation_path_schemes:
        IdentifiedVecVia<DerivationPathScheme>,
}

impl FactorSourceCryptoParameters {
    #[cfg(not(tarpaulin_include))] // false negative
    pub fn new<I, J>(curves: I, schemes: J) -> Result<Self>
    where
        I: IntoIterator<Item = SLIP10Curve>,
        J: IntoIterator<Item = DerivationPathScheme>,
    {
        let supported_curves = IdentifiedVecVia::from_iter(curves);
        if supported_curves.is_empty() {
            return Err(CommonError::FactorSourceCryptoParametersSupportedCurvesInvalidSize);
        }
        let supported_derivation_path_schemes =
            IdentifiedVecVia::from_iter(schemes);

        Ok(Self {
            supported_curves,
            supported_derivation_path_schemes,
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

    pub fn supports_babylon(&self) -> bool {
        self.supported_curves.contains(&SLIP10Curve::Curve25519)
            && self
                .supported_derivation_path_schemes
                .contains(&DerivationPathScheme::Cap26)
    }
}

impl Default for FactorSourceCryptoParameters {
    fn default() -> Self {
        Self::babylon()
    }
}

impl HasSampleValues for SupportedCurves {
    fn sample() -> Self {
        SupportedCurves::just(SLIP10Curve::Curve25519)
    }

    fn sample_other() -> Self {
        SupportedCurves::from_iter([
            SLIP10Curve::Curve25519,
            SLIP10Curve::Secp256k1,
        ])
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn babylon_has_curve25519_as_first_curve() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon().supported_curves[0],
            SLIP10Curve::Curve25519
        );
    }

    #[test]
    fn default_is_babylon() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon(),
            FactorSourceCryptoParameters::default()
        );
        assert!(FactorSourceCryptoParameters::babylon().supports_babylon());
    }

    #[test]
    fn babylon_has_cap26_as_first_derivation_path_scheme() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon()
                .supported_derivation_path_schemes
                .first()
                .unwrap(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn babylon_olympia_compat_has_curve25519_as_first_curve() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon_olympia_compatible()
                .supported_curves
                .first()
                .unwrap(),
            SLIP10Curve::Curve25519
        );
    }

    #[test]
    fn babylon_olympia_compat_has_cap26_as_first_derivation_path_scheme() {
        assert_eq!(
            FactorSourceCryptoParameters::babylon_olympia_compatible()
                .supported_derivation_path_schemes
                .first()
                .unwrap(),
            DerivationPathScheme::Cap26
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
    fn olympia_does_not_support_curve25519() {
        assert!(!FactorSourceCryptoParameters::olympia()
            .supported_curves
            .contains(&SLIP10Curve::Curve25519));
    }

    #[test]
    fn olympia_does_not_support_cap26() {
        assert!(!FactorSourceCryptoParameters::olympia()
            .supported_derivation_path_schemes
            .contains(&DerivationPathScheme::Cap26));
    }

    #[test]
    fn babylon_olympia_compat_has_supports_curve25519() {
        assert!(FactorSourceCryptoParameters::babylon_olympia_compatible()
            .supported_curves
            .contains(&SLIP10Curve::Curve25519));
    }

    #[test]
    fn babylon_olympia_compat_supports_cap26() {
        assert!(FactorSourceCryptoParameters::babylon_olympia_compatible()
            .supported_derivation_path_schemes
            .contains(&DerivationPathScheme::Cap26));
    }

    #[test]
    fn curves_must_not_be_empty() {
        assert_eq!(
            FactorSourceCryptoParameters::new([], []),
            Err(CommonError::FactorSourceCryptoParametersSupportedCurvesInvalidSize)
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
