use crate::prelude::*;

/// A discriminator for `***RoleWithFactor***` types. Especially useful for
/// `GeneralRoleWithHierarchicalDeterministicFactorInstances` which holds
/// the role it is used for.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    enum_iterator::Sequence,
)]
#[serde(rename_all = "camelCase")]
pub enum RoleKind {
    /// The primary role of some matrix of factors
    Primary,
    /// The recovery role of some matrix of factors
    Recovery,
    /// The confirmation role of some matrix of factors
    Confirmation,
}
impl RoleKind {
    pub fn all() -> Vec<Self> {
        enum_iterator::all::<Self>().collect()
    }
}
impl HasSampleValues for RoleKind {
    fn sample() -> Self {
        RoleKind::Primary
    }

    fn sample_other() -> Self {
        RoleKind::Recovery
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RoleKind;

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
    fn json_roundtrip_primary() {
        let model = SUT::Primary;
        assert_json_value_eq_after_roundtrip(&model, json!("primary"));
        assert_json_value_ne_after_roundtrip(&model, json!("recovery"));
        assert_json_roundtrip(&model);
    }

    #[test]
    fn json_roundtrip_recovery() {
        let model = SUT::Recovery;
        assert_json_value_eq_after_roundtrip(&model, json!("recovery"));
        assert_json_value_ne_after_roundtrip(&model, json!("primary"));
        assert_json_roundtrip(&model);
    }

    #[test]
    fn json_roundtrip_confirmation() {
        let model = SUT::Confirmation;
        assert_json_value_eq_after_roundtrip(&model, json!("confirmation"));
        assert_json_value_ne_after_roundtrip(&model, json!("primary"));
        assert_json_roundtrip(&model);
    }
}
