use crate::prelude::*;

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    derive_more::Display,
    PartialOrd,
    Ord,
    Hash,
    uniffi::Record,
)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
#[display("{value}")]
pub struct AppearanceID {
    pub value: u8,
}

impl AppearanceID {
    /// The number of different appearances
    pub const MAX: u8 = 11;
    pub fn new(value: u8) -> Result<Self> {
        if value > Self::MAX {
            return Err(CommonError::InvalidAppearanceID { bad_value: value });
        }
        Ok(Self { value })
    }

    pub fn from_number_of_accounts_on_network(n: usize) -> Self {
        Self::new((n % ((Self::MAX + 1) as usize)) as u8).unwrap()
    }

    // Probably want this as a macro... but it is just not worth it, why I boilerplate it.
    fn declare(value: u8) -> Self {
        Self::new(value).expect("Should have declared valid value.")
    }

    pub fn all() -> Vec<Self> {
        (0..=Self::MAX).map(Self::declare).collect_vec()
    }
}

impl HasSampleValues for AppearanceID {
    fn sample() -> Self {
        *Self::all().first().unwrap()
    }
    fn sample_other() -> Self {
        *Self::all().iter().last().unwrap()
    }
}

impl Default for AppearanceID {
    fn default() -> Self {
        Self::new(0).expect("AppearanceID of 0 to be valid")
    }
}

impl TryFrom<u8> for AppearanceID {
    type Error = crate::CommonError;

    fn try_from(value: u8) -> Result<Self> {
        AppearanceID::new(value)
    }
}
impl From<AppearanceID> for u8 {
    fn from(value: AppearanceID) -> Self {
        value.value
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AppearanceID;

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
    fn len_of_all_is_max_plus_one() {
        assert_eq!(SUT::all().len(), SUT::MAX as usize + 1);
    }

    #[test]
    fn test_from_number_of_accounts() {
        assert_eq!(SUT::from_number_of_accounts_on_network(12), SUT::sample());
        assert_eq!(
            SUT::from_number_of_accounts_on_network(23),
            SUT::sample_other()
        );
    }

    #[test]
    fn lowest() {
        assert!(SUT::new(0).is_ok());
    }

    #[test]
    fn highest() {
        assert!(SUT::new(11).is_ok());
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::new(11).unwrap()), "11");
    }

    #[test]
    fn err_too_big() {
        assert_eq!(
            SUT::new(12),
            Err(CommonError::InvalidAppearanceID { bad_value: 12 })
        );
    }

    #[test]
    fn try_from() {
        assert_eq!(
            SUT::try_from(250),
            Err(CommonError::InvalidAppearanceID { bad_value: 250 })
        );
        assert_eq!(SUT::try_from(1), SUT::new(1));
    }

    #[test]
    fn json() {
        assert_json_value_eq_after_roundtrip(&SUT::new(3).unwrap(), json!(3));
        assert_json_value_fails::<SUT>(json!("3"));
        assert_json_value_fails::<SUT>(json!(99));
    }

    #[test]
    fn presets() {
        let set = appearance_ids_all()
            .into_iter()
            .map(|a| a.value)
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), (SUT::MAX as usize) + 1);
    }
}
