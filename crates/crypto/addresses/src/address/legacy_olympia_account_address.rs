use crate::prelude::*;
use radix_engine_toolkit::functions::derive::{
    olympia_account_address_from_public_key as RET_olympia_account_address_from_public_key,
    public_key_from_olympia_account_address as RET_public_key_from_olympia_account_address,
};

use core_utils::prelude::format_string;
use radix_engine_toolkit::types::OlympiaNetwork as ScryptoOlympiaNetwork;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[display("{}", self.bech32_address())]
#[debug("{}", self.bech32_address())]
pub struct LegacyOlympiaAccountAddress {
    pub value: Secp256k1PublicKey,
}

impl FromStr for LegacyOlympiaAccountAddress {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RET_public_key_from_olympia_account_address(s)
            .map_err(|_| CommonError::InvalidOlympiaAddressString {
                bad_value: s.to_owned(),
            })
            .and_then(|k| {
                if s.starts_with(ScryptoOlympiaNetwork::Mainnet.hrp()) {
                    Ok(k)
                } else {
                    Err(CommonError::InvalidAddressNotOlympiaMainnet {
                        bad_value: s.to_owned(),
                    })
                }
            })
            .and_then(Secp256k1PublicKey::try_from)
            .map(Self::from)
    }
}

impl LegacyOlympiaAccountAddress {
    pub fn bech32_address(&self) -> String {
        RET_olympia_account_address_from_public_key(
            &self.value.into(),
            ScryptoOlympiaNetwork::Mainnet,
        )
    }
}

impl LegacyOlympiaAccountAddress {
    pub fn to_babylon_account_address(self) -> AccountAddress {
        AccountAddress::new_from_public_key(self.value, NetworkID::Mainnet)
    }
}

impl From<Secp256k1PublicKey> for LegacyOlympiaAccountAddress {
    fn from(value: Secp256k1PublicKey) -> Self {
        Self { value }
    }
}

impl LegacyOlympiaAccountAddress {
    pub fn formatted(&self, format: AddressFormat) -> String {
        match format {
            AddressFormat::Default => format_string(self.to_string(), 3, 9),
            AddressFormat::Full | AddressFormat::Raw => self.to_string(),
        }
    }
}

impl From<LegacyOlympiaAccountAddress> for AccountAddress {
    fn from(value: LegacyOlympiaAccountAddress) -> Self {
        value.to_babylon_account_address()
    }
}

impl AccountAddress {
    pub fn was_migrated_from_legacy_olympia_account_address(
        &self,
        address: &LegacyOlympiaAccountAddress,
    ) -> bool {
        let olympia_hash = PublicKeyHash::hash(address.value);
        self.node_id()
            .as_bytes()
            .ends_with(olympia_hash.as_secp256k1().unwrap().as_ref())
    }
}

impl HasSampleValues for LegacyOlympiaAccountAddress {
    fn sample() -> Self {
        LegacyOlympiaAccountAddress::from_str(
            "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842",
        )
        .unwrap()
    }

    fn sample_other() -> Self {
        LegacyOlympiaAccountAddress::from_str(
            "rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge",
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LegacyOlympiaAccountAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_invalid_str() {
        let s = "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf"; // a Babylon address is NOT a valid Olympia addr
        assert_eq!(
            SUT::from_str(s),
            Err(CommonError::InvalidOlympiaAddressString {
                bad_value: s.to_string()
            })
        );
    }

    #[test]
    fn from_valid_str_but_wrong_network() {
        // same public key as `rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge`, but on OlympiaNetwork::Stokenet.
        let s =
            "tdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xq6x3dc6";
        assert_eq!(
            SUT::from_str(s),
            Err(CommonError::InvalidAddressNotOlympiaMainnet {
                bad_value: s.to_string()
            })
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", SUT::sample_other()),
            "rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge"
        )
    }

    #[test]
    fn formatted_default_short() {
        assert_eq!(
            SUT::sample_other().formatted(AddressFormat::Default),
            "rdx...0xqm2ylge"
        );
    }

    #[test]
    fn formatted_full() {
        assert_eq!(
            SUT::sample_other().formatted(AddressFormat::Full),
            "rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge"
        );
    }

    #[test]
    fn formatted_raw() {
        assert_eq!(
            SUT::sample_other().formatted(AddressFormat::Raw),
            "rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::sample_other()),
            "rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge"
        )
    }

    #[test]
    fn to_babylon() {
        assert_eq!(SUT::sample().to_babylon_account_address(), "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf".parse::<AccountAddress>().unwrap())
    }

    #[test]
    fn to_babylon_other() {
        // https://console.radixdlt.com/convert-olympia-address
        assert_eq!(SUT::sample_other().to_babylon_account_address(), "account_rdx168fghy4kapzfnwpmq7t7753425lwklk65r82ys7pz2xzleehk2ap0k".parse::<AccountAddress>().unwrap())
    }

    #[test]
    fn was_migrated_from_legacy_olympia_account_address() {
        let babylon: AccountAddress = "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf".parse().unwrap();

        assert!(babylon
            .was_migrated_from_legacy_olympia_account_address(&SUT::sample()));

        assert!(!AccountAddress::sample()
            .was_migrated_from_legacy_olympia_account_address(&SUT::sample()));
        assert!(!AccountAddress::sample_other()
            .was_migrated_from_legacy_olympia_account_address(&SUT::sample()));

        assert!(!AccountAddress::sample()
            .was_migrated_from_legacy_olympia_account_address(
                &SUT::sample_other()
            ));
        assert!(!AccountAddress::sample_other()
            .was_migrated_from_legacy_olympia_account_address(
                &SUT::sample_other()
            ));
    }

    #[test]
    fn was_migrated_from_legacy_olympia_account_address_other() {
        let babylon: AccountAddress = "account_rdx168fghy4kapzfnwpmq7t7753425lwklk65r82ys7pz2xzleehk2ap0k".parse().unwrap();

        assert!(babylon.was_migrated_from_legacy_olympia_account_address(
            &SUT::sample_other()
        ));
        assert!(!babylon
            .was_migrated_from_legacy_olympia_account_address(&SUT::sample()));
    }

    #[test]
    fn from_public_key() {
        let public_key: Secp256k1PublicKey = "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c".parse().unwrap();
        let sut = SUT::from(public_key);
        assert_eq!(
            sut,
            // https://github.com/radixdlt/typescript-radix-engine-toolkit/blob/6b4d041fbffb0a42adb39b215b2f4c7381fdc77b/resources/fixtures/derive.json#L89C18-L89C84
            SUT::sample()
        );
    }

    #[test]
    fn from_public_key_other() {
        let public_key: Secp256k1PublicKey = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".parse().unwrap();
        let sut = SUT::from(public_key);
        assert_eq!(
            sut,
            // https://github.com/radixdlt/typescript-radix-engine-toolkit/blob/6b4d041fbffb0a42adb39b215b2f4c7381fdc77b/resources/fixtures/derive.json#L98C18-L98C83
            SUT::sample_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let a = &SUT::sample();
        assert_json_value_eq_after_roundtrip(
            a,
            json!("rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842"),
        );
        assert_json_roundtrip(a);
        assert_json_value_ne_after_roundtrip(
            a,
            json!("rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(
            json!("account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf")
        );

        assert_json_value_fails::<SUT>(json!("super invalid"));
    }
}
