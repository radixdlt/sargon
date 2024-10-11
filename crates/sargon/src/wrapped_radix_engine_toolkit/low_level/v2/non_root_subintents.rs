use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default, uniffi::Record)]
pub struct NonRootSubintents {
    pub subintents: Vec<Subintent>,
}

impl NonRootSubintents {
    pub fn new<I>(subintents: I) -> Self
    where
        I: IntoIterator<Item = Subintent>,
    {
        Self {
            subintents: subintents.into_iter().collect(),
        }
    }
}

impl From<NonRootSubintents> for ScryptoNonRootSubintents {
    fn from(value: NonRootSubintents) -> Self {
        Self {
            0: value.subintents.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<ScryptoNonRootSubintents> for NonRootSubintents {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoNonRootSubintents) -> Result<Self> {
        value
            .0
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<Subintent>>>()
            .map(Self::new)
    }
}

impl HasSampleValues for NonRootSubintents {
    fn sample() -> Self {
        Self::new(vec![Subintent::sample()])
    }

    fn sample_other() -> Self {
        Self::new(vec![Subintent::sample(), Subintent::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonRootSubintents;

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
    fn to_from_scrypto() {
        let roundtrip =
            |s: SUT| SUT::try_from(ScryptoNonRootSubintents::from(s)).unwrap();
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
