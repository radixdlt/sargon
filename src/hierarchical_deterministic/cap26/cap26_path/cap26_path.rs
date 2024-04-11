use crate::prelude::*;

/// A derivation path design specifically for Radix Babylon wallets used by Accounts and Personas
/// to be unique per network with separate key spaces for Accounts/Identities (Personas) and key
/// kind: sign transaction or sign auth.
#[derive(
    Clone,
    Debug,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    uniffi::Enum,
)]
pub enum CAP26Path {
    #[display("{value}")]
    GetID { value: GetIDPath },
    #[display("{value}")]
    Account { value: AccountPath },
    #[display("{value}")]
    Identity { value: IdentityPath },
}

impl TryFrom<&HDPath> for CAP26Path {
    type Error = CommonError;

    fn try_from(value: &HDPath) -> Result<Self> {
        if let Ok(get_id) = GetIDPath::try_from(value) {
            return Ok(get_id.into());
        }
        if let Ok(identity_path) = IdentityPath::try_from(value) {
            return Ok(identity_path.into());
        }
        AccountPath::try_from(value).map(|p| p.into())
    }
}

impl FromStr for CAP26Path {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<HDPath>().and_then(|p| Self::try_from(&p))
    }
}

impl Derivation for CAP26Path {
    fn hd_path(&self) -> &HDPath {
        match self {
            CAP26Path::Account { value } => value.hd_path(),
            CAP26Path::Identity { value } => value.hd_path(),
            CAP26Path::GetID { value } => value.hd_path(),
        }
    }

    fn curve(&self) -> SLIP10Curve {
        self.scheme().curve()
    }

    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::CAP26 {
            value: self.clone(),
        }
    }
}

impl CAP26Path {
    pub fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Cap26
    }
}

impl From<AccountPath> for CAP26Path {
    fn from(value: AccountPath) -> Self {
        Self::Account { value }
    }
}

impl From<IdentityPath> for CAP26Path {
    fn from(value: IdentityPath) -> Self {
        Self::Identity { value }
    }
}

impl From<GetIDPath> for CAP26Path {
    fn from(value: GetIDPath) -> Self {
        Self::GetID { value }
    }
}

impl HasSampleValues for CAP26Path {
    fn sample() -> Self {
        Self::sample_account()
    }

    fn sample_other() -> Self {
        Self::sample_identity()
    }
}

impl CAP26Path {
    pub fn sample_account() -> Self {
        Self::Account {
            value: AccountPath::sample(),
        }
    }

    pub fn sample_identity() -> Self {
        Self::Identity {
            value: IdentityPath::sample(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(CAP26Path::sample(), CAP26Path::sample());
        assert_eq!(CAP26Path::sample_other(), CAP26Path::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(CAP26Path::sample(), CAP26Path::sample_other());
    }

    #[test]
    fn scheme_account_path() {
        assert_eq!(
            CAP26Path::sample_account().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn scheme_identity_path() {
        assert_eq!(
            CAP26Path::sample_identity().scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn scheme_getid_path() {
        assert_eq!(
            CAP26Path::GetID {
                value: GetIDPath::default()
            }
            .scheme(),
            DerivationPathScheme::Cap26
        );
    }

    #[test]
    fn hdpath_account_path() {
        assert_eq!(
            CAP26Path::sample_account().hd_path(),
            AccountPath::sample().hd_path()
        );
    }

    #[test]
    fn hdpath_getid_path() {
        assert_eq!(
            CAP26Path::GetID {
                value: GetIDPath::default()
            }
            .hd_path(),
            GetIDPath::default().hd_path()
        );
    }

    #[test]
    fn into_from_account_path() {
        assert_eq!(
            CAP26Path::Account {
                value: AccountPath::sample()
            },
            AccountPath::sample().into()
        );
    }

    #[test]
    fn into_from_getid_path() {
        assert_eq!(
            CAP26Path::GetID {
                value: GetIDPath::default()
            },
            GetIDPath::default().into()
        );
    }

    #[test]
    fn json_roundtrip_getid() {
        let model: CAP26Path = GetIDPath::default().into();
        assert_json_value_eq_after_roundtrip(&model, json!("m/44H/1022H/365H"));
    }

    #[test]
    fn json_roundtrip_account() {
        let model: CAP26Path = AccountPath::sample().into();
        assert_json_value_eq_after_roundtrip(
            &model,
            json!("m/44H/1022H/1H/525H/1460H/0H"),
        );
    }
}
