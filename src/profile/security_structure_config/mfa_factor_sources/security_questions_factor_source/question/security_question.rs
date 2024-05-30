use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A security question
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct Security_NOT_PRODUCTION_READY_Question {
    pub id: Uuid,    // FIXME: newtype
    pub version: u8, // FIXME: newtype
    pub kind: SecurityQuestionKind,
    pub question: String,
}

impl AsRef<str> for Security_NOT_PRODUCTION_READY_Question {
    fn as_ref(&self) -> &str {
        &self.question
    }
}

impl Identifiable for Security_NOT_PRODUCTION_READY_Question {
    type ID = Uuid; // FIXME: newtype

    /// Return `Element`'s globally unique and stable ID, used to uniquely identify
    /// the `Element` in the `IdentifiedVecOf` collection of elements.
    fn id(&self) -> Self::ID {
        self.id.clone()
    }
}

impl Security_NOT_PRODUCTION_READY_Question {
    pub fn with_details(
        id: Uuid,
        version: u8,
        kind: SecurityQuestionKind,
        question: impl AsRef<str>,
    ) -> Self {
        Self {
            id,
            version,
            kind,
            question: question.as_ref().to_owned(),
        }
    }
    pub fn new(kind: SecurityQuestionKind, question: impl AsRef<str>) -> Self {
        Self::with_details(id(), 1, kind, question)
    }
    pub fn freeform(question: impl AsRef<str>) -> Self {
        Self::new(SecurityQuestionKind::Freeform, question)
    }
}
impl HasSampleValues for Security_NOT_PRODUCTION_READY_Question {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        let mut value =
            Self::freeform("What was the make and model of your first car?");
        value.id = Uuid::from_bytes([0x5a; 16]);
        value
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        let mut value =
            Self::freeform("What was the first concert you attended?");
        value.id = Uuid::from_bytes([0x50; 16]);
        value
    }
}
