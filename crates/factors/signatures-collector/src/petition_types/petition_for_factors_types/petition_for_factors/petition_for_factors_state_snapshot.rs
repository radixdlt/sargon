use crate::prelude::*;

/// An immutable "snapshot" of `PetitionForFactorsState`
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub(super) struct PetitionForFactorsStateSnapshot<ID: SignableID> {
    /// Factors that have signed.
    signed: IndexSet<HDSignature<ID>>,

    /// Factors that has been neglected.
    neglected: IndexSet<NeglectedFactorInstance>,
}

impl<ID: SignableID> PetitionForFactorsStateSnapshot<ID> {
    pub(super) fn new(
        signed: IndexSet<HDSignature<ID>>,
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

impl<ID: SignableID + HasSampleValues> HasSampleValues
    for PetitionForFactorsStateSnapshot<ID>
{
    fn sample() -> Self {
        Self::new(
            IndexSet::from_iter([
                HDSignature::<ID>::sample(),
                HDSignature::<ID>::sample_other(),
            ]),
            IndexSet::from_iter([
                NeglectedFactorInstance::sample(),
                NeglectedFactorInstance::sample_other(),
            ]),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            IndexSet::just(HDSignature::<ID>::sample_other()),
            IndexSet::just(NeglectedFactorInstance::sample_other()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PetitionForFactorsStateSnapshot<TransactionIntentHash>;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other())
    }

    #[test]
    fn debug() {
        assert!(!format!("{:?}", SUT::sample()).is_empty());
    }
}
