use uniffi::deps::bytes::Buf;

use crate::prelude::*;
use crate::UniFfiTag;

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
    pub fn new(url: Url) -> Result<Self> {
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
        let url = Url::parse("https://radixdlt.com").unwrap();
        let result = SUT::new(url.clone());
        assert_eq!(result.unwrap().url, url);
    }
}
