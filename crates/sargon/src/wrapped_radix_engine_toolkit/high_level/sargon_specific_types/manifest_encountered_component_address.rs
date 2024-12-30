use sargon_addresses::address_union;

use crate::prelude::*;

address_union!(
    /// A tagged union of all the encountered addresses in the manifest.
    /// This is to be primarily used for the "using dApps" section of the wallet's tx review screen.
    enum ManifestEncounteredComponentAddress: component, locker
);

macro_rules! impl_try_from_for_manifest_encountered_address {
    ($($variant:ident => $address_type:ty),*) => {
        impl TryFrom<(ScryptoGlobalAddress, NetworkID)> for ManifestEncounteredComponentAddress {
            type Error = CommonError;

            fn try_from(value: (ScryptoGlobalAddress, NetworkID)) -> Result<Self> {
                let (global_address, network_id) = value;

                $(
                    if let Ok(address) = <$address_type>::try_from((global_address, network_id)) {
                        return Ok(ManifestEncounteredComponentAddress::$variant(address));
                    }
                )*

                Err(CommonError::FailedToCreateAddressFromGlobalAddressAndNetworkID {
                    global_address_as_hex: global_address.to_hex(),
                    network_id: network_id.to_string(),
                })
            }
        }
    };
}

impl_try_from_for_manifest_encountered_address!(
    Component => ComponentAddress,
    Locker => LockerAddress
);

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ManifestEncounteredComponentAddress;

    #[test]
    fn sample_values_count() {
        let mut set = HashSet::<SUT>::new();
        set.extend(SUT::sample_values_all());
        // Duplicates should be removed
        set.extend(SUT::sample_values_all());

        assert_eq!(set.len(), 8);
    }

    #[test]
    fn try_from_failure() {
        let global_address = ScryptoGlobalAddress::new_or_panic(
            ResourceAddress::sample_stokenet()
                .scrypto()
                .into_node_id()
                .0,
        );
        let network_id = NetworkID::Stokenet;

        let result = SUT::try_from((global_address, network_id));
        assert_eq!(
            result.unwrap_err(),
            CommonError::FailedToCreateAddressFromGlobalAddressAndNetworkID {
                global_address_as_hex: global_address.to_hex(),
                network_id: network_id.to_string(),
            }
        );
    }

    macro_rules! generate_try_from_tests {
    ($($variant:ident => $address_type:ty),*) => {
        $(
            #[test]
            fn $variant() {
                let global_address = ScryptoGlobalAddress::new_or_panic(
                    <$address_type>::sample_stokenet().scrypto().into_node_id().0
                );
                let network_id = NetworkID::Stokenet;

                let result = SUT::try_from((global_address, network_id));
                assert!(result.is_ok());
            }
        )*
    };
}

    generate_try_from_tests!(
        component => ComponentAddress,
        locker => LockerAddress
    );
}
