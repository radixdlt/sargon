use crate::prelude::*;

macro_rules! path_union {
    (
        $(
            #[doc = $expr: expr]
        )*
        $union_name: ident,
        $(
            $variant_name: ident,
            $variant_type: ty
        )+
    ) => {
        paste::paste! {
            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                PartialEq,
                EnumAsInner,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                derive_more::Debug,
                derive_more::Display,
            )]
            pub enum $union_name {
                $(
                    #[display("{value}")]
                    #[debug("{:?}", value)]
                    $variant_name { value: $variant_type },
                )+
            }

            impl $union_name {
                $(
                    pub fn [< $variant_name:snake >](path: $variant_type) -> Self {
                        Self::$variant_name { value: path }
                    }
                )+
            }

            impl FromStr for $union_name {
                type Err = CommonError;
                fn from_str(s: &str) -> Result<Self> {
                    Self::from_bip32_string(s)
                }
            }

            impl $union_name {

                pub fn to_hd_path(&self) -> HDPath {
                    match self {
                        $(
                            Self::$variant_name { value } => value.to_hd_path(),
                        )+
                    }
                }
            }
            impl From<$union_name> for HDPath {
                fn from(value: $union_name) -> Self {
                    value.to_hd_path()
                }
            }

            impl ToBIP32Str for $union_name {
                fn to_bip32_string(&self) -> String {
                    self.to_hd_path().to_bip32_string()
                }
                fn to_bip32_string_debug(&self) -> String {
                    self.to_hd_path().to_bip32_string_debug()
                }
            }

            impl FromBIP32Str for $union_name {
                fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
                    let s = s.as_ref();
                    Result::<Self>::Err(CommonError::InvalidBIP32Path { bad_value: s.to_owned() })
                    $(
                        .or($variant_type::from_bip32_string(s).map(Self::[< $variant_name:snake >]))
                    )+

                }
            }

            impl HasDerivationPathSchemeObjectSafe for $union_name {
                fn get_derivation_path_scheme(&self) -> DerivationPathScheme {
                    match self {
                        $(
                            Self::$variant_name { value } => value.get_derivation_path_scheme(),
                        )+
                    }
                }
            }

            $(
                impl From<$variant_type> for $union_name {
                    fn from(value: $variant_type) -> Self {
                        Self::$variant_name { value }
                    }
                }
            )+

            $(
                impl TryFrom<$union_name> for $variant_type {
                    type Error = CommonError;
                    fn try_from(value: $union_name) -> Result<$variant_type> {
                        value.
                        [< into_ $variant_name:snake >]().map_err(|e| CommonError::InvalidPath { bad_value: format!("Path conversion fail: {:?}", e)})
                    }
                }
            )+


            impl<'de> serde::Deserialize<'de> for $union_name {
                fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result< $union_name, D::Error> {
                    #[derive(serde::Deserialize)]
                    pub struct Inner {
                        pub scheme: DerivationPathScheme,
                        pub path: serde_json::Value,
                    }

                    let inner = Inner::deserialize(d)?;
                    let scheme = inner.scheme;
                    let path = &inner.path;


                    $(
                        if scheme == $variant_type::derivation_path_scheme() && let Ok(success) = $variant_type::deserialize(path)
                                .map(Self::[< $variant_name:snake >]) {
                                    return Ok(success);
                        }
                    )+
                    return Err(serde::de::Error::custom("Fail"));


                }
            }

            impl serde::Serialize for $union_name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::*;
                    let mut state = serializer.serialize_struct("DerivationPath", 2)?;
                    state.serialize_field("scheme", &self.get_derivation_path_scheme())?;
                    state.serialize_field("path", &self.to_hd_path())?;
                    state.end()
                }
            }

        }
    };


}

pub trait HasIndex {
    fn index(&self) -> HDPathComponent;
}
impl<T: Clone + Into<HDPath>> HasIndex for T {
    fn index(&self) -> HDPathComponent {
        Into::<HDPath>::into(self.clone())
            .components()
            .last()
            .cloned()
            .expect("Path should never be empty")
    }
}

