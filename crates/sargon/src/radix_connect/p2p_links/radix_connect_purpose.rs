use crate::prelude::*;

/// The purpose of the connection, set by the other client, typically Connector Extension or dApp.
/// As part of the initial linking flow, user will be prompted about kind of link they're trying to make.
/// The user needs to make a conscious decision about general purpose links (because it comes with security risk).
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
)]
#[serde(rename_all = "lowercase")]
pub enum RadixConnectPurpose {
    General,

    #[default]
    Unknown,
}

impl FromStr for RadixConnectPurpose {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        Self::new_from_json_string(s).map_err(|_| {
            CommonError::InvalidRadixConnectPurpose {
                bad_value: s.to_owned(),
            }
        })
    }
}

impl RadixConnectPurpose {
    pub fn from_str_default_value(s: &str) -> Self {
        RadixConnectPurpose::from_str(s).unwrap_or_default()
    }
}

impl std::fmt::Display for RadixConnectPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_json_string())
    }
}

impl HasSampleValues for RadixConnectPurpose {
    fn sample() -> Self {
        Self::General
    }

    fn sample_other() -> Self {
        Self::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixConnectPurpose;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn from_str_err() {
        let s = "invalid radix connect purpose kind!";
        assert_eq!(
            SUT::from_str(s),
            Err(CommonError::InvalidRadixConnectPurpose {
                bad_value: s.to_owned(),
            })
        );
    }

    #[test]
    fn from_str_default_value_general() {
        let s = "general";
        assert_eq!(SUT::from_str_default_value(s), SUT::General);
    }

    #[test]
    fn from_str_default_value_unknown() {
        let s = "unknown radix connect purpose kind!";
        assert_eq!(SUT::from_str_default_value(s), SUT::Unknown);
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([SUT::General, SUT::General].into_iter()).len(),
            1
        );
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&SUT::General, json!("general"));
        assert_json_value_eq_after_roundtrip(&SUT::Unknown, json!("unknown"));
    }
}