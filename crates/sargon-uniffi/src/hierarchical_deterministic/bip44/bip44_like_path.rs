use crate::prelude::*;
use sargon::BIP44LikePath as InternalBIP44LikePath;

/// Either a canonical BIP44 derivation path like so:
///
/// `m / purpose' / coin_type' / account' / change / address_index`
///
/// Or an Radix Olympia BIP44 "like" path, where the `address_index` accidentally
/// was made hardened, i.e.:
///
/// `m / purpose' / coin_type' / account' / change / address_index'`
///
/// This was a mistake made during implementation of Radix Olympia.
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
///
/// fn parse(s: &str) -> Result<BIP44LikePath> {
///    s.parse::<BIP44LikePath>()
/// }
///
/// assert!(parse("m/44'/1022'/0'/0/0").is_ok()); // Canonical BIP44
/// assert!(parse("m/44'/1022'/0'/0/0'").is_ok()); // BIP44 like
///
/// assert_eq!(parse("m/44'/1022'/0'/0'/0"), Err(CommonError::InvalidBIP44LikePathChangeWasUnexpectedlyHardened));
/// assert_eq!(parse("m/44'/1022'/0'/0'/0'"), Err(CommonError::InvalidBIP44LikePathChangeWasUnexpectedlyHardened));
/// assert_eq!(parse("m/44'/0'/0'/0/0'"), Err(CommonError::CoinTypeNotFound { bad_value: 0 }));
/// ```
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct BIP44LikePath {
    pub path: HDPath,
}

impl From<InternalBIP44LikePath> for BIP44LikePath {
    fn from(value: InternalBIP44LikePath) -> Self {
        Self {
            path: value.path.into(),
        }
    }
}

impl Into<InternalBIP44LikePath> for BIP44LikePath {
    fn into(self) -> InternalBIP44LikePath {
        InternalBIP44LikePath {
            path: self.path.into(),
        }
    }
}

#[uniffi::export]
pub fn new_bip44_like_path_from_index(index: HDPathValue) -> BIP44LikePath {
    InternalBIP44LikePath::new(index.into()).into()
}

#[uniffi::export]
pub fn new_bip44_like_path_from_string(
    string: String,
) -> Result<BIP44LikePath, CommonError> {
    map_result_from_internal(InternalBIP44LikePath::from_str(&string))
}

#[uniffi::export]
pub fn bip44_like_path_to_string(path: &BIP44LikePath) -> String {
    path.into::<InternalBIP44LikePath>().to_string()
}

#[uniffi::export]
pub fn bip44_like_path_get_address_index(path: &BIP44LikePath) -> HDPathValue {
    path.into::<InternalBIP44LikePath>().last_component().index()
}

#[uniffi::export]
pub fn new_bip44_like_path_sample() -> BIP44LikePath {
    InternalBIP44LikePath::sample().into()
}

#[uniffi::export]
pub fn new_bip44_like_path_sample_other() -> BIP44LikePath {
    InternalBIP44LikePath::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BIP44LikePath;

    #[test]
    fn test_new_bip44_like_path_from_index() {
        assert_eq!(new_bip44_like_path_from_index(0), SUT::new(0))
    }

    #[test]
    fn test_new_bip44_like_path_from_string() {
        let path = new_bip44_like_path_sample();
        assert_eq!(
            path,
            new_bip44_like_path_from_string(path.to_string()).unwrap()
        );

        let path_other = new_bip44_like_path_sample_other();
        assert_eq!(
            path_other,
            new_bip44_like_path_from_string(path_other.to_string()).unwrap()
        );
    }

    #[test]
    fn test_bip44_like_path_to_string() {
        let path = new_bip44_like_path_sample();
        assert_eq!(
            bip44_like_path_to_string(&path),
            new_bip44_like_path_from_string(path.to_string())
                .unwrap()
                .to_string()
        );

        let path_other = new_bip44_like_path_sample_other();
        assert_eq!(
            bip44_like_path_to_string(&path_other),
            new_bip44_like_path_from_string(path_other.to_string())
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn test_bip44_like_path_get_address_index() {
        assert_eq!(bip44_like_path_get_address_index(&SUT::sample_other()), 1)
    }
}
