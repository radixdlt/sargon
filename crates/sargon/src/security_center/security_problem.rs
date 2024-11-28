use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, derive_more::Display)]

/// An enum describing each potential Security Problem the Wallet can encounter.
///
/// See [the Confluence doc for details][doc].
///
/// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3392569357/Security-related+Problem+States+in+the+Wallet
pub enum SecurityProblem {
    /// The given addresses of `accounts` and `personas` are unrecoverable if the user loses their phone, since their corresponding seed phrase has not been written down.
    /// NOTE: This definition differs from the one at Confluence since we don't have shields implemented yet.
    #[display("Problem3")]
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
    #[display("Problem9")]
    Problem9 {
        addresses: AddressesOfEntitiesInBadState,
    },
}

impl Identifiable for SecurityProblem {
    type ID = u64;

    fn id(&self) -> Self::ID {
        match self {
            SecurityProblem::Problem3 { .. } => 3,
            SecurityProblem::Problem5 => 5,
            SecurityProblem::Problem6 => 6,
            SecurityProblem::Problem7 => 7,
            SecurityProblem::Problem9 { .. } => 9,
        }
    }
}

impl SecurityProblem {
    pub fn kind(&self) -> SecurityProblemKind {
        match self {
            SecurityProblem::Problem3 { .. }
            | SecurityProblem::Problem9 { .. } => {
                SecurityProblemKind::SecurityFactors
            }
            SecurityProblem::Problem5
            | SecurityProblem::Problem6
            | SecurityProblem::Problem7 => {
                SecurityProblemKind::ConfigurationBackup
            }
        }
    }
}

impl HasSampleValues for SecurityProblem {
    fn sample() -> Self {
        Self::Problem3 {
            addresses: AddressesOfEntitiesInBadState::sample(),
        }
    }

    fn sample_other() -> Self {
        Self::Problem5
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityProblem;

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
    fn id() {
        assert_eq!(
            SUT::Problem3 {
                addresses: AddressesOfEntitiesInBadState::sample()
            }
            .id(),
            3
        );
        assert_eq!(SUT::Problem5.id(), 5);
        assert_eq!(SUT::Problem6.id(), 6);
        assert_eq!(SUT::Problem7.id(), 7);
        assert_eq!(
            SUT::Problem9 {
                addresses: AddressesOfEntitiesInBadState::sample()
            }
            .id(),
            9
        );
    }

    #[test]
    fn kind() {
        assert_eq!(
            SUT::Problem3 {
                addresses: AddressesOfEntitiesInBadState::sample()
            }
            .kind(),
            SecurityProblemKind::SecurityFactors
        );
        assert_eq!(
            SUT::Problem5.kind(),
            SecurityProblemKind::ConfigurationBackup
        );
        assert_eq!(
            SUT::Problem6.kind(),
            SecurityProblemKind::ConfigurationBackup
        );
        assert_eq!(
            SUT::Problem7.kind(),
            SecurityProblemKind::ConfigurationBackup
        );
        assert_eq!(
            SUT::Problem9 {
                addresses: AddressesOfEntitiesInBadState::sample()
            }
            .kind(),
            SecurityProblemKind::SecurityFactors
        );
    }
}
