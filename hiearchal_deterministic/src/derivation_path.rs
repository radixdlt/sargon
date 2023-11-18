use std::str::FromStr;

use serde::{Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use slip10::path::BIP32Path;
use strum::FromRepr;

use crate::cap26::HDPathValue;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DerivationPath(Vec<HDPathComponent>);

const BIP32_HARDENED: u32 = 2147483648;
// fn is_hardened(component: &HDPathValue) -> bool {
//     component >= &BIP32_HARDENED
// }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HDPathComponent(HDPathValue);

impl HDPathComponent {
    fn value(&self) -> HDPathValue {
        if self.is_hardened() {
            self.0 - BIP32_HARDENED
        } else {
            self.0
        }
    }
    fn is_hardened(&self) -> bool {
        self.0 >= BIP32_HARDENED
    }
    fn from_value(value: HDPathValue) -> Self {
        Self(value)
    }
    fn harden(value: HDPathValue) -> Self {
        assert!(value < BIP32_HARDENED);
        Self(value + BIP32_HARDENED)
    }
}
fn components_of(path: BIP32Path) -> Vec<HDPathComponent> {
    let mut bip32 = path.clone();
    let mut vec: Vec<HDPathComponent> = Vec::new();
    for _ in 0..bip32.depth() {
        match bip32.pop() {
            Some(c) => vec.push(HDPathComponent::from_value(c)),
            None => break,
        }
    }
    assert!(vec.len() == path.depth() as usize);
    vec.reverse();
    return vec;
}

use thiserror::Error;

#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[repr(u32)]
pub enum CAP26EntityKind {
    /// An account entity type
    Account = 525,

    /// Used by Persona
    Identity = 618,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[repr(u32)]
pub enum CAP26KeyKind {
    /// For a key to be used for signing transactions.
    /// The value is the ascii sum of `"TRANSACTION_SIGNING"`
    TransactionSigning = 1460,

    /// For a key to be used for signing authentication..
    /// The value is the ascii sum of `"AUTHENTICATION_SIGNING"`
    AuthenticationSigning = 1678,

    /// For a key to be used for encrypting messages.
    /// The value is the ascii sum of `"MESSAGE_ENCRYPTION"`
    MessageEncryption = 1391,
}

#[derive(Debug, Error, PartialEq)]
pub enum CAP26Error {
    #[error("Invalid BIP32 path '{0}'.")]
    InvalidBIP32Path(String),

    #[error("Invalid depth of CAP26 Path.")]
    InvalidDepthOfCAP26Path,

    #[error("Found non hardened components in path, invalid!")]
    NotAllComponentsAreHardened,

    #[error("Did not find 44H at expected index 1, found value: '{0}'")]
    BIP44PurposeNotFoundAtIndex1(HDPathValue),

    #[error("Did not find cointype 1022H at expected index 2, found value: '{0}'")]
    CoinTypeNotFoundAtIndex2(HDPathValue),

    #[error("Network ID exceeds limit of 255, will never be valid, at index 3, found value: '{0}', known network IDs: [1 (mainnet), 2 (stokenet)]")]
    InvalidNetworkIDExceedsLimit(HDPathValue),

    #[error("InvalidEntityKind, got: '{0}', expected any of: [525H, 618H].")]
    InvalidEntityKind(HDPathValue),

    #[error("InvalidKeyKind, got: '{0}', expected any of: [1460H, 1678H, 1391H].")]
    InvalidKeyKind(HDPathValue),
}

impl HDPathComponent {
    fn bip44_purpose() -> Self {
        Self::harden(44)
    }

    /// The `cointype` of Radix DLT: `1022H`, as defined in SLIP44, see
    /// merged PR: https://github.com/satoshilabs/slips/pull/1137
    fn radix_cointype() -> Self {
        Self::harden(1022)
    }
}

impl DerivationPath {
    pub fn from_str(s: &str) -> Result<Self, CAP26Error> {
        let _path =
            BIP32Path::from_str(s).map_err(|_| CAP26Error::InvalidBIP32Path(s.to_string()))?;
        if _path.depth() != 6 {
            return Err(CAP26Error::InvalidDepthOfCAP26Path);
        }
        let path = components_of(_path);
        if !path.iter().all(|c| c.is_hardened()) {
            return Err(CAP26Error::NotAllComponentsAreHardened);
        }
        if path[0] != HDPathComponent::bip44_purpose() {
            return Err(CAP26Error::BIP44PurposeNotFoundAtIndex1(path[0].value()));
        }
        if path[1] != HDPathComponent::radix_cointype() {
            return Err(CAP26Error::CoinTypeNotFoundAtIndex2(path[1].value()));
        }
        if path[2].value() >= (u8::MAX as u32) {
            return Err(CAP26Error::InvalidNetworkIDExceedsLimit(path[2].value()));
        }
        let entity_kind_value = path[3].value();
        let Some(entity_kind) = CAP26EntityKind::from_repr(entity_kind_value) else {
            return Err(CAP26Error::InvalidEntityKind(path[3].value()));
        };

        let key_kind_value = path[4].value();
        let Some(key_kind) = CAP26KeyKind::from_repr(key_kind_value) else {
            return Err(CAP26Error::InvalidKeyKind(path[4].value()));
        };

        return Ok(Self(path));
    }
}

impl Serialize for DerivationPath {
    /// Serializes this `AccountAddress` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        // serializer.serialize_str(&self.address)
        todo!()
    }
}

impl<'de> serde::Deserialize<'de> for DerivationPath {
    /// Tries to deserializes a JSON string as a bech32 address into an `AccountAddress`.
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<DerivationPath, D::Error> {
        let s = String::deserialize(d)?;
        // AccountAddress::try_from_bech32(&s).map_err(de::Error::custom)
        todo!()
    }
}

impl DerivationPath {
    pub fn placeholder() -> Self {
        Self::from_str("m/44H/1022H/10H/618H/1460H/0H").unwrap()
    }
}
