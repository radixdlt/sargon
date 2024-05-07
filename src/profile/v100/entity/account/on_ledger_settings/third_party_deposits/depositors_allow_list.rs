use crate::prelude::*;

decl_ordered_map!(
    /// Allows certain third-party depositors to deposit assets freely.
    /// Note: There is no `deny` counterpart for this.
    DepositorsAllowList,
    ResourceOrNonFungible
);

impl HasSampleValues for DepositorsAllowList {
    fn sample() -> Self {
        Self::from_iter([
            ResourceOrNonFungible::sample(),
            ResourceOrNonFungible::sample_other(),
        ])
    }

    fn sample_other() -> Self {
        Self::from_iter([ResourceOrNonFungible::sample_other()])
    }
}
