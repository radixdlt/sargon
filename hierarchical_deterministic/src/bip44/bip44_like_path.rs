use serde::{Deserialize, Serialize};

use crate::bip32::hd_path::HDPath;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BIP44LikePath(HDPath);
