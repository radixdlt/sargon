use crate::prelude::*;
use sargon::SecurityProblem as InternalSecurityProblem;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
/// An enum describing each potential Security Problem the Wallet can encounter.
///
/// See [the Confluence doc for details][doc].
///
/// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3392569357/Security-related+Problem+States+in+the+Wallet
pub enum SecurityProblem {
    /// The given addresses of `accounts` and `personas` are unrecoverable if the user loses their phone, since their corresponding seed phrase has not been written down.
    /// NOTE: This definition differs from the one at Confluence since we don't have shields implemented yet.
    Problem3 {
        addresses: AddressesOfEntitiesInBadState,
    },

    /// Wallet backups to the cloud aren’t working (wallet tried to do a backup, and it didn’t work within, say, 5 minutes.)
    /// This means that currently all accounts and personas are at risk of being practically unrecoverable if the user loses their phone.
    /// Also they would lose all of their other non-security wallet settings and data.
    Problem5,

    /// Cloud backups are turned off  and user has never done a manual file export. This means that currently all accounts and personas  are at risk of
    /// being practically unrecoverable if the user loses their phone. Also, they would lose all of their other non-security wallet settings and data.
    Problem6,

    /// Cloud backups are turned off and user previously did a manual file export, but has made a change and haven’t yet re-exported a file backup that
    /// includes that change. This means that any changes made will be lost if the user loses their phone - including control of new accounts/personas they’ve
    /// created, as well as changed settings or changed/added data.
    Problem7,

    /// User has gotten a new phone (and restored their wallet from backup) and the wallet sees that there are accounts without shields using a phone key,
    /// meaning they can only be recovered with the seed phrase. (See problem 2) This would also be the state if a user disabled their PIN (and reenabled it), clearing phone keys.
    Problem9 {
        addresses: AddressesOfEntitiesInBadState,
    },
}

#[uniffi::export]
pub fn security_problem_kind(value: &SecurityProblem) -> SecurityProblemKind {
    value.into_internal().kind().into()
}