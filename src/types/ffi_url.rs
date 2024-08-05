use uniffi::Lift;

use crate::prelude::*;
use crate::UniFfiTag;

#[derive(Debug, PartialEq, Eq, Hash, uniffi::Object, derive_more::Display)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct FfiUrl {
    pub url: Url,
}

#[uniffi::export]
impl FfiUrl {
    #[uniffi::constructor]
    pub fn parse(url_path: String) -> Result<Self> {
        let url = parse_url(url_path)?;
        Ok(Self { url })
    }
}
