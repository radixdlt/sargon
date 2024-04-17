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
        Self {
            value: (n % (Self::MAX as usize)) as u8,
        }
    }

    // Probably want this as a macro... but it is just not worth it, why I boilerplate it.
    fn declare(value: u8) -> Self {
        Self::new(value).expect("Should have declared valid value.")
    }
    pub fn gradient0() -> Self {
        Self::declare(0)
    }
    pub fn gradient1() -> Self {
        Self::declare(1)
    }
    pub fn gradient2() -> Self {
        Self::declare(2)
    }
    pub fn gradient3() -> Self {
        Self::declare(3)
    }
    pub fn gradient4() -> Self {
        Self::declare(4)
    }
    pub fn gradient5() -> Self {
        Self::declare(5)
    }
    pub fn gradient6() -> Self {
        Self::declare(6)
    }
    pub fn gradient7() -> Self {
        Self::declare(7)
    }
    pub fn gradient8() -> Self {
        Self::declare(8)
    }
    pub fn gradient9() -> Self {
        Self::declare(9)
    }
    pub fn gradient10() -> Self {
        Self::declare(10)
    }
    pub fn gradient11() -> Self {
        Self::declare(11)
    }
}

impl HasSampleValues for AppearanceID {
    fn sample() -> Self {
        Self::gradient0()
    }
    fn sample_other() -> Self {
        Self::gradient11()
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

    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(AppearanceID::sample(), AppearanceID::sample());
        assert_eq!(AppearanceID::sample_other(), AppearanceID::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(AppearanceID::sample(), AppearanceID::sample_other());
    }

    #[test]
    fn lowest() {
        assert!(AppearanceID::new(0).is_ok());
    }

    #[test]
    fn highest() {
        assert!(AppearanceID::new(11).is_ok());
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", AppearanceID::new(11).unwrap()), "11");
    }

    #[test]
    fn err_too_big() {
        assert_eq!(
            AppearanceID::new(12),
            Err(CommonError::InvalidAppearanceID { bad_value: 12 })
        );
    }

    #[test]
    fn try_from() {
        assert_eq!(
            AppearanceID::try_from(250),
            Err(CommonError::InvalidAppearanceID { bad_value: 250 })
        );
        assert_eq!(AppearanceID::try_from(1), AppearanceID::new(1));
    }

    #[test]
    fn json() {
        assert_json_value_eq_after_roundtrip(
            &AppearanceID::new(3).unwrap(),
            json!(3),
        );
        assert_json_value_fails::<AppearanceID>(json!("3"));
        assert_json_value_fails::<AppearanceID>(json!(99));
    }

    #[test]
    fn presets() {
        let set = appearance_ids_all()
            .into_iter()
            .map(|a| a.value)
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), (AppearanceID::MAX as usize) + 1);
    }
}
