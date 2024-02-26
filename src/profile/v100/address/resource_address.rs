use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalResourceAddress as RetResourceAddress;

impl ResourceAddress {
    pub fn is_fungible(&self) -> bool {
        self.secret_magic.is_fungible()
    }

    pub fn is_non_fungible(&self) -> bool {
        self.secret_magic.is_non_fungible()
    }
}

#[uniffi::export]
pub fn resource_address_is_fungible(address: &ResourceAddress) -> bool {
    address.is_fungible()
}

#[uniffi::export]
pub fn resource_address_is_non_fungible(address: &ResourceAddress) -> bool {
    address.is_non_fungible()
}

#[uniffi::export]
pub fn new_resource_address_placeholder_mainnet_xrd() -> ResourceAddress {
    ResourceAddress::placeholder_mainnet_xrd()
}

#[uniffi::export]
pub fn new_resource_address_placeholder_mainnet_candy() -> ResourceAddress {
    ResourceAddress::placeholder_mainnet_candy()
}

#[uniffi::export]
pub fn new_resource_address_placeholder_mainnet_nft_gc_membership(
) -> ResourceAddress {
    ResourceAddress::placeholder_mainnet_nft_gc_membership()
}

#[uniffi::export]
pub fn new_resource_address_placeholder_stokenet_xrd() -> ResourceAddress {
    ResourceAddress::placeholder_stokenet_xrd()
}

#[uniffi::export]
pub fn new_resource_address_placeholder_stokenet_gum() -> ResourceAddress {
    ResourceAddress::placeholder_stokenet_gum()
}

#[uniffi::export]
pub fn new_resource_address_placeholder_stokenet_gc_tokens() -> ResourceAddress
{
    ResourceAddress::placeholder_stokenet_gc_tokens()
}

#[uniffi::export]
pub fn new_resource_address_placeholder_stokenet_candy() -> ResourceAddress {
    ResourceAddress::placeholder_stokenet_candy()
}

impl HasPlaceholder for ResourceAddress {
    /// The RAD on mainnet
    fn placeholder() -> Self {
        Self::placeholder_mainnet_xrd()
    }

    /// Candy by Gumball club on mainnet
    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_candy()
    }
}

#[allow(unused)]
impl ResourceAddress {
    fn placeholder_stokenet() -> Self {
        Self::placeholder_stokenet_xrd()
    }

    fn placeholder_stokenet_other() -> Self {
        Self::placeholder_stokenet_gum()
    }
}

#[allow(unused)]
impl ResourceAddress {
    /// The RAD on mainnet
    fn placeholder_mainnet_xrd() -> Self {
        "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
            .parse()
            .expect("XRD")
    }

    /// Candy by Gumball club on mainnet
    fn placeholder_mainnet_candy() -> Self {
        "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
            .parse()
            .expect("Candy")
    }

    fn placeholder_mainnet_nft_gc_membership() -> Self {
        "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa"
            .parse()
            .expect("GC Membership")
    }

    fn placeholder_stokenet_xrd() -> Self {
        "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"
            .parse()
            .expect("XRD")
    }

    fn placeholder_stokenet_gum() -> Self {
        "resource_tdx_2_1t4kep9ldg9t0cszj78z6fcr2zvfxfq7muetq7pyvhdtctwxum90scq"
            .parse()
            .expect("Gum")
    }

    fn placeholder_stokenet_gc_tokens() -> Self {
        "resource_tdx_2_1thqcgjw37fjgycpvqr52nx4jcsdeuq75mf2nywme07kzsuds9a4psp"
            .parse()
            .expect("GC Tokens")
    }

    fn placeholder_stokenet_candy() -> Self {
        "resource_tdx_2_1tk30vj4ene95e3vhymtf2p35fzl29rv4us36capu2rz0vretw9gzr3"
            .parse()
            .expect("Candy")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());

        assert_eq!(SUT::placeholder_stokenet(), SUT::placeholder_stokenet());
        assert_eq!(
            SUT::placeholder_stokenet_other(),
            SUT::placeholder_stokenet_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
        assert_ne!(SUT::placeholder_stokenet(), SUT::placeholder());
    }

