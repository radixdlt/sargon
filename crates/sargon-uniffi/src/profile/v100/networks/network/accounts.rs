use crate::prelude::*;
use sargon::Accounts as InternalAccounts;

decl_identified_vec_of!(
    /// An ordered set of [`Account`]s on a specific network, most commonly
    /// the set is non-empty, since wallets guide user to create a first
    /// Account.
    Account
);