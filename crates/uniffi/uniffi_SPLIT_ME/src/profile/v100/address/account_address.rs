pub use crate::prelude::*;

decl_ret_wrapped_address!(
    /// Human readable address of an account. Always starts with `"account_"``, for example:
    ///
    /// `account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr`
    ///
    /// Most commonly the user will see this address in its abbreviated
    /// form which is:
    ///
    /// `acco...nvjdwr`
    ///
    /// Addresses are checksummed, as per Bech32. **Only** *Account* addresses starts with
    /// the prefix `account_`.
    ///
    /// There are fundamentally three different sub-types ([Scrypto's `EntityType`][entt]) of AccountAddresses:
    /// * GlobalAccount
    /// * GlobalVirtualSecp256k1Account
    /// * GlobalVirtualEd25519Account
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalAccountAddress`][ret], and
    /// give it UniFFI support, as a ` uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L224-L228
    account
);

#[uniffi::export]
pub fn new_account_address_from(
    public_key: PublicKey,
    network_id: NetworkID,
) -> AccountAddress {
    InternalAccountAddress::new_from_public_key(
        public_key.into_internal(),
        network_id.into_internal(),
    )
    .into()
}

#[uniffi::export]
pub fn new_account_address_sample_mainnet() -> AccountAddress {
    InternalAccountAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_account_address_sample_mainnet_other() -> AccountAddress {
    InternalAccountAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_account_address_sample_stokenet() -> AccountAddress {
    InternalAccountAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_account_address_sample_stokenet_other() -> AccountAddress {
    InternalAccountAddress::sample_stokenet_other().into()
}

/// Returns `false` for all addresses created with `Ed25519PublicKey`s, i.e.
/// for all accounts created by the Babylon Radix Wallets.
/// Returns `true` for all addresses created with `Secp256k1PublicKey`s, i.e.
/// imported from the legacy Olympia desktop application.
#[uniffi::export]
pub fn account_address_is_legacy(address: &AccountAddress) -> bool {
    address.into_internal().is_legacy_address()
}
