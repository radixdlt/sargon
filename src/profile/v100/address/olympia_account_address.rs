use crate::prelude::*;
use radix_engine_toolkit::functions::derive::{
    olympia_account_address_from_public_key as RET_olympia_account_address_from_public_key,
    public_key_from_olympia_account_address as RET_public_key_from_olympia_account_address,
};

use radix_engine_common::crypto::Secp256k1PublicKey as ScryptoSecp256k1PublicKey;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{olympia_address}")]
pub struct OlympiaAccountAddress {
    public_key: Secp256k1PublicKey,
    olympia_address: String,
}

impl OlympiaAccountAddress {
    fn with_pub_key_and_bech32_olympia_addr(
        public_key: Secp256k1PublicKey,
        olympia_address: String,
    ) -> Self {
        Self {
            public_key,
            olympia_address,
        }
    }

    #[allow(unused)]
    pub fn new(public_key: &Secp256k1PublicKey) -> Self {
        let olympia_address = RET_olympia_account_address_from_public_key(
            &public_key.clone().into(),
            radix_engine_toolkit::functions::derive::OlympiaNetwork::Mainnet,
        );

        Self::with_pub_key_and_bech32_olympia_addr(
            public_key.clone(),
            olympia_address,
        )
    }

    pub fn from_bech32_olympia_str(
        bech32_string: impl AsRef<str>,
    ) -> Result<Self> {
        let olympia_string = bech32_string.as_ref().to_owned();
        RET_public_key_from_olympia_account_address(bech32_string)
            .map_err(|e| {
                error!("Invalid Olympia addr, error: {:?}", e);
                CommonError::InvalidOlympiaAddressString {
                    bad_value: olympia_string.clone(),
                }
            })
            .and_then(|k| k.try_into())
            .map(|k| {
                Self::with_pub_key_and_bech32_olympia_addr(k, olympia_string)
            })
    }
}

impl FromStr for OlympiaAccountAddress {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bech32_olympia_str(s)
    }
}

impl OlympiaAccountAddress {
    #[allow(unused)]
    pub fn to_babylon_account_address(&self) -> AccountAddress {
        AccountAddress::new(self.public_key.clone().into(), NetworkID::Mainnet)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn public_key_to_olympia_to_babylon() {
        let olympia: OlympiaAccountAddress =
            "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842"
                .parse()
                .unwrap();
        let pubkey: Secp256k1PublicKey = "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c".parse().unwrap();
        let babylon: AccountAddress = "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf".parse().unwrap();
        assert_eq!(
            olympia.olympia_address,
            "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842"
        );
        assert_eq!(olympia.public_key, pubkey.clone());
        assert_eq!(olympia.to_babylon_account_address(), babylon.clone());
        assert_eq!(olympia, OlympiaAccountAddress::new(&pubkey));
    }
}
