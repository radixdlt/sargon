use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct AssetsTransfersTransactionPrototype {
    pub from_account: AccountAddress,
    pub transfers: Vec<AssetsTransfersToRecipient>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct AssetsTransfersToRecipient {
    pub recipient: AssetsTransfersRecipient,
    pub fungibles: Vec<FungiblePositiveAmount>,
    pub non_fungibles: Vec<NonFungibleGlobalId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]

pub struct FungiblePositiveAmount {
    pub resource_address: ResourceAddress,
    pub amount: Decimal192,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
#[allow(clippy::large_enum_variant)] // we cannot Box<Account>, since Box is not UniFFI compatible.
pub enum AssetsTransfersRecipient {
    MyOwnAccount { value: Account },
    ForeignAccount { value: AccountAddress },
}
