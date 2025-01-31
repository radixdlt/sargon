use crate::{address_union, prelude::*};

address_union!(
    enum AddressOfVaultOrAccount: vault, account
);

impl From<AddressOfVaultOrAccount> for Address {
    fn from(value: AddressOfVaultOrAccount) -> Self {
        match value {
            AddressOfVaultOrAccount::Vault(vault) => vault.into(),
            AddressOfVaultOrAccount::Account(account) => account.into(),
        }
    }
}
impl TryFrom<Address> for AddressOfVaultOrAccount {
    type Error = CommonError;
    fn try_from(value: Address) -> Result<Self> {
        match value {
            Address::Vault(vault) => Ok(AddressOfVaultOrAccount::Vault(vault)),
            Address::Account(account) => {
                Ok(AddressOfVaultOrAccount::Account(account))
            }
            _ => Err(CommonError::FailedToMapAddressToSpecficType {
                expected_specific_type: Self::type_name(),
                got_value: value.to_string(),
            }),
        }
    }
}
