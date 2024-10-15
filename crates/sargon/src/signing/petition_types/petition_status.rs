use crate::prelude::*;

/// An aggregation of the status of all petitions for transaction,
/// if all transactions are valid, if some are invalid, if none are invalid
/// (but all are not yet valid).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum PetitionsStatus {
    /// All transactions are valid.
    AllAreValid,

    /// Some transaction is invalid (one or more), and some might be valid.
    SomeIsInvalid,

    /// Not all transactions are valid, but none are invalid.
    InProgressNoneInvalid,
}

impl PetitionsStatus {
    /// returns true if all petitions are valid.
    pub(crate) fn are_all_valid(&self) -> bool {
        matches!(self, Self::AllAreValid)
    }

    /// returns true if some petitions are invalid.
    pub(crate) fn is_some_invalid(&self) -> bool {
        matches!(self, Self::SomeIsInvalid)
    }

    pub(crate) fn reducing(
        statuses: impl IntoIterator<Item = PetitionForFactorsStatus>,
    ) -> Self {
        PetitionForFactorsStatus::aggregate(
            statuses.into_iter().collect_vec(),
            Self::AllAreValid,
            Self::SomeIsInvalid,
            Self::InProgressNoneInvalid,
        )
    }
}
