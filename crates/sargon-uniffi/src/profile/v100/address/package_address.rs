use crate::prelude::*;

decl_ret_wrapped_address!(
    /// The unique address identifying a package - which is a collection of blueprints on Ledger, e.g.:
    /// `"package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet"`
    ///
    /// PackageAddress has [Scrypto's `EntityType`][entt] type `GlobalPackage`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalPackageAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L241C29-L241C42
    package
);

#[uniffi::export]
pub fn new_package_address_sample_mainnet() -> PackageAddress {
    InternalAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_package_address_sample_mainnet_other() -> PackageAddress {
    InternalAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_package_address_sample_stokenet() -> PackageAddress {
    InternalAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_package_address_sample_stokenet_other() -> PackageAddress {
    InternalAddress::sample_stokenet_other().into()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PackageAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63";
        let address = new_package_address(b32.to_owned()).unwrap();
        assert_eq!(package_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(package_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_package_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_package_address_sample_mainnet(),
                new_package_address_sample_mainnet_other(),
                new_package_address_sample_stokenet(),
                new_package_address_sample_stokenet_other(),
                // duplicates should be removed
                new_package_address_sample_mainnet(),
                new_package_address_sample_mainnet_other(),
                new_package_address_sample_stokenet(),
                new_package_address_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }
}
