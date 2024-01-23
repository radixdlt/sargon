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

    // #[test]
    // fn new_persona_data() {
    //     let persona_data = PersonaData::new(IdentifiedEntry::default());
    //     assert_eq!(persona_data.name, IdentifiedEntry::default());
    // }
}
