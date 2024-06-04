use crate::prelude::*;

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStructureMetadata {
    pub id: Uuid,
    pub display_name: DisplayName,
    pub created_on: Timestamp,
    pub last_updated_on: Timestamp,
}

impl Identifiable for SecurityStructureMetadata {
    type ID = Uuid;

    fn id(&self) -> Self::ID {
        self.id.clone()
    }
}

impl SecurityStructureMetadata {
    pub fn with_details(
        id: Uuid,
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
        Self::with_details(id(), display_name, now(), now())
    }
}
impl HasSampleValues for SecurityStructureMetadata {
    fn sample() -> Self {
        Self::with_details(
            Uuid::sample(),
            DisplayName::sample(),
            Timestamp::sample(),
            Timestamp::sample(),
        )
    }
    fn sample_other() -> Self {
        Self::with_details(
            Uuid::sample_other(),
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
