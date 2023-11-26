use serde::{Deserialize, Serialize};

use crate::{
    bip44::bip44_like_path::BIP44LikePath,
    cap26::cap26_path::{cap26_path::CAP26Path, paths::account_path::AccountPath},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "discriminator", content = "value")]
pub enum DerivationPath {
    CAP26(CAP26Path),
    BIP44Like(BIP44LikePath),
}

impl DerivationPath {
    pub fn placeholder() -> Self {
        Self::CAP26(CAP26Path::AccountPath(AccountPath::placeholder()))
    }
}
