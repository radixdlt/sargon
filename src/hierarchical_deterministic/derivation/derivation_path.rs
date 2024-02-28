use crate::prelude::*;

/// A derivation path on either supported schemes, either Babylon (CAP26) or Olympia (BIP44Like).
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumAsInner,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Enum,
)]
pub enum DerivationPath {
    #[debug("{}", self.bip32_string())]
    CAP26 { value: CAP26Path },
    #[debug("{}", self.bip32_string())]
    BIP44Like { value: BIP44LikePath },
}

impl TryFrom<&HDPath> for DerivationPath {
    type Error = CommonError;

    fn try_from(value: &HDPath) -> Result<Self> {
        if let Ok(bip44) = BIP44LikePath::try_from(value) {
            return Ok(bip44.into());
        };
        CAP26Path::try_from(value).map(|p| p.derivation_path())
    }
}

impl<'de> serde::Deserialize<'de> for DerivationPath {
    /// Tries to deserializes a JSON string as a bech32 address into an `AccountAddress`.
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        d: D,
    ) -> Result<DerivationPath, D::Error> {
        #[derive(Deserialize)]
        pub struct Inner {
            pub scheme: DerivationPathScheme,
            pub path: serde_json::Value,
        }

        let inner = Inner::deserialize(d)?;

        let derivation_path = match inner.scheme {
            DerivationPathScheme::Cap26 => DerivationPath::CAP26 {
                value: CAP26Path::deserialize(inner.path)
                    .map_err(serde::de::Error::custom)?,
            },
            DerivationPathScheme::Bip44Olympia => DerivationPath::BIP44Like {
                value: BIP44LikePath::deserialize(inner.path)
                    .map_err(serde::de::Error::custom)?,
            },
        };
        Ok(derivation_path)
    }
}

impl Serialize for DerivationPath {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DerivationPath", 2)?;
        state.serialize_field("scheme", &self.scheme())?;
        state.serialize_field("path", &self.hd_path())?;
        state.end()
    }
}

impl HasSampleValues for DerivationPath {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        AccountPath::sample().into()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        IdentityPath::sample().into()
    }
}

impl Derivation for DerivationPath {
    fn derivation_path(&self) -> DerivationPath {
        self.clone()
    }

    fn hd_path(&self) -> &HDPath {
        match self {
            DerivationPath::CAP26 { value } => value.hd_path(),
            DerivationPath::BIP44Like { value } => value.hd_path(),
        }
    }

    fn scheme(&self) -> DerivationPathScheme {
        match self {
            DerivationPath::CAP26 { value } => value.scheme(),
            DerivationPath::BIP44Like { value } => value.scheme(),
        }
    }
}

impl DerivationPath {
    pub fn sample_cap26() -> Self {
        DerivationPath::CAP26 {
            value: CAP26Path::sample_account(),
        }
    }
}

impl From<AccountPath> for DerivationPath {
    fn from(value: AccountPath) -> Self {
        Self::CAP26 {
            value: value.into(),
        }
    }
}

impl From<IdentityPath> for DerivationPath {
    fn from(value: IdentityPath) -> Self {
        Self::CAP26 {
            value: value.into(),
        }
    }
}

impl From<GetIDPath> for DerivationPath {
    fn from(value: GetIDPath) -> Self {
        Self::CAP26 {
            value: value.into(),
        }
    }
}

impl From<BIP44LikePath> for DerivationPath {
    fn from(value: BIP44LikePath) -> Self {
        Self::BIP44Like { value }
    }
}

