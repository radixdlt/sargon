use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalPackageAddress as RetPackageAddress;

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
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::FromStr,
    derive_more::Display,
    SerializeDisplay,
    DeserializeFromStr,
    uniffi::Record,
)]
#[display("{secret_magic}")]
pub struct PackageAddress {
    /// @Kotlin / Swift developer: Do NOT use this property/field. Instead use all the provided methods on this address type.
    /// (which are in fact vendored as freestanding global functions,
    /// due to limitations in UniFII as of Feb 2024, but you should
    /// create extension methods on this address type in FFI land, translating
    /// these functions into methods.)
    pub(crate) secret_magic: RetPackageAddress,
}

#[uniffi::export]
pub fn new_package_address_placeholder() -> PackageAddress {
    PackageAddress::placeholder()
}

#[uniffi::export]
pub fn new_package_address_placeholder_other() -> PackageAddress {
    PackageAddress::placeholder_other()
}

impl HasPlaceholder for PackageAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet_gumball_club()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_stokenet_gumball_club()
    }
}

impl PackageAddress {
    pub fn placeholder_mainnet_gumball_club() -> Self {
        "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet"
            .parse()
            .expect("Valid Mainnet package placeholder address")
    }
    pub fn placeholder_stokenet_gumball_club() -> Self {
        "package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay"
            .parse()
            .expect("Valid Stokenet package placeholder address")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PackageAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn display() {
        let s = "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: PackageAddress =
            "package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ax")
        );
        assert_json_value_fails::<SUT>(
            json!("account_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "package_tdx_2_1pkaw4m82c89hy0gk4dwqtqlln6md8anr2ysnrvegxar53mr6nvn5ay"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: PackageAddress =
            "package_rdx1p589ehmmvqa2dnw0jaky3kesjdjvln94hzunsqse8k52083hfcjh63"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Mainnet);
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{new_resource_address, EntityAddress};

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
    fn placeholder() {
        assert_eq!(new_package_address_placeholder(), SUT::placeholder());

        assert_eq!(
            new_package_address_placeholder_other(),
            SUT::placeholder_other()
        );
    }
}
