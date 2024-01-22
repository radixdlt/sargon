use super::IdentifiedEntry;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    Display,
    uniffi::Record,
)]
#[display("{}", name)]
#[derive(Default)]
pub struct PersonaData {
    pub name: IdentifiedEntry,
}

impl PersonaData {
    pub fn new(name: IdentifiedEntry) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_persona_data() {
        let persona_data = PersonaData::new(IdentifiedEntry::default());
        assert_eq!(persona_data.name, IdentifiedEntry::default());
    }
}
