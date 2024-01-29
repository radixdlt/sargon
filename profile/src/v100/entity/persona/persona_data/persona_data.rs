use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Default,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.existing_values())]
#[debug("{}", self.existing_values())]
#[serde(rename_all = "camelCase")]
pub struct PersonaData {
    pub name: Option<PersonaDataIdentifiedName>,
}

impl PersonaData {
    pub fn new(name: Option<PersonaDataIdentifiedName>) -> Self {
        Self { name }
    }
}

impl PersonaData {
    pub fn existing_values(&self) -> String {
        let name = self
            .name
            .as_deref()
            .map(|v| format!("name: {}\n", v.clone()));

        [name].into_iter().map(|v| v.unwrap_or_default()).join("")
    }
}

impl HasPlaceholder for PersonaData {
    fn placeholder() -> Self {
        Self::new(Some(PersonaDataIdentifiedName::placeholder()))
    }

    fn placeholder_other() -> Self {
        Self::new(Some(PersonaDataIdentifiedName::placeholder_other()))
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
        let persona_data = PersonaData::new(Some(
            PersonaDataIdentifiedName::with_id(Uuid::nil(), name.clone()),
        ));
        assert_eq!(
            persona_data.name,
            Some(PersonaDataIdentifiedName::with_id(
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
                name
            ))
        );
    }

    #[test]
    fn placeholder() {
        let persona_data = PersonaData::placeholder();
        let identified_entry = PersonaDataIdentifiedName::placeholder();
        assert_eq!(persona_data.name, Some(identified_entry));
    }

    #[test]
    fn placeholder_other() {
        assert_eq!(
            PersonaData::placeholder_other().name,
            Some(PersonaDataIdentifiedName::placeholder_other())
        );
    }
}
