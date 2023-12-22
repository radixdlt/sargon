use nutype::nutype;

use crate::CommonError as Error;

#[nutype(
    validate(less_or_equal = 11),
    derive(
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )
)]
pub struct AppearanceID(u8);

impl Default for AppearanceID {
    fn default() -> Self {
        Self::new(0).expect("AppearanceID of 0 to be valid")
    }
}

impl TryFrom<u8> for AppearanceID {
    type Error = crate::CommonError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        AppearanceID::new(value).map_err(|_| Error::InvalidAppearanceID)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_json_value_eq_after_roundtrip, assert_json_value_fails};
    use serde_json::json;

    use crate::v100::entity::account::appearance_id::{AppearanceID, AppearanceIDError};
    use crate::CommonError as Error;

    #[test]
    fn lowest() {
        assert!(AppearanceID::new(0).is_ok());
    }

    #[test]
    fn highest() {
        assert!(AppearanceID::new(11).is_ok());
    }

    #[test]
    fn err_too_big() {
        assert_eq!(
            AppearanceID::new(12),
            Err(AppearanceIDError::LessOrEqualViolated)
        );
    }

    #[test]
    fn try_from() {
        assert_eq!(AppearanceID::try_from(250), Err(Error::InvalidAppearanceID));
        assert_eq!(AppearanceID::try_from(1), Ok(AppearanceID::new(1).unwrap()));
    }

    #[test]
    fn json() {
        assert_json_value_eq_after_roundtrip(&AppearanceID::new(3).unwrap(), json!(3));
        assert_json_value_fails::<AppearanceID>(json!("3"));
        assert_json_value_fails::<AppearanceID>(json!(99));
    }
}
