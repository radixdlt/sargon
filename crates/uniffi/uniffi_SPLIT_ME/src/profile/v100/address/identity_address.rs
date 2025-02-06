use crate::prelude::*;

decl_ret_wrapped_address!(
    /// Human readable address of an identity, which are used by Personas. Always starts with
    /// the prefix `"identity_"`, for example:
    ///
    /// `identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j`
    ///
    /// Addresses are checksummed, as per Bech32. **Only** *Identity* addresses starts with
    /// the prefix `"identity_"`.
    ///
    /// There are fundamentally three different sub-types ([Scrypto's `EntityType`][entt]) of IdentityAddresses:
    /// * GlobalIdentity,
    /// * GlobalVirtualSecp256k1Identity,
    /// * GlobalVirtualEd25519Identity
    ///
    /// ```
    /// extern crate sargon;
    /// use sargon::prelude::*;
    ///
    /// assert_eq!(
    ///     "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j".parse::<IdentityAddress>().unwrap().network_id(),
    ///     NetworkID::Mainnet
    /// );
    /// ```
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalIdentityAddress`][ret], and
    /// give it UniFFI support, as a ` uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L229-L234
    identity
);

#[uniffi::export]
pub fn new_identity_address_from(
    public_key: PublicKey,
    network_id: NetworkID,
) -> IdentityAddress {
    InternalIdentityAddress::new_from_public_key(
        public_key.into(),
        network_id.into(),
    )
    .into()
}

#[uniffi::export]
pub fn new_identity_address_sample_mainnet() -> IdentityAddress {
    InternalIdentityAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_identity_address_sample_mainnet_other() -> IdentityAddress {
    InternalIdentityAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_identity_address_sample_stokenet() -> IdentityAddress {
    InternalIdentityAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_identity_address_sample_stokenet_other() -> IdentityAddress {
    InternalIdentityAddress::sample_stokenet_other().into()
}
