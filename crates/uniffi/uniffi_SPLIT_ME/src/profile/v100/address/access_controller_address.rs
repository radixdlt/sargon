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
    /// give it UniFFI support, as a ` uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L247-L248
    accessController
);

#[uniffi::export]
pub fn new_access_controller_address_sample_mainnet() -> AccessControllerAddress
{
    InternalAccessControllerAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_access_controller_address_sample_mainnet_other(
) -> AccessControllerAddress {
    InternalAccessControllerAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_access_controller_address_sample_stokenet() -> AccessControllerAddress
{
    InternalAccessControllerAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_access_controller_address_sample_stokenet_other(
) -> AccessControllerAddress {
    InternalAccessControllerAddress::sample_stokenet_other().into()
}
