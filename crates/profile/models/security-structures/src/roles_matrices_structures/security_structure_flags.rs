use crate::prelude::*;

/// Flags which describe a certain state a Security Shield might be in, primarily used
/// to mark which Security Shield is the **default** one.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
#[serde(rename_all = "camelCase")]
pub enum SecurityStructureFlag {
    /// Used to mark a Security Shield as "default". We can only have one.
    Default,
}

pub type SecurityStructureFlags = IdentifiedVecOf<SecurityStructureFlag>;

impl Identifiable for SecurityStructureFlag {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_roundtrip_main() {
        let model = SecurityStructureFlag::Default;
        assert_json_value_eq_after_roundtrip(&model, json!("default"));
        assert_json_roundtrip(&model);
    }
}