impl From<CAP26Path> for DerivationPath {
    fn from(value: CAP26Path) -> Self {
        Self::CAP26 { value }
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(DerivationPath::sample(), DerivationPath::sample());
        assert_eq!(
            DerivationPath::sample_other(),
            DerivationPath::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(DerivationPath::sample(), DerivationPath::sample_other());
    }

    #[test]
    fn cap26_scheme() {
        assert_eq!(
            DerivationPath::sample_cap26().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn cap26_hdpath() {
        assert_eq!(
            DerivationPath::sample_cap26().hd_path(),
            AccountPath::sample().hd_path()
        );
    }

    #[test]
    fn bip44like_scheme() {
        assert_eq!(
            DerivationPath::BIP44Like {
                value: BIP44LikePath::new(0)
            }
            .scheme(),
            DerivationPathScheme::Bip44Olympia
        );
    }

    #[test]
    fn bip44like_hdpath() {
        assert_eq!(
            DerivationPath::BIP44Like {
                value: BIP44LikePath::new(0)
            }
            .hd_path(),
            BIP44LikePath::new(0).hd_path()
        );
    }

    #[test]
    fn into_from_account_bip44_path() {
        assert_eq!(
            DerivationPath::BIP44Like {
                value: BIP44LikePath::sample()
            },
            BIP44LikePath::sample().into()
        );
    }

    #[test]
    fn as_bip44_path() {
        let path: DerivationPath = BIP44LikePath::sample().into();
        assert_eq!(path.as_bip44_like().unwrap(), &BIP44LikePath::sample());
    }

    #[test]
    fn into_from_account_cap26_path() {
        assert_eq!(
            DerivationPath::CAP26 {
                value: AccountPath::sample().into()
            },
            AccountPath::sample().into()
        );
    }

    #[test]
    fn into_from_identity_cap26_path() {
        assert_eq!(
            DerivationPath::CAP26 {
                value: IdentityPath::sample().into()
            },
            IdentityPath::sample().into()
        );
    }

    #[test]
    fn derivation_path_identity() {
        let derivation_path: DerivationPath = IdentityPath::sample().into();
        assert_eq!(derivation_path, derivation_path.derivation_path());
    }

    #[test]
    fn try_from_hdpath_account() {
        let derivation_path: DerivationPath = AccountPath::sample().into();
        let hd_path = derivation_path.hd_path();
        assert_eq!(DerivationPath::try_from(hd_path), Ok(derivation_path));
    }

    #[test]
    fn try_from_hdpath_identity() {
        let derivation_path: DerivationPath = IdentityPath::sample().into();
        let hd_path = derivation_path.hd_path();
        assert_eq!(DerivationPath::try_from(hd_path), Ok(derivation_path));
    }

    #[test]
    fn try_from_hdpath_bip44() {
        let derivation_path: DerivationPath = BIP44LikePath::sample().into();
        let hd_path = derivation_path.hd_path();
        assert_eq!(DerivationPath::try_from(hd_path), Ok(derivation_path));
    }

    #[test]
    fn try_from_hdpath_getid() {
        let derivation_path: DerivationPath = GetIDPath::default().into();
        let hd_path = derivation_path.hd_path();
        assert_eq!(DerivationPath::try_from(hd_path), Ok(derivation_path));
    }

    #[test]
    fn from_cap26() {
        let derivation_path: DerivationPath = CAP26Path::Account {
            value: AccountPath::sample(),
        }
        .into();
        assert_eq!(
            derivation_path.derivation_path(),
            AccountPath::sample().derivation_path()
        )
    }

    #[test]
    fn as_cap26_path() {
        let path: DerivationPath = AccountPath::sample().into();
        assert_eq!(
            path.as_cap26().unwrap(),
            &CAP26Path::Account {
                value: AccountPath::sample()
            }
        );
    }

    #[test]
    fn into_from_getid_path() {
        assert_eq!(
            DerivationPath::CAP26 {
                value: GetIDPath::default().into()
            },
            GetIDPath::default().into()
        );
    }

    #[test]
    fn json_cap26_account() {
        let model = DerivationPath::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
			"scheme": "cap26",
			"path": "m/44H/1022H/1H/525H/1460H/0H"
		}
        "#,
        );
    }

    #[test]
    fn display() {
        let model = DerivationPath::sample();
        assert_eq!(format!("{}", model), "m/44H/1022H/1H/525H/1460H/0H")
    }

    #[test]
    fn debug() {
        let model = DerivationPath::sample();
        assert_eq!(format!("{:?}", model), "m/44H/1022H/1H/525H/1460H/0H")
    }

    #[test]
    fn json_cap26_getid() {
        let path = GetIDPath::default();
        let model: DerivationPath = path.into();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
			"scheme": "cap26",
			"path": "m/44H/1022H/365H"
		}
        "#,
        );
    }

    #[test]
    fn json_bip44like_account() {
        let path = BIP44LikePath::sample();
        let model: DerivationPath = path.into();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
			"scheme": "bip44Olympia",
			"path": "m/44H/1022H/0H/0/0H"
		}
        "#,
        );
    }
}
