use crate::prelude::*;

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStructureMetadata {
    pub id: SecurityStructureID,
    pub display_name: DisplayName,
    pub created_on: Timestamp,
    pub last_updated_on: Timestamp,
}

impl Identifiable for SecurityStructureMetadata {
    type ID = SecurityStructureID;

    fn id(&self) -> Self::ID {
        self.id
    }
}

impl SecurityStructureMetadata {
    pub fn with_details(
        id: SecurityStructureID,
        display_name: DisplayName,
        created_on: Timestamp,
        last_updated_on: Timestamp,
    ) -> Self {
        Self {
            id,
            display_name,
            created_on,
            last_updated_on,
        }
    }

    pub fn new(display_name: DisplayName) -> Self {
        Self::with_details(
            SecurityStructureID::from(Uuid::new_v4()),
            display_name,
            now(),
            now(),
        )
    }
}
impl HasSampleValues for SecurityStructureMetadata {
    fn sample() -> Self {
        Self::with_details(
            SecurityStructureID::sample(),
            DisplayName::sample(),
            Timestamp::sample(),
            Timestamp::sample(),
        )
    }
    fn sample_other() -> Self {
        Self::with_details(
            SecurityStructureID::sample_other(),
            DisplayName::sample_other(),
            Timestamp::sample_other(),
            Timestamp::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureMetadata;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
