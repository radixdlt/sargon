use crate::prelude::*;

decl_ordered_map!(
    /// An ordered collection of unique [`AccountForDisplay`]s.
    AccountsForDisplay,
    AccountForDisplay
);

impl HasSampleValues for AccountsForDisplay {
    fn sample() -> Self {
        Self::from_iter([
            AccountForDisplay::sample(),
            AccountForDisplay::sample_other(),
        ])
    }

    fn sample_other() -> Self {
        Self::from_iter([AccountForDisplay::sample_other()])
    }
}
