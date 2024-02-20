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
pub struct VaultAddress {
    pub(crate) __inner: InnerVaultAddress,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn display() {
        let s = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd";
        let a = VaultAddress::try_from_bech32(s).unwrap();
        assert_eq!(format!("{a}"), s);
    }

    #[test]
    fn json_roundtrip() {
        let a: VaultAddress =
            "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                .parse()
                .unwrap();

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
        assert_json_value_fails::<VaultAddress>(
            json!("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxxx")
        );
        assert_json_value_fails::<VaultAddress>(
            json!("account_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
        );
        assert_json_value_fails::<VaultAddress>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: VaultAddress =
            "resource_tdx_2_1tkckx9fynl9f7756z8wxphq7wce6vk874nuq4f2nnxgh3nzrwhjdlp"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: VaultAddress =
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
    type SUT = VaultAddress;

    #[test]
    fn new() {
        let s = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd";
        let a = VaultAddress::try_from_bech32(s).unwrap();
        let b = new_vault_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }
}
