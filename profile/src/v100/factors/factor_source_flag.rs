use serde::{Deserialize, Serialize};

/// Flags which describe a certain state a FactorSource might be in, primarily used
/// by DeviceFactorSource's to mark which "Babylon" FactorSource is the **main** one.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, uniffi::Enum)]
#[serde(rename_all = "camelCase")]
pub enum FactorSourceFlag {
    /// Used to mark a "babylon" `.device` FactorSource as "main". All new accounts
    /// and Personas are created using the `main` `DeviceFactorSource`.
    ///
    /// We can only ever have one.
    /// We might have zero `main` flags across all  `DeviceFactorSource`s if and only if we have only one  `DeviceFactorSource`s. If we have two or more  `DeviceFactorSource`s one of them MUST
    /// be marked with `main`.
    Main,

    /// Until we have implemented "proper" deletion, we will "flag" a
    /// FactorSource as deleted by the user and hide it, meaning e.g.
    /// that in Multi-Factor Setup flows it will not show up.
    DeletedByUser,
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };
    use serde_json::json;

    use super::FactorSourceFlag;

    #[test]
    fn json_roundtrip_main() {
        let model = FactorSourceFlag::Main;
        assert_json_value_eq_after_roundtrip(&model, json!("main"));
        assert_json_value_ne_after_roundtrip(&model, json!("deletedByUser"));
        assert_json_roundtrip(&model);
    }
    #[test]
    fn json_roundtrip_deleted_by_user() {
        let model = FactorSourceFlag::DeletedByUser;
        assert_json_value_eq_after_roundtrip(&model, json!("deletedByUser"));
        assert_json_value_ne_after_roundtrip(&model, json!("main"));
        assert_json_roundtrip(&model);
    }
}
