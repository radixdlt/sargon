use crate::prelude::*;

/// An immutable "snapshot" of `PetitionForFactorsState`
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub(super) struct PetitionForFactorsStateSnapshot {
    /// Factors that have signed.
    signed: IndexSet<HDSignature>,

    /// Factors that has been neglected.
    neglected: IndexSet<NeglectedFactorInstance>,
}

impl PetitionForFactorsStateSnapshot {
    pub(super) fn new(
        signed: IndexSet<HDSignature>,
        neglected: IndexSet<NeglectedFactorInstance>,
    ) -> Self {
        Self { signed, neglected }
    }

    pub(super) fn prompted_count(&self) -> i8 {
        self.signed_count() + self.neglected_count()
    }

    pub(super) fn signed_count(&self) -> i8 {
        self.signed.len() as i8
    }

    fn neglected_count(&self) -> i8 {
        self.neglected.len() as i8
    }

    #[allow(unused)]
    fn debug_str(&self) -> String {
        let signatures = self
            .signed
            .clone()
            .into_iter()
            .map(|s| format!("{:?}", s))
            .join(", ");

        let neglected = self
            .neglected
            .clone()
            .into_iter()
            .map(|s| format!("{:?}", s))
            .join(", ");

        format!("signatures: {:#?}, neglected: {:#?}", signatures, neglected)
    }
}

impl HasSampleValues for PetitionForFactorsStateSnapshot {
    fn sample() -> Self {
        Self::new(
            IndexSet::from_iter([HDSignature::sample(), HDSignature::sample_other()]),
            IndexSet::from_iter([
                NeglectedFactorInstance::sample(),
                NeglectedFactorInstance::sample_other(),
            ]),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            IndexSet::just(HDSignature::sample_other()),
            IndexSet::just(NeglectedFactorInstance::sample_other()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = PetitionForFactorsStateSnapshot;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other())
    }

    #[test]
    fn debug() {
        assert!(!format!("{:?}", Sut::sample()).is_empty());
    }
}
