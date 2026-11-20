use crate::prelude::*;
use sargon::RestrictedWithdrawal as InternalRestrictedWithdrawal;
use sargon::RestrictedWithdrawalStatus as InternalRestrictedWithdrawalStatus;
use sargon::TransferRequiredBadges as InternalTransferRequiredBadges;

/// Whether the account can satisfy the withdraw rule of a restricted resource.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
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
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct RestrictedWithdrawal {
    /// The transferred resource.
    pub resource: ResourceAddress,

    /// The badges its withdraw rule asks for, i.e. the resources an account can hold (not the
    /// implicit ones, like signatures). Empty when no badge can satisfy the rule.
    pub badges: Vec<ResourceAddress>,

    /// Whether the account can satisfy the rule.
    pub status: RestrictedWithdrawalStatus,
}

/// What the withdrawals of a transfer need: the badges to prove before them (give them to
/// `manifest_per_asset_transfers` / `manifest_per_recipient_transfers`), and the restricted
/// resources with what their rules ask for.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct TransferRequiredBadges {
    /// The badges to prove before withdrawing the restricted resources.
    pub required_badges: Vec<ResourceSpecifier>,

    /// The transferred resources that not everybody may withdraw.
    pub restricted_withdrawals: Vec<RestrictedWithdrawal>,
}

decl_conversion_tests_for!(RestrictedWithdrawalStatus);
decl_conversion_tests_for!(RestrictedWithdrawal);
decl_conversion_tests_for!(TransferRequiredBadges);
