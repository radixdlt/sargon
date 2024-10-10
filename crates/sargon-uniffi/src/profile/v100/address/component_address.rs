use crate::prelude::*;

decl_ret_wrapped_address!(
    /// An address to some On-Ledger (OnNetwork) component, e.g. a Dapp, being an instantiation
    /// of some Scrypto blueprint, e.g:
    /// `"component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet"`
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of ComponentAddress:
    /// * GlobalGenericComponent
    /// * InternalGenericComponent
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalComponentAddress`][ret], and
    /// give it UniFFI support, as a ` uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L243-L246
    component
);

/// Returns `true` if the ComponentAddress is `global` (i.e. not `internal`)
#[uniffi::export]
pub fn component_address_is_global(address: &ComponentAddress) -> bool {
    address.into_internal().is_global()
}

/// Returns `true` if the ComponentAddress is `internal` (i.e. not `global`)
#[uniffi::export]
pub fn component_address_is_internal(address: &ComponentAddress) -> bool {
    address.into_internal().is_internal()
}

/// Sample to a mainnet ComponentAddress (global)
#[uniffi::export]
pub fn new_component_address_sample_mainnet_global() -> ComponentAddress {
    InternalComponentAddress::sample_mainnet().into()
}

/// Sample to a mainnet ComponentAddress (internal)
#[uniffi::export]
pub fn new_component_address_sample_mainnet_internal() -> ComponentAddress {
    InternalComponentAddress::sample_mainnet_other().into()
}

/// Sample to a stokenet ComponentAddress (global)
#[uniffi::export]
pub fn new_component_address_sample_stokenet_global() -> ComponentAddress {
    InternalComponentAddress::sample_stokenet().into()
}

/// Sample to a stokenet ComponentAddress (internal)
#[uniffi::export]
pub fn new_component_address_sample_stokenet_internal() -> ComponentAddress {
    InternalComponentAddress::sample_stokenet_other().into()
}

