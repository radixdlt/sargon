use crate::prelude::*;

/// A string matching `[_0-9a-zA-Z]{1,64}`.
///
/// This is an internal wrapping of Scrypto's `StringNonFungibleLocalId`
/// with a UniFFI custom converter using `String` as `Builtin`.
///
/// Using this type instead of `String` directly in `NonFungibleLocalId::Str`,
/// allows us to do impl `From<NonFungibleLocalId> for NonFungibleLocalId` instead
/// of `TryFrom<NonFungibleLocalId>`.
#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[debug("{}", self.to_string())]
#[display("{}", self.secret_magic.value().to_owned())]
pub struct NonFungibleLocalIdString {
    secret_magic: ScryptoStringNonFungibleLocalId,
}

#[uniffi::export]
pub fn new_non_fungible_local_id_string_from_str(
    string: String,
) -> Result<NonFungibleLocalIdString> {
    string.parse()
}

impl FromStr for NonFungibleLocalIdString {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        scrypto_string_non_fungible_local_id(s).map(|i| i.into())
    }
}

impl From<ScryptoStringNonFungibleLocalId> for NonFungibleLocalIdString {
    fn from(value: ScryptoStringNonFungibleLocalId) -> Self {
        Self {
            secret_magic: value,
        }
    }
}
impl From<NonFungibleLocalIdString> for ScryptoStringNonFungibleLocalId {
    fn from(value: NonFungibleLocalIdString) -> Self {
        value.secret_magic
    }
}

fn scrypto_string_non_fungible_local_id(
    string: impl AsRef<str>,
) -> Result<ScryptoStringNonFungibleLocalId> {
    ScryptoStringNonFungibleLocalId::new(string).map_err(|e| {
        error!("{:?}", e);
        CommonError::InvalidNonFungibleLocalIDString
    })
}

impl crate::UniffiCustomTypeConverter for ScryptoStringNonFungibleLocalId {
    type Builtin = String;

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        scrypto_string_non_fungible_local_id(val).map_err(|e| e.into())
    }

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.value().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleLocalIdString;

    #[test]
    fn from_str() {
        let s = "foo";
        let sut: SUT = s.parse().unwrap();
        assert_eq!(sut.to_string(), s.to_owned());
    }

    #[test]
    fn invalid_forbidden_chars() {
        let s = "<foo>";
        assert_eq!(
            s.parse::<SUT>(),
            Err(CommonError::InvalidNonFungibleLocalIDString)
        );
    }

    #[test]
    fn invalid_empty() {
        let s = "";
        assert_eq!(
            s.parse::<SUT>(),
            Err(CommonError::InvalidNonFungibleLocalIDString)
        );
    }

    #[test]
    fn invalid_too_long() {
        let s = "a".repeat(64 + 1);
        assert_eq!(
            s.parse::<SUT>(),
            Err(CommonError::InvalidNonFungibleLocalIDString)
        );
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleLocalIdString;

    #[test]
    fn from_str() {
        let s = "foo";
        let sut: SUT =
            new_non_fungible_local_id_string_from_str(s.to_owned()).unwrap();
        assert_eq!(sut.to_string(), s.to_owned());
    }

    #[test]
    fn invalid_forbidden_chars() {
        let s = "<foo>";
        assert_eq!(
            new_non_fungible_local_id_string(s.to_owned()),
            Err(CommonError::InvalidNonFungibleLocalIDString)
        );
    }

    #[test]
    fn invalid_empty() {
        let s = "";
        assert_eq!(
            new_non_fungible_local_id_string(s.to_owned()),
            Err(CommonError::InvalidNonFungibleLocalIDString)
        );
    }

    #[test]
    fn invalid_too_long() {
        let s = "a".repeat(64 + 1);
        assert_eq!(
            new_non_fungible_local_id_string(s.to_owned()),
            Err(CommonError::InvalidNonFungibleLocalIDString)
        );
    }
}
