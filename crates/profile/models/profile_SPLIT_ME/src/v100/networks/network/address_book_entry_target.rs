use crate::prelude::*;

/// What an [`AddressBookEntry`] names: an address, e.g. of an account, or a
/// non-fungible, e.g. the signature resource of someone's key.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(
    from = "AddressBookEntryTargetJson",
    into = "AddressBookEntryTargetJson"
)]
pub enum AddressBookEntryTarget {
    Address { value: Address },
    NonFungible { value: NonFungibleGlobalId },
}

/// How an entry's target is written in the Profile: as a single key holding
/// it, which entries flatten into themselves, so an entry for an address
/// reads as it always has.
#[derive(Serialize, Deserialize)]
enum AddressBookEntryTargetJson {
    #[serde(rename = "address")]
    Address(Address),

    #[serde(rename = "nonFungibleGlobalID")]
    NonFungible(NonFungibleGlobalId),
}

impl From<AddressBookEntryTargetJson> for AddressBookEntryTarget {
    fn from(value: AddressBookEntryTargetJson) -> Self {
        match value {
            AddressBookEntryTargetJson::Address(value) => {
                Self::Address { value }
            }
            AddressBookEntryTargetJson::NonFungible(value) => {
                Self::NonFungible { value }
            }
        }
    }
}

impl From<AddressBookEntryTarget> for AddressBookEntryTargetJson {
    fn from(value: AddressBookEntryTarget) -> Self {
        match value {
            AddressBookEntryTarget::Address { value } => Self::Address(value),
            AddressBookEntryTarget::NonFungible { value } => {
                Self::NonFungible(value)
            }
        }
    }
}

macro_rules! address_book_entry_target_from_address {
    ($($address: ty),+) => {
        $(
            impl From<$address> for AddressBookEntryTarget {
                fn from(value: $address) -> Self {
                    Self::Address {
                        value: value.into(),
                    }
                }
            }
        )+
    };
}

address_book_entry_target_from_address!(
    Address,
    AccessControllerAddress,
    AccountAddress,
    ComponentAddress,
    IdentityAddress,
    LockerAddress,
    PackageAddress,
    PoolAddress,
    ResourceAddress,
    ValidatorAddress,
    VaultAddress
);

impl From<NonFungibleGlobalId> for AddressBookEntryTarget {
    fn from(value: NonFungibleGlobalId) -> Self {
        Self::NonFungible { value }
    }
}

impl AddressBookEntryTarget {
    pub fn as_address(&self) -> Option<&Address> {
        match self {
            Self::Address { value } => Some(value),
            Self::NonFungible { .. } => None,
        }
    }

    pub fn is_account(&self) -> bool {
        self.as_address().is_some_and(Address::is_account)
    }
}

impl IsNetworkAware for AddressBookEntryTarget {
    fn network_id(&self) -> NetworkID {
        match self {
            Self::Address { value } => value.network_id(),
            Self::NonFungible { value } => value.resource_address.network_id(),
        }
    }
}

impl HasSampleValues for AddressBookEntryTarget {
    fn sample() -> Self {
        AccountAddress::sample_mainnet().into()
    }

    fn sample_other() -> Self {
        NonFungibleGlobalId::sample_other().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AddressBookEntryTarget;

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
    fn address_is_written_under_address() {
        assert_eq_after_json_roundtrip(
            &SUT::from(AccountAddress::sample_mainnet()),
            r#"{ "address": "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr" }"#,
        );
    }

    #[test]
    fn non_fungible_is_written_under_its_own_key() {
        assert_eq_after_json_roundtrip(
            &SUT::from(NonFungibleGlobalId::sample()),
            r#"{ "nonFungibleGlobalID": "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>" }"#,
        );
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(
            SUT::from(NonFungibleGlobalId::sample()).network_id(),
            NetworkID::Mainnet
        );
    }

    #[test]
    fn is_account() {
        assert!(SUT::sample().is_account());
        assert!(!SUT::from(ResourceAddress::sample_mainnet_xrd()).is_account());
        assert!(!SUT::sample_other().is_account());
    }
}
