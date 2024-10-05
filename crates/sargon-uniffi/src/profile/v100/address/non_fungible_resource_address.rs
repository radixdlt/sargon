use crate::prelude::*;

/// This macro exists since UniFFI does not support generics currently, when/if
/// UniFFI does, we SHOULD remove this macro and use generics.
macro_rules! decl_specialized_address {
    (
        $(
            #[doc = $expr: expr]
        )*
        $specialized_address_type: ident,
        $base_addr: ty,
        $validate: ident,
        $validation_err: ident
    ) => {

        paste! {
            $(
                #[doc = $expr]
            )*
            #[derive(
                Clone,
                Copy,
                PartialEq,
                Eq,
                Hash,
                derive_more::Display,
                derive_more::Debug,
                uniffi::Record,
            )]
            #[debug("{:?}", self.secret_magic)]
            pub struct $specialized_address_type {
                secret_magic: $base_addr
            }

            /// Tries to bech32 decode the string into a specialized address.
            #[uniffi::export]
            pub fn [< new_ $specialized_address_type:snake >](bech32: String) -> Result<$specialized_address_type> {
                $base_addr::try_from_bech32(&bech32).and_then(TryInto::<$specialized_address_type>::try_into)
            }

            /// Returns the base address of this specialized address.
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _as_ $base_addr:snake>](address: &$specialized_address_type) -> $base_addr {
                address.secret_magic
            }

            /// Returns a new address, with the same node_id, but using `network_id` as
            /// network.
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _map_to_network >](address: &$specialized_address_type, network_id: NetworkID) -> $specialized_address_type {
                address.map_to_network(network_id)
            }

            /// Returns the bech32 encoding of this address
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _bech32_address >](address: &$specialized_address_type) -> String {
                address.to_string()
            }

            /// Returns the network id this address
            #[uniffi::export]
            pub fn [< $specialized_address_type:snake _network_id >](address: &$specialized_address_type) -> NetworkID {
                address.secret_magic.network_id()
            }


            #[cfg(test)]
            mod [<uniffi_tests_of_ $specialized_address_type:snake>] {
                use super::*;

                #[allow(clippy::upper_case_acronyms)]
                type SUT = $specialized_address_type;

                #[test]
                fn map_to_network() {
                    let sut = SUT::sample();
                    assert_eq!([< $specialized_address_type:snake _map_to_network >](&sut, sut.network_id()), sut); // unchanged
                }
            }

        }
    };
}

decl_specialized_address!(
    /// NonFungibleResourceAddress is a specialized ResourceAddress for resources
    /// which are non fungible, it ALWAYS has an `'n'` after bech32 separator `'1'`, e.g.:
    /// `"resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"`.
    ///
    /// As opposed to a fungible resource address, e.g. that of XRD which has `'t'`
    /// after bech32 separator `'1'`, see:
    /// `"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"`
    ///
    /// This means that given an instance of `NonFungibleResourceAddress`, it is
    /// guaranteed that its entity type is [`::GlobalNonFungibleResourceManager`],
    /// and not `::GlobalFungibleResourceManager`.
    ///
    /// This type can safely be used with [`StakeClaim`]s, unfortunately since Radix Engine
    /// and/or network does not validate the resource address of a `NonFungibleGlobalId`,
    /// we cannot use this for that type.
    NonFungibleResourceAddress,
    ResourceAddress,
    is_non_fungible,
    FungibleResourceAddressNotAcceptedInNonFungibleContext
);


#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_mainnet(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_mainnet_other(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_stokenet(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_stokenet_other(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_stokenet_other()
}

/// Returns a random address in `network_id` as Network
#[uniffi::export]
pub fn new_non_fungible_resource_address_random(
    network_id: NetworkID,
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::random(network_id)
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleResourceAddress;

    #[test]
    fn from_bech32() {
        assert_eq!(new_non_fungible_resource_address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa".to_owned()).unwrap(), SUT::sample());
        assert_eq!(new_non_fungible_resource_address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd".to_owned()).unwrap(), SUT::sample_other());
    }

    #[test]
    fn to_bech32() {
        assert_eq!(non_fungible_resource_address_bech32_address(&SUT::sample()), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa");
        assert_eq!(non_fungible_resource_address_bech32_address(&SUT::sample_other()), "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd");
    }

    #[test]
    fn network_id() {
        assert_eq!(
            non_fungible_resource_address_network_id(&SUT::sample_mainnet()),
            NetworkID::Mainnet
        );
        assert_eq!(
            non_fungible_resource_address_network_id(&SUT::sample_stokenet()),
            NetworkID::Stokenet
        );
    }

    #[test]
    fn hash_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_non_fungible_resource_address_sample_mainnet(),
                new_non_fungible_resource_address_sample_mainnet_other(),
                new_non_fungible_resource_address_sample_stokenet(),
                new_non_fungible_resource_address_sample_stokenet_other(),
                // duplicates should be removed
                new_non_fungible_resource_address_sample_mainnet(),
                new_non_fungible_resource_address_sample_mainnet_other(),
                new_non_fungible_resource_address_sample_stokenet(),
                new_non_fungible_resource_address_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }

    #[test]
    fn random_address() {
        let n = 10;
        for network_id in NetworkID::all() {
            let addresses = (0..n)
                .map(|_| new_non_fungible_resource_address_random(network_id))
                .collect::<HashSet<SUT>>();
            assert_eq!(addresses.len(), n);
        }
    }
}
