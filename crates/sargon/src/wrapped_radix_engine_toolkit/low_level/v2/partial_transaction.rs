use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct PartialTransaction {
    pub root_subintent: Subintent,
    pub non_root_subintents: NonRootSubintents,
}

impl PartialTransaction {
    pub fn new(
        root_subintent: Subintent,
        non_root_subintents: NonRootSubintents,
    ) -> Self {
        Self {
            root_subintent,
            non_root_subintents,
        }
    }

    pub fn with_root_subintent(root_subintent: Subintent) -> Self {
        Self {
            root_subintent,
            non_root_subintents: NonRootSubintents::default(),
        }
    }
}

impl From<PartialTransaction> for ScryptoPartialTransaction {
    fn from(value: PartialTransaction) -> Self {
        Self {
            root_subintent: value.root_subintent.into(),
            non_root_subintents: value.non_root_subintents.into(),
        }
    }
}

impl TryFrom<ScryptoPartialTransaction> for PartialTransaction {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoPartialTransaction) -> Result<Self> {
        Ok(Self {
            root_subintent: value.root_subintent.try_into()?,
            non_root_subintents: value.non_root_subintents.try_into()?,
        })
    }
}

impl HasSampleValues for PartialTransaction {
    fn sample() -> Self {
        Self {
            root_subintent: Subintent::sample(),
            non_root_subintents: NonRootSubintents::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            root_subintent: Subintent::sample_other(),
            non_root_subintents: NonRootSubintents::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PartialTransaction;

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
            |s: SUT| SUT::try_from(ScryptoPartialTransaction::from(s)).unwrap();
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
