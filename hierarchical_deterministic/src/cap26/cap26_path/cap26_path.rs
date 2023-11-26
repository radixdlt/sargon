use serde::{Deserialize, Serialize};

use super::paths::{account_path::AccountPath, getid_path::GetIDPath};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "discriminator", content = "value")]
pub enum CAP26Path {
    GetID(GetIDPath),
    AccountPath(AccountPath),
}
