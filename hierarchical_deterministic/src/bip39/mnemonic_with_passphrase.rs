use serde::{Deserialize, Serialize};

use super::mnemonic::Mnemonic;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MnemonicWithPassphrase {
    pub mnemonic: Mnemonic,
}
