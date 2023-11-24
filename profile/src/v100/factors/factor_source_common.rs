use std::{
    cell::{Cell, RefCell},
    collections::BTreeSet,
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{
    factor_source_crypto_parameters::FactorSourceCryptoParameters,
    factor_source_flag::FactorSourceFlag,
};

/// Common properties shared between FactorSources of different kinds, describing
/// its state, when added, and supported cryptographic parameters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FactorSourceCommon {
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
