use crate::prelude::*;

decl_identified_vec_of!(
    /// Allows certain third-party depositors to deposit assets freely.
    /// Note: There is no `deny` counterpart for this.
    DepositorsAllowList,
    ResourceOrNonFungible
);
