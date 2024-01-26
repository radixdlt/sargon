use crate::prelude::*;

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
#[display("{}", name)]
#[serde(rename_all = "camelCase")]
pub struct PersonaData {
    pub name: IdentifiedEntry,
}

impl PersonaData {
    pub fn new(name: IdentifiedEntry) -> Self {
        Self { name }
    }
}

impl HasPlaceholder for PersonaData {
    fn placeholder() -> Self {
        Self::new(IdentifiedEntry::placeholder())
    }

    fn placeholder_other() -> Self {
        Self::new(IdentifiedEntry::placeholder_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(PersonaData::placeholder(), PersonaData::placeholder());
        assert_eq!(
            PersonaData::placeholder_other(),
            PersonaData::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            PersonaData::placeholder(),
            PersonaData::placeholder_other()
        );
    }

    #[test]
    fn new_persona_data() {
        let name =
            Name::new(Variant::Western, "Skywalker", "Anakin", "Darth Vader")
                .expect("Name counstruction should not fail");
        let persona_data =
            PersonaData::new(IdentifiedEntry::new(Uuid::nil(), name.clone()));
        assert_eq!(
            persona_data.name,
            IdentifiedEntry::new(
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
                name
            )
        );
    }

    #[test]
    fn placeholder() {
        let persona_data = PersonaData::placeholder();
        let identified_entry = IdentifiedEntry::placeholder();
        assert_eq!(persona_data.name, identified_entry);
    }

    #[test]
    fn placeholder_other() {
        let persona_data = PersonaData::placeholder_other();
        let identified_entry = IdentifiedEntry::placeholder_other();
        assert_eq!(persona_data.name, identified_entry);
    }
}