    #[test]
    fn is_fungible() {
        assert!(SUT::placeholder_mainnet_nft_gc_membership().is_non_fungible());
        assert!(!SUT::placeholder_mainnet_nft_gc_membership().is_fungible());

        assert!(!SUT::placeholder_mainnet_xrd().is_non_fungible());
        assert!(SUT::placeholder_mainnet_xrd().is_fungible());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::placeholder_mainnet_xrd(),
                SUT::placeholder_mainnet_candy(),
                SUT::placeholder_mainnet_nft_gc_membership(),
                SUT::placeholder_stokenet_xrd(),
                SUT::placeholder_stokenet_gc_tokens(),
                SUT::placeholder_stokenet_gum(),
                SUT::placeholder_stokenet_candy(),
                // twice => duplicates should be removed
                SUT::placeholder_mainnet_xrd(),
                SUT::placeholder_mainnet_candy(),
                SUT::placeholder_mainnet_nft_gc_membership(),
                SUT::placeholder_stokenet_xrd(),
                SUT::placeholder_stokenet_gc_tokens(),
                SUT::placeholder_stokenet_gum(),
                SUT::placeholder_stokenet_candy(),
            ])
            .len(),
            7
        )
    }

    #[test]
    fn display() {
        let s = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{:?}", a), s);
    }

    #[test]
    fn manual_perform_uniffi_conversion() {
        type RetAddr = <SUT as FromRetAddress>::RetAddress;
        let sut = SUT::placeholder();
        let bech32 = sut.to_string();
        let ret = RetAddr::try_from_bech32(&bech32).unwrap();

        let ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::from_custom(ret);
        assert_eq!(ffi_side, bech32);
        let from_ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::into_custom(
                ffi_side,
            )
            .unwrap();
        assert_eq!(ret, from_ffi_side);
    }

    #[test]
    fn json_roundtrip() {
        let a = SUT::placeholder();
        assert_json_value_eq_after_roundtrip(
            &a,
            json!("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxxx")
        );
        assert_json_value_fails::<SUT>(
            json!("account_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "resource_tdx_2_1tkckx9fynl9f7756z8wxphq7wce6vk874nuq4f2nnxgh3nzrwhjdlp"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: SUT =
            "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Mainnet);
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd";
        let address = new_resource_address(b32.to_owned()).unwrap();
        assert_eq!(SUT::try_from_bech32(b32).unwrap(), address);
        assert_eq!(resource_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(resource_address_bech32_address(&address), b32);
    }

    #[test]
    fn is_fungible() {
        assert!(!resource_address_is_fungible(
            &SUT::placeholder_mainnet_nft_gc_membership()
        ));
        assert!(resource_address_is_non_fungible(
            &SUT::placeholder_mainnet_nft_gc_membership()
        ));

        assert!(resource_address_is_fungible(&SUT::placeholder_mainnet_xrd()));
        assert!(!resource_address_is_non_fungible(
            &SUT::placeholder_mainnet_xrd()
        ));
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            new_resource_address_placeholder_mainnet_xrd(),
            SUT::placeholder_mainnet_xrd()
        );
        assert_eq!(
            new_resource_address_placeholder_mainnet_candy(),
            SUT::placeholder_mainnet_candy()
        );
        assert_eq!(
            new_resource_address_placeholder_mainnet_nft_gc_membership(),
            SUT::placeholder_mainnet_nft_gc_membership()
        );
        assert_eq!(
            new_resource_address_placeholder_stokenet_xrd(),
            SUT::placeholder_stokenet_xrd()
        );
        assert_eq!(
            new_resource_address_placeholder_stokenet_gum(),
            SUT::placeholder_stokenet_gum()
        );
        assert_eq!(
            new_resource_address_placeholder_stokenet_gc_tokens(),
            SUT::placeholder_stokenet_gc_tokens()
        );
        assert_eq!(
            new_resource_address_placeholder_stokenet_candy(),
            SUT::placeholder_stokenet_candy()
        );
    }
}
