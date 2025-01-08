use crate::prelude::*;

decl_ret_wrapped_address!(
    /// Addresses to a specific vault, owned by a user, holding asset of one kind, either fungible or non_fungible.
    /// Identities cannot own assets so they do not have vaults, but Accounts do, e.g.:
    /// `"internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq"`
    ///
    /// There are fundamentally two different sub-types ([Scrypto's `EntityType`][entt]) of VaultAddresses:
    /// * InternalFungibleVault
    /// * InternalNonFungibleVault
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalVaultAddress`][ret], and
    /// give it UniFFI support, as a ` uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L251-L255
    vault
);

#[uniffi::export]
pub fn vault_address_is_fungible(address: &VaultAddress) -> bool {
    address.into_internal().is_fungible()
}

#[uniffi::export]
pub fn vault_address_is_non_fungible(address: &VaultAddress) -> bool {
    address.into_internal().is_non_fungible()
}

#[uniffi::export]
pub fn new_vault_address_sample_mainnet_fungible() -> VaultAddress {
    InternalVaultAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_vault_address_sample_mainnet_non_fungible() -> VaultAddress {
    InternalVaultAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_vault_address_sample_stokenet_fungible() -> VaultAddress {
    InternalVaultAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_vault_address_sample_stokenet_non_fungible() -> VaultAddress {
    InternalVaultAddress::sample_stokenet_other().into()
}
