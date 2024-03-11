use crate::prelude::*;

/// If `true` the `try_deposit_batch_or_abort` method will be used instead of `deposit`,
/// typically wallets sets this to try if and only if the recipient is a self-owned account
/// (`AssetsTransfersRecipient::MyOwnAccount`) controlled by a DeviceFactorSource thy have
/// access to and which third party deposit setting's `DepositRule` is `AcceptKnown` and
/// which resource is known (`resource_address` is owned or has been owned before).
// pub use_try_deposit_or_abort: bool

pub struct AssetsTransfersPrototype {
    pub from_account: AccountAddress,
    pub of_fungible_resources: Vec<TransfersOfFungibleResource>,
}

pub struct TransfersOfFungibleResource {
    pub amounts: Vec<FungibleTransferAmount>,
    pub fungible: FungibleResource,
}

impl TransfersOfFungibleResource {
    /// sum of all `amount` in
    pub fn total_transfer_amount(&self) -> Decimal192 {
        self.amounts
            .into_iter()
            .map(|x| x.amount)
            .fold(Decimal::zero(), |acc, x| acc + x)
    }
}

pub struct FungibleTransferAmount {
    pub recipient: AssetsTransfersRecipient,
    pub amount: Decimal192,
}

pub struct FungibleResource {
    pub address: ResourceAddress,
    pub divisibility: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
#[allow(clippy::large_enum_variant)] // we cannot Box<Account>, since Box is not UniFFI compatible.
pub enum AssetsTransfersRecipient {
    MyOwnAccount { value: Account },
    ForeignAccount { value: AccountAddress },
}
