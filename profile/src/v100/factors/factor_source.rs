use serde::{Deserialize, Serialize};

use super::factor_sources::device_factor_source::DeviceFactorSource;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FactorSource {
    DeviceFactorSource(DeviceFactorSource),
}
