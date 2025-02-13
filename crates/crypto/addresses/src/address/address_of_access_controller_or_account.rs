use crate::{address_union, prelude::*};

address_union!(
    enum AddressOfAccessControllerOrAccount: access_controller, account
);

impl From<AddressOfAccessControllerOrAccount> for Address {
    fn from(value: AddressOfAccessControllerOrAccount) -> Self {
        match value {
            AddressOfAccessControllerOrAccount::AccessController(ac) => {
                ac.into()
            }
            AddressOfAccessControllerOrAccount::Account(account) => {
                account.into()
            }
        }
    }
}

impl TryFrom<Address> for AddressOfAccessControllerOrAccount {
    type Error = CommonError;
    fn try_from(value: Address) -> Result<Self> {
        match value {
            Address::AccessController(ac) => {
                Ok(AddressOfAccessControllerOrAccount::AccessController(ac))
            }
            Address::Account(account) => {
                Ok(AddressOfAccessControllerOrAccount::Account(account))
            }
            _ => Err(CommonError::FailedToMapAddressToSpecificType {
                expected_specific_type: Self::type_name(),
                got_value: value.to_string(),
            }),
        }
    }
}
