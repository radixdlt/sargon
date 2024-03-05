use crate::prelude::*;

use radix_engine_common::types::ComponentAddress as ScryptoComponentAddress;
use radix_engine_common::types::ResourceAddress as ScryptoResourceAddress;
use transaction::model::DynamicComponentAddress as ScryptoDynamicComponentAddress;
use transaction::model::DynamicResourceAddress as ScryptoDynamicResourceAddress;

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
