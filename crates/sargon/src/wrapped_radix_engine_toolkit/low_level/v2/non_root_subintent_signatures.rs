use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default, uniffi::Record)]
pub struct NonRootSubintentSignatures {
    pub by_subintent: Vec<IntentSignaturesV2>,
}

impl NonRootSubintentSignatures {
    pub fn new<I>(by_subintent: I) -> Self
    where
        I: IntoIterator<Item = IntentSignaturesV2>,
    {
        Self {
            by_subintent: by_subintent.into_iter().collect(),
        }
    }
}

impl From<NonRootSubintentSignatures> for ScryptoNonRootSubintentSignatures {
    fn from(value: NonRootSubintentSignatures) -> Self {
        Self {
            by_subintent: value
                .by_subintent
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl TryFrom<(ScryptoNonRootSubintentSignatures, Hash)>
    for NonRootSubintentSignatures
{
    type Error = crate::CommonError;

    fn try_from(
        value: (ScryptoNonRootSubintentSignatures, Hash),
    ) -> Result<Self> {
        value
            .0
            .by_subintent
            .into_iter()
            .map(|s| {
                TryInto::<IntentSignaturesV2>::try_into((s, value.1.to_owned()))
            })
            .collect::<Result<Vec<IntentSignaturesV2>>>()
            .map(Self::new)
    }
}

impl HasSampleValues for NonRootSubintentSignatures {
    fn sample() -> Self {
        Self::new(vec![IntentSignaturesV2::sample()])
    }

    fn sample_other() -> Self {
        Self::new(vec![
            IntentSignaturesV2::sample(),
            IntentSignaturesV2::sample_other(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonRootSubintentSignatures;

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
        let roundtrip = |s: SUT| {
            SUT::try_from((
                ScryptoNonRootSubintentSignatures::from(s),
                Hash::sample(),
            ))
            .unwrap()
        };
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}