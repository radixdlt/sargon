use crate::*;

use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScryptoSborValue {
    pub programmatic_json: ProgrammaticScryptoSborValue,
}
