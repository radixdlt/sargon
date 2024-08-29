use crate::prelude::*;

/// A wrapper around `Url` that allows us to safely deal with Urls generated on hosts.
///
/// Context: We have defined a custom type conversion between Rust's `Url` and the hosts analogues.
/// However, a Url could be considered valid on host but not on Rust. For example, Swift allows to build a Url
/// from string `"invalid input"`, while Rust doesn't.
///
/// Therefore, if a given Rust function expects a `Url` as param and is sent one from host side which is invalid,
/// the code will panic. However, if we send the wrapper instead, we make sure the conversion is safely done on the
/// host side, dealing with the failing conversion properly rather than panicking.
#[derive(Debug, PartialEq, Eq, Hash, uniffi::Object, derive_more::Display)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct FfiUrl {
    pub url: Url,
}

#[uniffi::export]
impl FfiUrl {
    #[uniffi::constructor]
    pub fn new(url_path: String) -> Result<Self> {
        Self::from_str(&url_path)
    }
}

#[uniffi::export]
pub fn ffi_url_get_url(ffi_url: &FfiUrl) -> Url {
    ffi_url.url.clone()
}

impl FromStr for FfiUrl {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = parse_url(s)?;
        Ok(Self { url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FfiUrl;

    #[test]
    fn test_new() {
        let url_path = "https://radixdlt.com";
        let result = SUT::new(url_path.to_string());
        assert_eq!(result.unwrap().url, Url::parse(url_path).unwrap());
    }

    #[test]
    fn test_from_str() {
        let result = SUT::from_str("https://radixdlt.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_url() {
        let url_path = "https://radixdlt.com";
        let sut = SUT::new(url_path.to_string()).unwrap();
        assert_eq!(ffi_url_get_url(&sut), Url::parse(url_path).unwrap());
    }
}
