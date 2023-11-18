use nutype::nutype;

#[nutype(validate(max = 11))]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppearanceID(u8);

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_test_utils::json::{
        assert_json_value_eq_after_roundtrip, assert_json_value_fails,
    };

    use crate::v100::entity::account::appearance_id::{AppearanceID, AppearanceIDError};

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
        assert_eq!(AppearanceID::new(12), Err(AppearanceIDError::TooBig));
    }

    #[test]
    fn json() {
        assert_json_value_eq_after_roundtrip(&AppearanceID::new(3).unwrap(), json!(3));
        assert_json_value_fails::<AppearanceID>(json!("3"));
        assert_json_value_fails::<AppearanceID>(json!(99));
    }
}
