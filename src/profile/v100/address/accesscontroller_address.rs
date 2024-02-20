use crate::prelude::*;

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
#[display("{__inner}")]
pub struct AccessControllerAddress {
    pub(crate) __inner: InnerAccessControllerAddress,
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
    /*
        these should all be valid
        [
        "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a",
        "accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak",
        "accesscontroller_rdx1cva6mtja4crwxxhmd63q2xlhew7fh0af67zw3snhzj8cm7xq2cm06g",
        "accesscontroller_rdx1cvlu8kvmqu56arywyzkkewyuv7mdg448d69k083dpq0nvrd44me6qd",
        "accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt",
        "accesscontroller_rdx1cwggxzkqxwg9zjhv3jvyvkcn2hl7gxgwgjeqxl2d7xyuyx3tklg77y",
        "accesscontroller_rdx1cwtvlhhg0pwyrlcujv9gv2adastmcc03ewg9vww8ke3s5t9gjf7jmp",
        "accesscontroller_rdx1cwufa4a2j7klu5hh72uwxvd9gyevxfuxspxynne7fqnnzzr7nh4uya"
      ]
      and these:
      accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s
      accesscontroller_rdx1cvqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq09959m

      from GW:
      accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt
      accesscontroller_tdx_2_1cw68j9ca4fye09mz3hshp4qydjnxhsahm68hvmz9cjhftcz9f53juq

      Working roundtrip test from OA:
      #[test]
    fn access_controller() {
        let address = CanonicalAccessControllerAddress::from_str("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a").unwrap();
        assert_eq!(CanonicalAccessControllerAddress::from_str(&address.to_string()).unwrap().to_string(), "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a");
    }
        */
    pub fn placeholder_mainnet() -> Self {
        "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a".parse().expect("Placeholder")
    }

    pub fn placeholder_stokenet() -> Self {
        "accesscontroller_tdx_2_1c0llllllllllllllllllllllllllllllllllllllllllllllhcg0ny".parse().expect("Placeholder")
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_toolkit::models::canonical_address_types::CanonicalAccessControllerAddress;

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessControllerAddress;

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
        let s = "accesscontroller_rdx1cw9383xuqx6cme0knucw5aggknvrqmc8lzu7jcn3kwherk8x55zmtt";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
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
            json!("resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s")
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

    #[test]
    fn access_controller() {
        let address = CanonicalAccessControllerAddress::from_str("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a").unwrap();
        println!("{address}");
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
        let address = new_accesscontroller_address(b32.to_owned()).unwrap();
        assert_eq!(
            accesscontroller_address_network_id(&address),
            NetworkID::Mainnet
        );
        assert_eq!(accesscontroller_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "accesscontroller_rdx1c0llllllllllllllllllllllllllllllllllllllllllllllkl2v3s";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_accesscontroller_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }
}
