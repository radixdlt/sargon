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
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radix-engine-toolkit/blob/476d779fee08ed1e561ac8fc8730a2a404b7de79/crates/radix-engine-toolkit-uniffi/src/common/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/476d779fee08ed1e561ac8fc8730a2a404b7de79/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L262-L265
    locker
);

#[uniffi::export]
pub fn new_locker_address_sample_mainnet() -> LockerAddress {
    LockerAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_locker_address_sample_mainnet_other() -> LockerAddress {
    LockerAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_locker_address_sample_stokenet() -> LockerAddress {
    LockerAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_locker_address_sample_stokenet_other() -> LockerAddress {
    LockerAddress::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LockerAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 =
            "locker_rdx1drn4q2zk6dvljehytnhfah330xk7emfznv59rqlps5ayy52d7xkzzz";
        let address = new_locker_address(b32.to_owned()).unwrap();
        assert_eq!(SUT::try_from_bech32(b32).unwrap(), address);
        assert_eq!(locker_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(locker_address_bech32_address(&address), b32);
    }

    #[test]
    fn sample() {
        assert_eq!(new_locker_address_sample_mainnet(), SUT::sample_mainnet());

        assert_eq!(
            new_locker_address_sample_mainnet_other(),
            SUT::sample_mainnet_other()
        );

        assert_eq!(
            new_locker_address_sample_stokenet(),
            SUT::sample_stokenet()
        );

        assert_eq!(
            new_locker_address_sample_stokenet_other(),
            SUT::sample_stokenet_other()
        );
    }
}
