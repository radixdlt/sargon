use crate::prelude::*;

decl_ret_wrapped_address!(
    /// The unique address identifying a package - which is a collection of blueprints on Ledger, e.g.:
    /// `"package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet"`
    ///
    /// PackageAddress has [Scrypto's `EntityType`][entt] type `GlobalPackage`.
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalPackageAddress`][ret], and
    /// give it UniFFI support, as a ` uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L241C29-L241C42
    package
);

#[uniffi::export]
pub fn new_package_address_sample_mainnet() -> PackageAddress {
    InternalPackageAddress::sample_mainnet().into()
}

#[uniffi::export]
pub fn new_package_address_sample_mainnet_other() -> PackageAddress {
    InternalPackageAddress::sample_mainnet_other().into()
}

#[uniffi::export]
pub fn new_package_address_sample_stokenet() -> PackageAddress {
    InternalPackageAddress::sample_stokenet().into()
}

#[uniffi::export]
pub fn new_package_address_sample_stokenet_other() -> PackageAddress {
    InternalPackageAddress::sample_stokenet_other().into()
}

