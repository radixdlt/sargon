use crate::prelude::*;
use time_utils::now;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AddressBookEntry {
    /// What the entry names, written as the key it is under: `address` or
    /// `nonFungibleGlobalID`.
    #[serde(flatten)]
    pub target: AddressBookEntryTarget,
    pub name: DisplayName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl AddressBookEntry {
    pub const NOTE_MAX_LEN: usize = 140;

    pub fn new(
        target: impl Into<AddressBookEntryTarget>,
        name: DisplayName,
        note: Option<String>,
    ) -> Self {
        let timestamp = now();
        Self::with_timestamps(target, name, note, timestamp, timestamp)
    }

    pub fn with_timestamps(
        target: impl Into<AddressBookEntryTarget>,
        name: DisplayName,
        note: Option<String>,
        created_at: Timestamp,
        updated_at: Timestamp,
    ) -> Self {
        Self {
            target: target.into(),
            name,
            note: Self::normalize_note(note),
            created_at,
            updated_at,
        }
    }

    pub fn update_name_and_note(
        &mut self,
        name: DisplayName,
        note: Option<String>,
    ) {
        self.name = name;
        self.note = Self::normalize_note(note);
        self.updated_at = now();
    }

    pub fn normalize_note(note: Option<String>) -> Option<String> {
        note.map(|n| n.trim().to_owned())
            .and_then(|n| if n.is_empty() { None } else { Some(n) })
            .map(|n| n.chars().take(Self::NOTE_MAX_LEN).collect())
    }
}

impl Identifiable for AddressBookEntry {
    type ID = AddressBookEntryTarget;

    fn id(&self) -> Self::ID {
        self.target.clone()
    }
}

impl IsNetworkAware for AddressBookEntry {
    fn network_id(&self) -> NetworkID {
        self.target.network_id()
    }
}

impl HasSampleValues for AddressBookEntry {
    fn sample() -> Self {
        Self::with_timestamps(
            AccountAddress::sample_mainnet(),
            DisplayName::sample(),
            Some("Exchange address".to_owned()),
            Timestamp::sample(),
            Timestamp::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::with_timestamps(
            ResourceAddress::sample_stokenet_gum(),
            DisplayName::sample_other(),
            Some("Shared address".to_owned()),
            Timestamp::sample_other(),
            Timestamp::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AddressBookEntry;

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
    fn note_is_normalized() {
        assert_eq!(
            SUT::normalize_note(Some("   hello   ".to_owned())),
            Some("hello".to_owned())
        );
        assert_eq!(SUT::normalize_note(Some("   ".to_owned())), None);
        assert_eq!(SUT::normalize_note(None), None);
    }

    #[test]
    fn note_is_limited_to_max_len() {
        let note = "a".repeat(SUT::NOTE_MAX_LEN + 10);
        let normalized = SUT::normalize_note(Some(note)).unwrap();
        assert_eq!(normalized.len(), SUT::NOTE_MAX_LEN);
    }

    #[test]
    fn update_name_and_note_bumps_updated_at() {
        let mut sut = SUT::with_timestamps(
            AccountAddress::sample_mainnet(),
            DisplayName::sample(),
            Some("one".to_owned()),
            Timestamp::parse("2024-01-01T00:00:00Z").unwrap(),
            Timestamp::parse("2024-01-01T00:00:00Z").unwrap(),
        );
        let created_at = sut.created_at;

        sut.update_name_and_note(
            DisplayName::sample_other(),
            Some("two".to_owned()),
        );

        assert_eq!(sut.created_at, created_at);
        assert_ne!(
            sut.updated_at,
            Timestamp::parse("2024-01-01T00:00:00Z").unwrap()
        );
        assert_eq!(sut.note, Some("two".to_owned()));
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_json_roundtrip(&sut);
    }

    #[test]
    fn entry_for_an_address_is_written_as_before() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
            {
              "address": "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
              "name": "Spending Account",
              "note": "Exchange address",
              "createdAt": "2023-09-11T16:05:56.000Z",
              "updatedAt": "2023-09-11T16:05:56.000Z"
            }
            "#,
        );
    }

    #[test]
    fn entry_for_a_non_fungible_names_it() {
        let sut = SUT::with_timestamps(
            NonFungibleGlobalId::sample(),
            DisplayName::sample(),
            None,
            Timestamp::sample(),
            Timestamp::sample(),
        );
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "nonFungibleGlobalID": "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>",
              "name": "Spending Account",
              "createdAt": "2023-09-11T16:05:56.000Z",
              "updatedAt": "2023-09-11T16:05:56.000Z"
            }
            "#,
        );
    }
}
