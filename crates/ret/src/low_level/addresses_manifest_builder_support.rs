use crate::prelude::*;

macro_rules! from_scrypto_global_address {
    ($address_type: ty) => {
        paste! {
            impl TryFrom<(ScryptoGlobalAddress, NetworkID)> for $address_type {
                type Error = crate::CommonError;
                fn try_from(value: (ScryptoGlobalAddress, NetworkID)) -> Result<Self> {
                    <$address_type as AddressViaRet>::new(value.0.into_node_id(), value.1)
                }
            }
        }
    };
}

from_scrypto_global_address!(PackageAddress);
from_scrypto_global_address!(ResourceAddress);
from_scrypto_global_address!(NonFungibleResourceAddress);
from_scrypto_global_address!(ValidatorAddress);
from_scrypto_global_address!(AccessControllerAddress);
from_scrypto_global_address!(AccountAddress);
from_scrypto_global_address!(IdentityAddress);
from_scrypto_global_address!(ComponentAddress);

macro_rules! from_scrypto_address_variant {

    ($address_type: ty) => {
        paste! {
            impl From<([< Scrypto $address_type >], NetworkID)> for $address_type {
                fn from(value: ([< Scrypto $address_type >], NetworkID)) -> Self {
                    let target_type_name = stringify!($address_type);
                    <$address_type as AddressViaRet>::new(value.0.into_node_id(), value.1)
                    .expect(&format!("Should always be able to convert from Scrypto {} to Sargon {}", target_type_name, target_type_name))
                }
            }
        }
    };
}

from_scrypto_address_variant!(ResourceAddress);

macro_rules! from_scrypto_component_address {
    ($address_type: ty) => {
        paste! {
            impl From<(ScryptoComponentAddress, NetworkID)> for $address_type {
                fn from(value: (ScryptoComponentAddress, NetworkID)) -> Self {
                    <$address_type as AddressViaRet>::new(value.0.into_node_id(), value.1)
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

is_dynamic_component_address!(AccountAddress);
is_dynamic_component_address!(AccessControllerAddress);
is_dynamic_component_address!(ComponentAddress);
is_dynamic_component_address!(IdentityAddress);
is_dynamic_component_address!(ValidatorAddress);
is_dynamic_component_address!(VaultAddress);

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

#[cfg(not(tarpaulin_include))] // false negative, tested.
pub fn to_vec_network_aware<T, U>(
    values: impl IntoIterator<Item = T>,
    network_id: NetworkID,
) -> Vec<U>
where
    U: From<(T, NetworkID)>,
{
    values
        .into_iter()
        .map(|x| (x, network_id))
        .map(U::from)
        .collect_vec()
}

pub fn to_hashmap_network_aware_key<K, V, L, U>(
    values: impl IntoIterator<Item = (K, V)>,
    network_id: NetworkID,
) -> HashMap<L, U>
where
    L: Eq + std::hash::Hash + From<(K, NetworkID)>,
    U: From<V>,
{
    values
        .into_iter()
        .map(|(k, v)| (L::from((k, network_id)), U::from(v)))
        .collect::<HashMap<L, U>>()
}

#[cfg(not(tarpaulin_include))] // false negative, tested.
pub fn filter_try_to_vec_network_aware<T, U>(
    values: impl IntoIterator<Item = T>,
    network_id: NetworkID,
) -> Vec<U>
where
    U: TryFrom<(T, NetworkID)>,
{
    values
        .into_iter()
        .map(|x| (x, network_id))
        .map(U::try_from)
        .filter_map(Result::ok)
        .collect_vec()
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
                        &<AccountAddress as AddressViaRet>::new(
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
                        &<ResourceAddress as AddressViaRet>::new(
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
