use std::{
    cell::{Cell, RefCell},
    collections::BTreeSet,
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use wallet_kit_common::utils::factory::now;

use super::{
    factor_source_crypto_parameters::FactorSourceCryptoParameters,
    factor_source_flag::FactorSourceFlag,
};

/// Common properties shared between FactorSources of different kinds, describing
/// its state, when added, and supported cryptographic parameters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FactorSourceCommon {
    /// Cryptographic parameters a certain FactorSource supports, e.g. Elliptic Curves.
    pub crypto_parameters: FactorSourceCryptoParameters,

    /// When this factor source for originally added by the user.
    pub added_on: NaiveDateTime,

    /// Date of last usage of this factor source
    ///
    /// This is the only mutable property, it is mutable
    /// since we will update it every time this FactorSource
    /// is used.
    ///
    /// Has interior mutability (`Cell`) since every time this
    /// factor source is used we should update this date.
    pub last_used_on: Cell<NaiveDateTime>,

    /// Flags which describe a certain state a FactorSource might be in, e.g. `Main` (BDFS).
    ///
    /// Has interior mutability (`RefCell`) since a user might wanna flag a FactorSource as
    /// "deleted".
    pub flags: RefCell<BTreeSet<FactorSourceFlag>>,
}

impl FactorSourceCommon {
    pub fn with_values<I>(
        crypto_parameters: FactorSourceCryptoParameters,
        added_on: NaiveDateTime,
        last_used_on: NaiveDateTime,
        flags: I,
    ) -> Self
    where
        I: IntoIterator<Item = FactorSourceFlag>,
    {
        Self {
            crypto_parameters,
            added_on,
            last_used_on: Cell::new(last_used_on),
            flags: RefCell::new(BTreeSet::from_iter(flags.into_iter())),
        }
    }

    pub fn new<I>(crypto_parameters: FactorSourceCryptoParameters, flags: I) -> Self
    where
        I: IntoIterator<Item = FactorSourceFlag>,
    {
        let date: NaiveDateTime = now();
        Self::with_values(crypto_parameters, date, date, flags)
    }

    pub fn new_bdfs(is_main: bool) -> Self {
        Self::new(
            FactorSourceCryptoParameters::babylon(),
            if is_main {
                vec![FactorSourceFlag::Main]
            } else {
                Vec::new()
            }
            .into_iter(),
        )
    }
}

impl Default for FactorSourceCommon {
    fn default() -> Self {
        Self::new(FactorSourceCryptoParameters::default(), [])
    }
}

impl FactorSourceCommon {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        let date =
            NaiveDateTime::parse_from_str("2023-09-11T16:05:56", "%Y-%m-%dT%H:%M:%S").unwrap();
        FactorSourceCommon::with_values(
            FactorSourceCryptoParameters::default(),
            date.clone(),
            date,
            [FactorSourceFlag::Main],
        )
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDateTime;
    use wallet_kit_common::{json::assert_eq_after_json_roundtrip, utils::factory::now};

    use crate::v100::factors::{
        factor_source_crypto_parameters::FactorSourceCryptoParameters,
        factor_source_flag::FactorSourceFlag,
    };

    use super::FactorSourceCommon;

    #[test]
    fn default_support_babylon() {
        assert_eq!(
            FactorSourceCommon::default().crypto_parameters,
            FactorSourceCryptoParameters::babylon()
        )
    }

    #[test]
    fn new_uses_now_as_date() {
        let date0 = now();
        let model = FactorSourceCommon::new(FactorSourceCryptoParameters::default(), []);
        let mut date1 = now();
        for _ in 0..10 {
            // rust is too fast... lol.
            date1 = now();
        }
        let do_test = |d: NaiveDateTime| {
            assert!(d > date0);
            assert!(d < date1);
        };
        do_test(model.added_on);
        do_test(model.last_used_on.get());
    }

    #[test]
    fn json_roundtrip() {
        let date =
            NaiveDateTime::parse_from_str("2023-09-11T16:05:56", "%Y-%m-%dT%H:%M:%S").unwrap();
        let model = FactorSourceCommon::with_values(
            FactorSourceCryptoParameters::default(),
            date.clone(),
            date,
            [FactorSourceFlag::Main],
        );

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "addedOn": "2023-09-11T16:05:56",
                "cryptoParameters": {
                    "supportedCurves": ["curve25519"],
                    "supportedDerivationPathSchemes": ["cap26"]
                },
                "flags": ["main"],
                "lastUsedOn": "2023-09-11T16:05:56"
            }
            "#,
        );
    }

    #[test]
    fn main_flag_present_if_main() {
        assert!(FactorSourceCommon::new_bdfs(true)
            .flags
            .get_mut()
            .contains(&FactorSourceFlag::Main));
    }

    #[test]
    fn main_flag_not_present_if_not_main() {
        assert!(FactorSourceCommon::new_bdfs(false)
            .flags
            .get_mut()
            .is_empty());
    }
}
