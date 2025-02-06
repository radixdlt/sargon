use crate::prelude::*;

macro_rules! from_scrypto_address_variant {

    ($address_type: ty) => {
        paste::paste! {
            impl From<([< Scrypto $address_type >], NetworkID)> for $address_type {
                fn from(value: ([< Scrypto $address_type >], NetworkID)) -> Self {
                    let target_type_name = stringify!($address_type);
                    $address_type::new_from_node_id(value.0.into_node_id(), value.1)
                    .expect(&format!("Should always be able to convert from Scrypto {} to Sargon {}", target_type_name, target_type_name))
                }
            }
        }
    };
}

from_scrypto_address_variant!(ResourceAddress);

macro_rules! from_scrypto_component_address {
    ($address_type: ty) => {
        paste::paste! {
            impl From<(ScryptoComponentAddress, NetworkID)> for $address_type {
                fn from(value: (ScryptoComponentAddress, NetworkID)) -> Self {
                    $address_type::new_from_node_id(value.0.into_node_id(), value.1)
                    .expect(&format!("Should always be able to convert from ScryptoComponentAddress to Sargon {}", stringify!($address_type)))
                }
            }
        }
    };
}

from_scrypto_component_address!(AccountAddress);
from_scrypto_component_address!(IdentityAddress);
from_scrypto_component_address!(ComponentAddress);
from_scrypto_component_address!(ValidatorAddress);
from_scrypto_component_address!(PoolAddress);
from_scrypto_component_address!(LockerAddress);

macro_rules! is_dynamic_component_address {
    ($address_type: ty) => {
        impl TryInto<ScryptoDynamicComponentAddress> for &$address_type {
            type Error = crate::CommonError;

            fn try_into(
                self,
            ) -> Result<ScryptoDynamicComponentAddress, Self::Error> {
                let scrypto = TryInto::<ScryptoComponentAddress>::try_into(self.node_id())
                    .map(ScryptoDynamicComponentAddress::Static)
                    .expect("Should always be able to convert between Sargon address to Scrypto's ComponentAddress");
                Ok(scrypto)
            }
        }
    };
}

macro_rules! is_dynamic_resource_address {
    ($address_type: ty) => {
        impl TryInto<ScryptoDynamicResourceAddress> for &$address_type {
            type Error = crate::CommonError;

            fn try_into(
                self,
            ) -> Result<ScryptoDynamicResourceAddress, Self::Error> {
                let scrypto = TryInto::<ScryptoResourceAddress>::try_into(self.node_id())
                    .map(ScryptoDynamicResourceAddress::Static)
                    .expect("Should always be able to convert between Sargon address to Scrypto's ResourceAddress");

                Ok(scrypto)
            }
        }
    };
}

impl TryInto<ScryptoDynamicGlobalAddress> for &AddressOfAccountOrPersona {
    type Error = crate::CommonError;
    fn try_into(self) -> Result<ScryptoDynamicGlobalAddress, Self::Error> {
        match self {
            AddressOfAccountOrPersona::Account(value) => {
                TryInto::<ScryptoDynamicGlobalAddress>::try_into(value)
            }
            AddressOfAccountOrPersona::Identity(value) => {
                TryInto::<ScryptoDynamicGlobalAddress>::try_into(value)
            }
        }
    }
}
is_dynamic_component_address!(AccountAddress);
is_dynamic_component_address!(AccessControllerAddress);
is_dynamic_component_address!(ComponentAddress);
is_dynamic_component_address!(IdentityAddress);
is_dynamic_component_address!(ValidatorAddress);
is_dynamic_component_address!(VaultAddress);
is_dynamic_component_address!(LockerAddress);

is_dynamic_resource_address!(ResourceAddress);
is_dynamic_resource_address!(NonFungibleResourceAddress);

impl From<ResourceAddress> for ScryptoResourceAddress {
    fn from(value: ResourceAddress) -> Self {
        ScryptoResourceAddress::try_from(value.node_id()).expect(
            "Should always be able to convert to Scrypto ResourceAddress",
        )
    }
}

impl From<ResourceOrNonFungible> for ScryptoResourceOrNonFungible {
    fn from(value: ResourceOrNonFungible) -> Self {
        match value {
            ResourceOrNonFungible::Resource { value } => {
                ScryptoResourceOrNonFungible::Resource(value.into())
            }
            ResourceOrNonFungible::NonFungible { value } => {
                ScryptoResourceOrNonFungible::NonFungible(value.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_address_to_scrypto_component_address() {
        let roundtrip = |a: &AccountAddress| {
            let scrypto: ScryptoDynamicComponentAddress = a.try_into().unwrap();
            match scrypto {
                ScryptoDynamicComponentAddress::Static(static_scrypto) => {
                    assert_eq!(
                        &AccountAddress::new_from_node_id(
                            static_scrypto.into_node_id(),
                            a.network_id()
                        )
                        .unwrap(),
                        a
                    );
                }
                ScryptoDynamicComponentAddress::Named(_) => {
                    panic!("wrong kind")
                }
            }
        };
        roundtrip(&AccountAddress::sample());
        roundtrip(&AccountAddress::sample_other());
    }

    #[test]
    fn resource_address_to_scrypto_resource_address() {
        let roundtrip = |a: &ResourceAddress| {
            let scrypto: ScryptoDynamicResourceAddress = a.try_into().unwrap();
            match scrypto {
                ScryptoDynamicResourceAddress::Static(static_scrypto) => {
                    assert_eq!(
                        &ResourceAddress::new_from_node_id(
                            static_scrypto.into_node_id(),
                            a.network_id()
                        )
                        .unwrap(),
                        a
                    );
                }
                ScryptoDynamicResourceAddress::Named(_) => {
                    panic!("wrong kind")
                }
            }
        };
        roundtrip(&ResourceAddress::sample());
        roundtrip(&ResourceAddress::sample_other());
    }
}
