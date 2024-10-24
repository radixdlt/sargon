use crate::prelude::*;

decl_ret_wrapped_address!(
    /// Address to a Validator that secures the network by validating transactions, users can stake to these
    /// validators (Delegated Proof of Stake) by using the Dashboard and sending a TX to the Radix Wallet to sign;
    /// e.g.:
    /// `"validator_rdx1sd5368vqdmjk0y2w7ymdts02cz9c52858gpyny56xdvzuheepdeyy0"`
    ///
    /// A `ValidatorAddress` has the [Scrypto's `EntityType`][entt] `GlobalValidator`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalValidatorAddress`][ret], and
    /// give it UniFFI support, as a ` uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L249-L250
    validator
);

#[uniffi::export]
pub fn new_validator_address_sample_mainnet() -> ValidatorAddress {
    InternalValidatorAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_validator_address_sample_mainnet_other() -> ValidatorAddress {
    InternalValidatorAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_validator_address_sample_stokenet() -> ValidatorAddress {
    InternalValidatorAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_validator_address_sample_stokenet_other() -> ValidatorAddress {
    InternalValidatorAddress::sample_stokenet_other().into()
}
