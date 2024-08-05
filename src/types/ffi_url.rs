use uniffi::{Lift, RustBuffer};

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

unsafe impl Lift<UniFfiTag> for FfiUrl {
    type FfiType = RustBuffer;

    fn try_lift(buf: RustBuffer) -> uniffi::Result<Self> {
        Self::try_lift_from_rust_buffer(buf)
    }

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
        let url = Url::try_read(buf)?;
        Ok(Self { url })
    }
}
