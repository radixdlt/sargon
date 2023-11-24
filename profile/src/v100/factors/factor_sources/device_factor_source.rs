use serde::{Deserialize, Serialize};

use crate::v100::factors::{
    factor_source_common::FactorSourceCommon, factor_source_id_from_hash::FactorSourceIDFromHash,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFactorSource {
    id: FactorSourceIDFromHash,
    common: FactorSourceCommon,
}