path_union!(
    DerivationPath,
    Account, AccountPath
    Identity, IdentityPath
    Bip44Like, BIP44LikePath
);

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

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = DerivationPath;

    #[test]
    fn test_to_bip32_string_is_display_account() {
        let sut = Sut::Account {
            value: AccountPath::sample(),
        };
        assert_eq!(sut.to_bip32_string(), format!("{}", sut));
    }

    #[test]
    fn test_to_bip32_string_is_debug_account() {
        let sut = Sut::Account {
            value: AccountPath::sample(),
        };
        assert_eq!(sut.to_bip32_string_debug(), format!("{:?}", sut));
    }

    #[test]
    fn test_to_bip32_string_is_display_identity() {
        let sut = Sut::Identity {
            value: IdentityPath::sample(),
        };
        assert_eq!(sut.to_bip32_string(), format!("{}", sut));
    }

    #[test]
    fn test_to_bip32_string_is_debug_identity() {
        let sut = Sut::Identity {
            value: IdentityPath::sample(),
        };
        assert_eq!(sut.to_bip32_string_debug(), format!("{:?}", sut));
    }

    #[test]
    fn string_roundtrip_account_from_account() {
        let value = AccountPath::sample();
        let s = value.to_bip32_string();
        let path2 = Sut::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Account { value }, path2);
    }

    #[test]
    fn string_roundtrip_account_from_cap26() {
        let sut = Sut::Account {
            value: AccountPath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = AccountPath::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Account { value }, sut)
    }

    #[test]
    fn string_roundtrip_identity_from_identity() {
        let value = IdentityPath::sample();
        let s = value.to_bip32_string();
        let path2 = Sut::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Identity { value }, path2);
    }

    #[test]
    fn string_roundtrip_identity_from_cap26() {
        let sut = Sut::Identity {
            value: IdentityPath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = IdentityPath::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Identity { value }, sut)
    }

    #[test]
    fn string_roundtrip_bip44_from_bip44() {
        let value = BIP44LikePath::sample();
        let s = value.to_bip32_string();
        let path2 = Sut::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Bip44Like { value }, path2);
    }

    #[test]
    fn string_roundtrip_getid_from_cap26() {
        let sut = Sut::Bip44Like {
            value: BIP44LikePath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = BIP44LikePath::from_bip32_string(&s).unwrap();
        assert_eq!(Sut::Bip44Like { value }, sut)
    }
}
/*
#[cfg(test)]
mod old_sargon_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DerivationPath;

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
    fn curve() {
        assert_eq!(SUT::sample().curve(), SLIP10Curve::Curve25519)
    }

    #[test]
    fn cap26_scheme() {
        assert_eq!(SUT::sample_cap26().scheme(), DerivationPathScheme::Cap26);
    }

    #[test]
    fn cap26_hdpath() {
        assert_eq!(
            SUT::sample_cap26().hd_path(),
            AccountPath::sample().hd_path()
        );
    }

    #[test]
    fn bip44like_scheme() {
        assert_eq!(
            SUT::BIP44Like {
                value: BIP44LikePath::new(0)
            }
            .scheme(),
            DerivationPathScheme::Bip44Olympia
        );
    }

    #[test]
    fn bip44like_hdpath() {
        assert_eq!(
            SUT::BIP44Like {
                value: BIP44LikePath::new(0)
            }
            .hd_path(),
            BIP44LikePath::new(0).hd_path()
        );
    }

    #[test]
    fn into_from_account_bip44_path() {
        assert_eq!(
            SUT::BIP44Like {
                value: BIP44LikePath::sample()
            },
            BIP44LikePath::sample().into()
        );
    }

    #[test]
    fn as_bip44_path() {
        let path: SUT = BIP44LikePath::sample().into();
        assert_eq!(path.as_bip44_like().unwrap(), &BIP44LikePath::sample());
    }

    #[test]
    fn into_from_account_cap26_path() {
        assert_eq!(
            SUT::CAP26 {
                value: AccountPath::sample().into()
            },
            AccountPath::sample().into()
        );
    }

    #[test]
    fn into_from_identity_cap26_path() {
        assert_eq!(
            SUT::CAP26 {
                value: IdentityPath::sample().into()
            },
            IdentityPath::sample().into()
        );
    }

    #[test]
    fn derivation_path_identity() {
        let derivation_path: SUT = IdentityPath::sample().into();
        assert_eq!(derivation_path, derivation_path.derivation_path());
    }

    #[test]
    fn try_from_hdpath_account() {
        let derivation_path: SUT = AccountPath::sample().into();
        let hd_path = derivation_path.hd_path();
        assert_eq!(SUT::try_from(hd_path), Ok(derivation_path));
    }

    #[test]
    fn try_from_hdpath_identity() {
        let derivation_path: SUT = IdentityPath::sample().into();
        let hd_path = derivation_path.hd_path();
        assert_eq!(SUT::try_from(hd_path), Ok(derivation_path));
    }

    #[test]
    fn try_from_hdpath_bip44() {
        let derivation_path: SUT = BIP44LikePath::sample().into();
        let hd_path = derivation_path.hd_path();
        assert_eq!(SUT::try_from(hd_path), Ok(derivation_path));
    }

    #[test]
    fn try_from_hdpath_getid() {
        let derivation_path: SUT = GetIDPath::default().into();
        let hd_path = derivation_path.hd_path();
        assert_eq!(SUT::try_from(hd_path), Ok(derivation_path));
    }

    #[test]
    fn into_from_getid_path() {
        assert_eq!(
            SUT::CAP26 {
                value: GetIDPath::default().into()
            },
            GetIDPath::default().into()
        );
    }

    #[test]
    fn json_cap26_account() {
        let model = SUT::sample();
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
    fn test_from_str_bip44() {
        let s = "m/44H/1022H/0H/0/0H";
        assert_eq!(SUT::from_str(s).unwrap(), BIP44LikePath::sample().into())
    }

    #[test]
    fn test_from_str_cap26_account_path() {
        let s = "m/44H/1022H/1H/525H/1460H/0H";
        assert_eq!(SUT::from_str(s).unwrap(), AccountPath::sample().into())
    }

    #[test]
    fn display() {
        let model = SUT::sample();
        assert_eq!(format!("{}", model), "m/44H/1022H/1H/525H/1460H/0H")
    }

    #[test]
    fn debug() {
        let model = SUT::sample();
        assert_eq!(format!("{:?}", model), "m/44H/1022H/1H/525H/1460H/0H")
    }

    #[test]
    fn json_cap26_getid() {
        let path = GetIDPath::default();
        let model: SUT = path.into();
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
        let model: SUT = path.into();
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

    #[test]
    fn json_android_bug_bip44like_incorrectly_marked_as_cap26_is_indeed_deserialized_as_bip44(
    ) {
        let json = r#"
        {
            "scheme": "cap26",
            "path": "m/44H/1022H/0H/0/0H"
        }
        "#;
        let sut = serde_json::from_str::<SUT>(json).unwrap();
        assert_eq!(
            sut,
            SUT::BIP44Like {
                value: BIP44LikePath::sample()
            }
        );
    }
}
 */
