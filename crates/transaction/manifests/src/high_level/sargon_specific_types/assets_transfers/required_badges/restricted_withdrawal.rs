use crate::prelude::*;

/// Whether the account can satisfy the withdraw rule of a restricted resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RestrictedWithdrawalStatus {
    /// The account holds the badges the rule asks for (or signing is enough): their proofs are
    /// in the `required_badges` of the transfer.
    Satisfied,

    /// The account does not hold (enough of) the badges the rule asks for.
    MissingBadges,

    /// No badge the account could hold satisfies the rule: it denies everybody, or it asks for
    /// something a transfer cannot give (the signature of another key, a call from a component
    /// or package).
    NotWithdrawable,
}

/// A transferred resource that not everybody may withdraw.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RestrictedWithdrawal {
    /// The transferred resource.
    pub resource: ResourceAddress,

    /// The badges its withdraw rule asks for, i.e. the resources an account can hold (not the
    /// implicit ones, like signatures). Empty when no badge can satisfy the rule.
    pub badges: Vec<ResourceAddress>,

    /// Whether the account can satisfy the rule.
    pub status: RestrictedWithdrawalStatus,
}

impl RestrictedWithdrawal {
    pub fn new(
        resource: ResourceAddress,
        badges: impl IntoIterator<Item = ResourceAddress>,
        status: RestrictedWithdrawalStatus,
    ) -> Self {
        Self {
            resource,
            badges: badges.into_iter().collect(),
            status,
        }
    }
}

impl HasSampleValues for RestrictedWithdrawalStatus {
    fn sample() -> Self {
        Self::Satisfied
    }

    fn sample_other() -> Self {
        Self::NotWithdrawable
    }
}

impl HasSampleValues for RestrictedWithdrawal {
    fn sample() -> Self {
        Self::new(
            ResourceAddress::sample_mainnet_xrd(),
            [ResourceAddress::sample_mainnet_candy()],
            RestrictedWithdrawalStatus::Satisfied,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            ResourceAddress::sample_mainnet_nft_gc_membership(),
            [],
            RestrictedWithdrawalStatus::NotWithdrawable,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RestrictedWithdrawal;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
        assert_eq!(
            RestrictedWithdrawalStatus::sample(),
            RestrictedWithdrawalStatus::sample()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(
            RestrictedWithdrawalStatus::sample(),
            RestrictedWithdrawalStatus::sample_other()
        );
    }
}
