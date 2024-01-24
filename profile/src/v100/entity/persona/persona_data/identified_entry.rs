use crate::prelude::*;

type PersonaDataEntryID = Uuid;

// TODO: Needs to be made generic when adding more entry_kinds. Right now uniffi::Record complains if the function is generic.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{} - {}", value, id)]
pub struct IdentifiedEntry {
    pub id: PersonaDataEntryID,
    pub value: Name,
}

impl IdentifiedEntry {
    pub fn new(id: PersonaDataEntryID, value: Name) -> Self {
        Self { id, value }
    }
}

impl HasPlaceholder for IdentifiedEntry {
    fn placeholder() -> Self {
        IdentifiedEntry::new(
            Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap(),
            Name::placeholder(),
        )
    }

    fn placeholder_other() -> Self {
        IdentifiedEntry::new(
            Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
            Name::placeholder_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_after_json_roundtrip;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn new() {
        let name = Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
            .expect("Name counstruction should not fail");
        let identified_entry = IdentifiedEntry::new(Uuid::nil(), name.clone());
        assert_eq!(
            identified_entry.id,
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
        assert_eq!(identified_entry.value, name)
    }

    #[test]
    fn display() {
        let name = Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
            .expect("Name counstruction should not fail");
        let identified_entry = IdentifiedEntry::new(Uuid::nil(), name);
        assert_eq!(
            format!("{identified_entry}"),
            "Bruce Batman Wayne - 00000000-0000-0000-0000-000000000000"
        );
    }

    #[test]
    fn placeholder() {
        let placeholder = IdentifiedEntry::placeholder();
        assert_eq!(
            placeholder.id.to_string(),
            "00000000-0000-0000-0000-000000000001"
        );
        assert_eq!(placeholder.value.to_string(), "Bruce Batman Wayne");
    }

    #[test]
    fn placeholder_other() {
        let placeholder = IdentifiedEntry::placeholder_other();
        assert_eq!(
            placeholder.id.to_string(),
            "00000000-0000-0000-0000-000000000002"
        );
        assert_eq!(placeholder.value.to_string(), "Jun-fan Bruce Lee");
    }

    #[test]
    fn json_roundtrip_batman() {
        let model = IdentifiedEntry::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000001",
                "value": {
                    "variant": "Western",
                    "familyName": "Wayne",
                    "givenName": "Bruce",
                    "nickname": "Batman"
                }
             }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_bruce_lee() {
        let model = IdentifiedEntry::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000002",
                "value": {
                    "variant": "Eastern",
                    "familyName": "Jun-fan",
                    "givenName": "Lee",
                    "nickname": "Bruce"
                }
            }
            "#,
        )
    }
}
