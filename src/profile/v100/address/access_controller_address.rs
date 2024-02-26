use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalAccessControllerAddress as RetAccessControllerAddress;

#[uniffi::export]
pub fn new_access_controller_address_placeholder() -> AccessControllerAddress {
    AccessControllerAddress::placeholder()
}

#[uniffi::export]
pub fn new_access_controller_address_placeholder_other(
) -> AccessControllerAddress {
    AccessControllerAddress::placeholder_other()
}

impl HasPlaceholder for AccessControllerAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_stokenet()
    }
}

impl AccessControllerAddress {
    pub fn placeholder_mainnet() -> Self {
        "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a".parse().expect("Placeholder")
    }

    pub fn placeholder_mainnet_other() -> Self {
        "accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak".parse().expect("Placeholder")
    }

    pub fn placeholder_stokenet() -> Self {
        "accesscontroller_tdx_2_1cw68j9ca4fye09mz3hshp4qydjnxhsahm68hvmz9cjhftcz9f53juq".parse().expect("Placeholder")
    }

    pub fn placeholder_stokenet_other() -> Self {
        "accesscontroller_tdx_2_1c0llllllllllllllllllllllllllllllllllllllllllllllhcg0ny".parse().expect("Placeholder")
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_common::address::AddressBech32DecodeError;
    use radix_engine_toolkit::models::canonical_address_types::{
        CanonicalAccessControllerAddress, CanonicalAddress,
        CanonicalAddressError,
    };

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessControllerAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder_mainnet(), SUT::placeholder_mainnet());
        assert_eq!(
            SUT::placeholder_mainnet_other(),
            SUT::placeholder_mainnet_other()
        );
        assert_eq!(SUT::placeholder_stokenet(), SUT::placeholder_stokenet());
        assert_eq!(
            SUT::placeholder_stokenet_other(),
            SUT::placeholder_stokenet_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
        assert_ne!(
            SUT::placeholder_mainnet(),
            SUT::placeholder_mainnet_other()
        );
        assert_ne!(SUT::placeholder_mainnet(), SUT::placeholder_stokenet());
        assert_ne!(
            SUT::placeholder_mainnet_other(),
            SUT::placeholder_stokenet_other()
        );
    }

    #[test]
    fn display() {
        let s = "accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s = "accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt";
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
        let a: SUT =
            "accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2vxx")
        );
        assert_json_value_fails::<SUT>(
            json!("account_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "accesscontroller_tdx_2_1c0llllllllllllllllllllllllllllllllllllllllllllllhcg0ny"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: SUT =
            "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s"
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
    type SUT = AccessControllerAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s";
        let address = new_access_controller_address(b32.to_owned()).unwrap();
        assert_eq!(
            access_controller_address_network_id(&address),
            NetworkID::Mainnet
        );
        assert_eq!(access_controller_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_access_controller_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            new_access_controller_address_placeholder(),
            SUT::placeholder()
        );

        assert_eq!(
            new_access_controller_address_placeholder_other(),
            SUT::placeholder_other()
        );
    }
}
