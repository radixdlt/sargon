use crate::prelude::*;

/// An ID of some PersonaData Entry a user has shared.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    derive_more::Display,
    derive_more::Debug,
    Hash,
)]
#[serde(transparent)]
pub struct PersonaDataEntryID(pub Uuid); // Unfortunately we have to declare this since we want it to impl `Identifiable`, which is not our trait and `Uuid` is not our type... => force to have a newtype.

impl Identifiable for PersonaDataEntryID {
    type ID = Self;

    fn id(&self) -> Self::ID {
        *self
    }
}

impl std::ops::Deref for PersonaDataEntryID {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PersonaDataEntryID {
    /// Generates a new `PersonaDataEntryID` (using Uuid::new_v4())
    pub fn generate() -> Self {
        id().into()
    }
}

impl FromStr for PersonaDataEntryID {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s)
            .map_err(|_| CommonError::InvalidUUIDv4 {
                bad_value: s.to_owned(),
            })
            .map(|v| v.into())
    }
}

impl From<Uuid> for PersonaDataEntryID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl PersonaDataEntryID {
    fn from_u64(value: u64) -> Self {
        Uuid::from_u64_pair(0, value).into()
    }

    pub fn sample_one() -> Self {
        Self::from_u64(1)
    }

    pub fn sample_two() -> Self {
        Self::from_u64(2)
    }

    pub fn sample_three() -> Self {
        Self::from_u64(3)
    }

    pub fn sample_four() -> Self {
        Self::from_u64(4)
    }
}

impl HasSampleValues for PersonaDataEntryID {
    fn sample() -> Self {
        Self::sample_one()
    }

    fn sample_other() -> Self {
        Self::sample_two()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PersonaDataEntryID;

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
    fn deref() {
        assert_eq!(*SUT::sample_one(), Uuid::from_u64_pair(0, 1));
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<_>::from_iter([
                SUT::sample_one(),
                SUT::sample_two(),
                SUT::sample_three(),
                SUT::sample_four(),
                // twice
                SUT::sample_one(),
                SUT::sample_two(),
                SUT::sample_three(),
                SUT::sample_four(),
            ])
            .len(),
            4
        );
    }

    #[test]
    fn generate() {
        let n = 1000;
        assert_eq!(
            (0..n)
                .map(|_| SUT::generate())
                .collect::<HashSet<_>>()
                .len(),
            n
        );
    }

    #[test]
    fn from_str_ok() {
        let s = "00000000-0000-0000-0000-000000000000";
        assert_eq!(s.parse::<SUT>().unwrap().to_string(), s.to_string());
    }
    #[test]
    fn from_str_err() {
        assert_eq!(
            "foobar".parse::<SUT>(),
            Err(CommonError::InvalidUUIDv4 {
                bad_value: "foobar".to_owned()
            })
        );
    }
}
