use serde::{Deserialize, Serialize};

use super::factor_source_crypto_parameters::FactorSourceCryptoParameters;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FactorSourceCommon {
    crypto_parameters: FactorSourceCryptoParameters,
}
