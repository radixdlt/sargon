use crate::prelude::*;

pub fn parse_url(s: impl AsRef<str>) -> Result<Url, CommonError> {
    Url::try_from(s.as_ref()).map_err(|_| CommonError::InvalidURL {
        bad_value: s.as_ref().to_owned(),
    })
}
