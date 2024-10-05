use crate::prelude::*;

decl_ret_wrapped_address!(
    /// Address to an AccessController that controls an Account or Identity (Persona),
    /// it said entity has been "securified", e.g.:
    /// `"accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a"`
    ///
    /// When a user applies a SecurityStructureConfiguration for the first time on a
    /// non-securified entity (and signs and submit the resulting TX) said entity is
    /// "assigned" an AccessControllerAddress by the network.
    ///
    /// An `AccessControllerAddress` has the [Scrypto's `EntityType`][entt] `GlobalAccessController`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalAccessControllerAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L247-L248
    accessController
);

#[uniffi::export]
pub fn new_access_controller_address_sample_mainnet() -> AccessControllerAddress
{
    AccessControllerAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_access_controller_address_sample_mainnet_other(
) -> AccessControllerAddress {
    AccessControllerAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_access_controller_address_sample_stokenet() -> AccessControllerAddress
{
    AccessControllerAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_access_controller_address_sample_stokenet_other(
) -> AccessControllerAddress {
    AccessControllerAddress::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessControllerAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s";
        let address = new_access_controller_address(b32.to_owned()).unwrap();
        assert_eq!(
            access_controller_address_network_id(&address),
            NetworkID::Mainnet
        );
        assert_eq!(access_controller_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_access_controller_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_access_controller_address_sample_mainnet(),
                new_access_controller_address_sample_mainnet_other(),
                new_access_controller_address_sample_stokenet(),
                new_access_controller_address_sample_stokenet_other(),
                // duplicates should be removed
                new_access_controller_address_sample_mainnet(),
                new_access_controller_address_sample_mainnet_other(),
                new_access_controller_address_sample_stokenet(),
                new_access_controller_address_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }
}
