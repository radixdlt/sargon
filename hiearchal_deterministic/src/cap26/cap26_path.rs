use serde::{Deserialize, Serialize};

use crate::bip32::hd_path::HDPath;

use super::account_path::AccountPath;

/// `m/44'/1022'/365'
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetIDPath(HDPath);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "discriminator", content = "value")]
pub enum CAP26Path {
    GetID(GetIDPath),
    AccountPath(AccountPath),
}
