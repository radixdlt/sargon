use crate::prelude::*;

decl_ret_wrapped_address!(
    /// Addresses identifying an asset, either fungible (Token) or non-fungible (NFT), on the Radix network, e.g.
    /// `"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"`
    /// Being the unique identifier of the Radix Token, the Rad, on mainnet.
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of ResourceAddresses:
    /// * GlobalFungibleResourceManager
    /// * GlobalNonFungibleResourceManager
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalResourceAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L236-L239
    resource
);

#[uniffi::export]
pub fn resource_address_is_fungible(address: &ResourceAddress) -> bool {
    address.into_internal().is_fungible()
}

#[uniffi::export]
pub fn resource_address_is_non_fungible(address: &ResourceAddress) -> bool {
    address.into_internal().is_non_fungible()
}

#[uniffi::export]
pub fn new_resource_address_sample_mainnet_xrd() -> ResourceAddress {
    InternalAddress::sample_mainnet_xrd().into()
}

#[uniffi::export]
pub fn new_resource_address_sample_mainnet_candy() -> ResourceAddress {
    InternalAddress::sample_mainnet_candy().into()
}

#[uniffi::export]
pub fn new_resource_address_sample_mainnet_nft_gc_membership() -> ResourceAddress
{
    InternalAddress::sample_mainnet_nft_gc_membership().into()
}

#[uniffi::export]
pub fn new_resource_address_sample_stokenet_xrd() -> ResourceAddress {
    InternalAddress::sample_stokenet_xrd().into()
}

#[uniffi::export]
pub fn new_resource_address_sample_stokenet_gum() -> ResourceAddress {
    InternalAddress::sample_stokenet_gum().into()
}

#[uniffi::export]
pub fn new_resource_address_sample_stokenet_gc_tokens() -> ResourceAddress {
    InternalAddress::sample_stokenet_gc_tokens().into()
}

#[uniffi::export]
pub fn new_resource_address_sample_stokenet_candy() -> ResourceAddress {
    InternalAddress::sample_stokenet_candy().into()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd";
        let address = new_resource_address(b32.to_owned()).unwrap();
        assert_eq!(SUT::try_from_bech32(b32).unwrap(), address);
        assert_eq!(resource_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(resource_address_bech32_address(&address), b32);
    }

    #[test]
    fn is_fungible() {
        assert!(!resource_address_is_fungible(
            &SUT::sample_mainnet_nft_gc_membership()
        ));
        assert!(resource_address_is_non_fungible(
            &SUT::sample_mainnet_nft_gc_membership()
        ));

        assert!(resource_address_is_fungible(&SUT::sample_mainnet_xrd()));
        assert!(!resource_address_is_non_fungible(&SUT::sample_mainnet_xrd()));
    }

    #[test]
    fn sample() {
        assert_eq!(
            new_resource_address_sample_mainnet_xrd(),
            SUT::sample_mainnet_xrd()
        );
        assert_eq!(
            new_resource_address_sample_mainnet_candy(),
            SUT::sample_mainnet_candy()
        );
        assert_eq!(
            new_resource_address_sample_mainnet_nft_gc_membership(),
            SUT::sample_mainnet_nft_gc_membership()
        );
        assert_eq!(
            new_resource_address_sample_stokenet_xrd(),
            SUT::sample_stokenet_xrd()
        );
        assert_eq!(
            new_resource_address_sample_stokenet_gum(),
            SUT::sample_stokenet_gum()
        );
        assert_eq!(
            new_resource_address_sample_stokenet_gc_tokens(),
            SUT::sample_stokenet_gc_tokens()
        );
        assert_eq!(
            new_resource_address_sample_stokenet_candy(),
            SUT::sample_stokenet_candy()
        );
    }
}
