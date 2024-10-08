use crate::prelude::*;
use sargon::AccountOrAddressOf as InternalAccountOrAddressOf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
#[allow(clippy::large_enum_variant)] // we cannot Box<Account>, since Box is not UniFFI compatible.
pub enum AccountOrAddressOf {
    ProfileAccount { value: Account },
    AddressOfExternalAccount { value: AccountAddress },
}

impl From<InternalAccountOrAddressOf> for AccountOrAddressOf {
    fn from(value: InternalAccountOrAddressOf) -> Self {
        match value {
            InternalAccountOrAddressOf::ProfileAccount { value } => AccountOrAddressOf::ProfileAccount {
                value: value.into(),
            },
            InternalAccountOrAddressOf::AddressOfExternalAccount { value } => AccountOrAddressOf::AddressOfExternalAccount {
                value: value.into(),
            },
        }
    }
}

impl Into<InternalAccountOrAddressOf> for AccountOrAddressOf {
    fn into(self) -> InternalAccountOrAddressOf {
        match self {
            AccountOrAddressOf::ProfileAccount { value } => InternalAccountOrAddressOf::ProfileAccount {
                value: value.into(),
            },
            AccountOrAddressOf::AddressOfExternalAccount { value } => InternalAccountOrAddressOf::AddressOfExternalAccount {
                value: value.into(),
            },
        }
    }
}

#[uniffi::export]
pub fn new_account_or_address_of_sample() -> AccountOrAddressOf {
    InternalAccountOrAddressOf::sample().into()
}

#[uniffi::export]
pub fn new_account_or_address_of_sample_other() -> AccountOrAddressOf {
    InternalAccountOrAddressOf::sample_other().into()
}

#[uniffi::export]
pub fn account_or_address_of_account_address(
    recipient: &AccountOrAddressOf,
) -> AccountAddress {
    *recipient.into_internal().account_address()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountOrAddressOf;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_or_address_of_sample(),
                new_account_or_address_of_sample_other(),
                // duplicates should get removed
                new_account_or_address_of_sample(),
                new_account_or_address_of_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_account_address() {
        let sut = SUT::sample();
        assert_eq!(
            account_or_address_of_account_address(&sut),
            *sut.account_address()
        )
    }
}
