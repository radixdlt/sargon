use crate::prelude::*;

decl_ret_wrapped_address!(
    /// Addresses to a specific locker, owned by a dApp, holding assets, either fungible or non-fungible,
    /// that can be claimed by destined account addresses.
    /// Identities cannot own assets so they do not have vaults, but Accounts do, e.g.:
    /// e.g.:
    /// `"locker_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"`
    ///
    /// A `LockerAddress` has the [Scrypto's `EntityType`][entt] `GlobalAccountLocker`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalLockerAddress`][ret], and
    /// give it UniFFI support, as a ` uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radix-engine-toolkit/blob/476d779fee08ed1e561ac8fc8730a2a404b7de79/crates/radix-engine-toolkit-uniffi/src/common/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/476d779fee08ed1e561ac8fc8730a2a404b7de79/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L262-L265
    locker
);

#[uniffi::export]
pub fn new_locker_address_sample_mainnet() -> LockerAddress {
    InternalAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_locker_address_sample_mainnet_other() -> LockerAddress {
    InternalAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_locker_address_sample_stokenet() -> LockerAddress {
    InternalAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_locker_address_sample_stokenet_other() -> LockerAddress {
    InternalAddress::sample_stokenet_other().into()
}

