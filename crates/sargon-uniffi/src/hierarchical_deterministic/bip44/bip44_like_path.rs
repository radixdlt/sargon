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
    
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
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
) -> Result<BIP44LikePath> {
    InternalBIP44LikePath::from_str(&string).map_result()
}

#[uniffi::export]
pub fn bip44_like_path_to_string(path: &BIP44LikePath) -> String {
    path.into_internal().to_string()
}

#[uniffi::export]
pub fn bip44_like_path_get_address_index(path: &BIP44LikePath) -> HDPathValue {
    path.into_internal().last_component().index()
}

#[uniffi::export]
pub fn new_bip44_like_path_sample() -> BIP44LikePath {
    InternalBIP44LikePath::sample().into()
}

#[uniffi::export]
pub fn new_bip44_like_path_sample_other() -> BIP44LikePath {
    InternalBIP44LikePath::sample_other().into()
}

