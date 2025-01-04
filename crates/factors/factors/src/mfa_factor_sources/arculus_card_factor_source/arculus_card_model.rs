use core_utils::prelude::{DeserializeStr, SerializeToString};

use crate::prelude::*;

/// The model of a Arculus Card.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[serde(rename_all = "camelCase")]
pub enum ArculusCardModel {
    /// Arculus card model: "Arculus® Cold Storage Wallet",
    /// for more info [see][link].
    ///
    /// [link]: https://www.getarculus.com/products/arculus-cold-storage-wallet.html
    ArculusColdStorageWallet,
}

impl Default for ArculusCardModel {
    fn default() -> Self {
        Self::ArculusColdStorageWallet
    }
}

impl FromStr for ArculusCardModel {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        s.deserialize()
            .map_err(|_| CommonError::InvalidArculusCardModel {
                bad_value: s.to_owned(),
            })
    }
}

impl std::fmt::Display for ArculusCardModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize_to_string())
    }
}

impl HasSampleValues for ArculusCardModel {
    fn sample() -> Self {
        ArculusCardModel::ArculusColdStorageWallet
    }

    fn sample_other() -> Self {
        ArculusCardModel::ArculusColdStorageWallet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ArculusCardModel;

    #[test]
    fn string_roundtrip() {
        use ArculusCardModel::*;
        let eq = |f: SUT, s| {
            assert_eq!(f.to_string(), s);
            assert_eq!(SUT::from_str(s).unwrap(), f);
        };

        eq(ArculusColdStorageWallet, "arculusColdStorageWallet");
    }

    #[test]
    fn from_str_err() {
        let s = "invalid!";
        assert_eq!(
            SUT::from_str(s),
            Err(CommonError::InvalidArculusCardModel {
                bad_value: s.to_owned(),
            })
        );
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(
            &SUT::ArculusColdStorageWallet,
            json!("arculusColdStorageWallet"),
        );
    }
}
