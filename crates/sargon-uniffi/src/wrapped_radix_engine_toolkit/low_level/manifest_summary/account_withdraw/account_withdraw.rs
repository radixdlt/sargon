use crate::prelude::*;
use sargon::AccountWithdraw as InternalAccountWithdraw;

/// Represents a withdrawal from an account, either by amount or by specific IDs.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum AccountWithdraw {
    /// Withdraw a specific amount from the account.
    Amount {
        resource_address: ResourceAddress,
        amount: Decimal,
    },

    /// Withdraw specific IDs from the account.
    Ids {
        resource_address: ResourceAddress,
        ids: Vec<NonFungibleLocalId>,
    },
}