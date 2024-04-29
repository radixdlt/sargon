use crate::prelude::*;

decl_can_be_empty_identified_array_of!(AccountsForDisplay, AccountForDisplay);

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
