use crate::prelude::*;

#[derive(uniffi::Object)]
pub struct UniFFIUrl {
    pub url: Url,
}

#[uniffi::export]
impl UniFFIUrl {
    #[uniffi::constructor]
    pub fn parse(url_path: String) -> Result<Self> {
        let url = parse_url(url_path)?;
        Ok(Self { url })
    }
}
