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

                pub fn to_canonical_bip32_string(&self) -> String {
                    self.to_hd_path().to_canonical_bip32_string()
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
                    // This is a hack to fix a bug in the Android app where BIP44 paths
                    // were incorrectly marked cap26.
                    $(
                        if let Ok(success) = $variant_type::deserialize(path)
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

impl BIP44LikePath {
    pub fn network_id(&self) -> NetworkID {
        NetworkID::Mainnet
    }
}

impl IsKeySpaceAware for DerivationPath {
    fn key_space(&self) -> KeySpace {
        self.to_hd_path().index().key_space()
    }
}

impl IsNetworkAware for DerivationPath {
    fn network_id(&self) -> NetworkID {
        match self {
            Self::Account { value } => value.network_id(),
            Self::Identity { value } => value.network_id(),
            Self::Bip44Like { value } => value.network_id(),
        }
    }
}
impl HasKeyKindObjectSafe for DerivationPath {
    fn get_key_kind(&self) -> CAP26KeyKind {
        match self {
            Self::Account { value } => value.get_key_kind(),
            Self::Identity { value } => value.get_key_kind(),
            Self::Bip44Like { value: _ } => CAP26KeyKind::TransactionSigning,
        }
    }
}

impl HasEntityKindObjectSafe for DerivationPath {
    fn get_entity_kind(&self) -> CAP26EntityKind {
        match self {
            Self::Account { value } => value.get_entity_kind(),
            Self::Identity { value } => value.get_entity_kind(),
            Self::Bip44Like { value: _ } => CAP26EntityKind::Account,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DerivationPath;

    #[test]
    fn test_to_bip32_string_is_display_account() {
        let sut = SUT::Account {
            value: AccountPath::sample(),
        };
        assert_eq!(sut.to_bip32_string(), format!("{}", sut));
    }

    #[test]
    fn test_to_bip32_string_is_debug_account() {
        let sut = SUT::Account {
            value: AccountPath::sample(),
        };
        assert_eq!(sut.to_bip32_string_debug(), format!("{:?}", sut));
    }

    #[test]
    fn test_to_bip32_string_is_display_identity() {
        let sut = SUT::Identity {
            value: IdentityPath::sample(),
        };
        assert_eq!(sut.to_bip32_string(), format!("{}", sut));
    }

    #[test]
    fn test_to_bip32_string_is_debug_identity() {
        let sut = SUT::Identity {
            value: IdentityPath::sample(),
        };
        assert_eq!(sut.to_bip32_string_debug(), format!("{:?}", sut));
    }

    #[test]
    fn string_roundtrip_account_from_account() {
        let value = AccountPath::sample();
        let s = value.to_bip32_string();
        let path2 = SUT::from_bip32_string(&s).unwrap();
        assert_eq!(SUT::Account { value }, path2);
    }

    #[test]
    fn string_roundtrip_account_from_cap26() {
        let sut = SUT::Account {
            value: AccountPath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = AccountPath::from_bip32_string(&s).unwrap();
        assert_eq!(SUT::Account { value }, sut)
    }

    #[test]
    fn string_roundtrip_identity_from_identity() {
        let value = IdentityPath::sample();
        let s = value.to_bip32_string();
        let path2 = SUT::from_bip32_string(&s).unwrap();
        assert_eq!(SUT::Identity { value }, path2);
    }

    #[test]
    fn string_roundtrip_identity_from_cap26() {
        let sut = SUT::Identity {
            value: IdentityPath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = IdentityPath::from_bip32_string(&s).unwrap();
        assert_eq!(SUT::Identity { value }, sut)
    }

    #[test]
    fn string_roundtrip_bip44_from_bip44() {
        let value = BIP44LikePath::sample();
        let s = value.to_bip32_string();
        let path2 = SUT::from_bip32_string(&s).unwrap();
        assert_eq!(SUT::Bip44Like { value }, path2);
    }

    #[test]
    fn string_roundtrip_getid_from_cap26() {
        let sut = SUT::Bip44Like {
            value: BIP44LikePath::sample(),
        };
        let s = sut.to_bip32_string();
        let value = BIP44LikePath::from_bip32_string(&s).unwrap();
        assert_eq!(SUT::Bip44Like { value }, sut)
    }

    #[test]
    fn string_representation_of_canonical_and_non_canonical_for_securified_derivation_path(
    ) {
        let sut = SUT::Account {
            value: AccountPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::TransactionSigning,
                Hardened::from_local_key_space(U31::new(2), IsSecurified(true))
                    .unwrap(),
            ),
        };

        assert_eq!(sut.to_bip32_string(), "m/44H/1022H/1H/525H/1460H/2S");
        assert_eq!(
            sut.to_canonical_bip32_string(),
            "m/44H/1022H/1H/525H/1460H/1073741826H"
        )
    }

    #[test]
    fn string_representation_of_canonical_and_non_canonical_for_unsecurified_derivation_path(
    ) {
        let sut = SUT::Account {
            value: AccountPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::TransactionSigning,
                Hardened::from_local_key_space(U31::ZERO, IsSecurified(false))
                    .unwrap(),
            ),
        };

        assert_eq!(sut.to_bip32_string(), sut.to_canonical_bip32_string());
    }

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
        assert_eq!(
            SUT::sample().get_derivation_path_scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn cap26_hdpath() {
        assert_eq!(
            SUT::sample().to_hd_path(),
            AccountPath::sample().to_hd_path()
        );
    }

    #[test]
    fn bip44like_scheme() {
        assert_eq!(
            SUT::Bip44Like {
                value: BIP44LikePath::new(
                    HDPathComponent::from_global_key_space(0).unwrap()
                )
            }
            .get_derivation_path_scheme(),
            DerivationPathScheme::Bip44Olympia
        );
    }

    #[test]
    fn bip44like_hdpath() {
        assert_eq!(
            SUT::Bip44Like {
                value: BIP44LikePath::new(
                    HDPathComponent::from_global_key_space(0).unwrap()
                )
            }
            .to_hd_path(),
            BIP44LikePath::new(
                HDPathComponent::from_local_key_space(
                    0,
                    KeySpace::Unsecurified { is_hardened: false }
                )
                .unwrap()
            )
            .to_hd_path()
        );
    }

    #[test]
    fn into_from_account_bip44_path() {
        assert_eq!(
            SUT::Bip44Like {
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
            SUT::Account {
                value: AccountPath::sample()
            },
            AccountPath::sample().into()
        );
    }

    #[test]
    fn into_from_identity_cap26_path() {
        assert_eq!(
            SUT::Identity {
                value: IdentityPath::sample()
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
        let s = "m/44H/1022H/0H/0/1H";
        assert_eq!(
            SUT::from_str(s).unwrap(),
            BIP44LikePath::sample_other().into()
        )
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
        assert_eq!(format!("{:?}", model), "m/44'/1022'/1'/525'/1460'/0'")
    }

    #[test]
    fn json_bip44like_account_hardened() {
        let path = BIP44LikePath::sample_other();
        let model: SUT = path.into();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
            "scheme": "bip44Olympia",
            "path": "m/44H/1022H/0H/0/1H"
        }
        "#,
        );
    }

    #[test]
    fn invalid_json() {
        let json = r#"
        {
            "scheme": "so invalid",
            "path": "this is not a path"
        }
        "#;
        let sut = serde_json::from_str::<SUT>(json);
        assert!(sut.is_err());
    }

    #[test]
    fn json_bip44like_account_unhardened() {
        let path = BIP44LikePath::sample();
        let model: SUT = path.into();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
            "scheme": "bip44Olympia",
            "path": "m/44H/1022H/0H/0/0"
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
            "path": "m/44H/1022H/0H/0/1H"
        }
        "#;
        let sut = serde_json::from_str::<SUT>(json).unwrap();
        assert_eq!(
            sut,
            SUT::Bip44Like {
                value: BIP44LikePath::sample_other()
            }
        );
    }

    #[test]
    fn bip44_network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn bip44_get_entity_kind() {
        assert_eq!(SUT::sample().get_entity_kind(), CAP26EntityKind::Account);
    }
}
