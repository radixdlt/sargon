use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
#[allow(clippy::large_enum_variant)] // we cannot Box<Account>, since Box is not UniFFI compatible.
pub enum AssetsTransfersRecipient {
    MyOwnAccount { value: Account },
    ForeignAccount { value: AccountAddress },
}

impl From<Account> for AssetsTransfersRecipient {
    fn from(value: Account) -> Self {
        Self::MyOwnAccount { value }
    }
}

impl From<AccountAddress> for AssetsTransfersRecipient {
    fn from(value: AccountAddress) -> Self {
        Self::ForeignAccount { value }
    }
}

impl AssetsTransfersRecipient {
    pub fn account_address(&self) -> &AccountAddress {
        match self {
            AssetsTransfersRecipient::MyOwnAccount { value } => &value.address,
            AssetsTransfersRecipient::ForeignAccount { value } => value,
        }
    }
}
