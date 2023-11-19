use serde::{de, Deserializer, Serialize, Serializer};
use wallet_kit_common::network_id::NetworkID;

use crate::bip32::{hd_path::HDPath, hd_path_component::HDPathValue};

use super::{cap26::CAP26Repr, cap26_entity_kind::CAP26EntityKind, cap26_key_kind::CAP26KeyKind};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountPath {
    pub path: HDPath,
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    pub index: HDPathValue,
}
impl CAP26Repr for AccountPath {
    fn hd_path(&self) -> &HDPath {
        &self.path
    }

    fn entity_kind() -> Option<CAP26EntityKind> {
        Some(CAP26EntityKind::Account)
    }

    fn __with_path_and_components(
        path: HDPath,
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: HDPathValue,
    ) -> Self {
        Self {
            path,
            network_id,
            entity_kind,
            key_kind,
            index,
        }
    }
}

impl AccountPath {
    pub fn placeholder() -> Self {
        Self::from_str("m/44H/1022H/1H/525H/1460H/0H").unwrap()
    }
}

impl Serialize for AccountPath {
    /// Serializes this `AccountAddress` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for AccountPath {
    /// Tries to deserializes a JSON string as a bech32 address into an `AccountAddress`.
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<AccountPath, D::Error> {
        let s = String::deserialize(d)?;
        AccountPath::from_str(&s).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::network_id::NetworkID;

    use crate::cap26::{
        cap26::CAP26Repr, cap26_entity_kind::CAP26EntityKind, cap26_key_kind::CAP26KeyKind,
    };

    use super::AccountPath;

    #[test]
    fn string_roundtrip() {
        let str = "m/44H/1022H/1H/525H/1460H/0H";
        let parsed = AccountPath::from_str(str).unwrap();
        assert_eq!(parsed.network_id, NetworkID::Mainnet);
        assert_eq!(parsed.entity_kind, CAP26EntityKind::Account);
        assert_eq!(parsed.key_kind, CAP26KeyKind::TransactionSigning);
        assert_eq!(parsed.index, 0);
        assert_eq!(parsed.to_string(), str);
        let built = AccountPath::new(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, 0);
        assert_eq!(built, parsed)
    }
}
