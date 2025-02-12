use core_utils::prelude::format_string;

use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    EnumAsInner,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub enum NonFungibleLocalId {
    /// Unsigned integers, up to u64.
    ///
    /// Create using `NonFungibleLocalId::integer(...)`.
    Integer { value: u64 },

    /// String matching `[_0-9a-zA-Z]{1,64}`.
    ///
    /// Create using `NonFungibleLocalId::string(...).unwrap()`.
    Str { value: NonFungibleLocalIdString },

    /// Bytes, of length between 1 and 64.
    ///
    /// Create using `NonFungibleLocalId::bytes(...).unwrap()`.
    Bytes { value: NonEmptyMax64Bytes },

    /// RUID, v4, variant 1, big endian. See https://www.rfc-editor.org/rfc/rfc4122
    ///
    /// Create using `NonFungibleLocalId::ruid(...).unwrap()`.
    Ruid { value: Exactly32Bytes },
}

impl NonFungibleLocalId {
    pub fn integer(i: u64) -> Self {
        Self::Integer { value: i }
    }

    pub fn string(id: impl AsRef<str>) -> Result<Self> {
        id.as_ref()
            .parse::<NonFungibleLocalIdString>()
            .map(|value| Self::Str { value })
    }

    pub fn ruid(bytes: impl AsRef<[u8]>) -> Result<Self> {
        Exactly32Bytes::try_from(bytes.as_ref())
            .map(|value| Self::Ruid { value })
    }

    pub fn bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        NonEmptyMax64Bytes::try_from(bytes.as_ref())
            .map(|value| Self::Bytes { value })
    }
}

impl NonFungibleLocalId {
    pub fn random() -> Self {
        Self::Bytes {
            value: NonEmptyMax64Bytes::generate(),
        }
    }
}

impl NonFungibleLocalId {
    pub fn derives_account_address(
        &self,
        account_address: AccountAddress,
    ) -> bool {
        self.as_bytes()
            .map(|local_id_bytes| {
                let bytes = local_id_bytes.bag_of_bytes.bytes();
                if bytes.len() == ScryptoNodeId::LENGTH {
                    bytes[..ScryptoNodeId::LENGTH]
                        == account_address.node_id().0
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }
}

impl NonFungibleLocalId {
    pub fn formatted(&self, format: AddressFormat) -> String {
        match format {
            AddressFormat::Default => match self {
                NonFungibleLocalId::Ruid { value: _ } => {
                    format_string(self.to_user_facing_string(), 4, 4)
                }
                _ => self.to_user_facing_string(),
            },
            AddressFormat::Full => self.to_user_facing_string(),
            AddressFormat::Raw => self.to_string(),
        }
    }
    pub fn to_user_facing_string(&self) -> String {
        let mut raw = self.to_string();
        _ = raw.drain(..1);
        _ = raw.drain(raw.len() - 1..);
        raw.to_owned()
    }
}

impl NonFungibleLocalId {
    fn scrypto(&self) -> ScryptoNonFungibleLocalId {
        self.clone().into()
    }
}

impl std::fmt::Display for NonFungibleLocalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scrypto())
    }
}

impl From<AddressOfAccountOrPersona> for NonFungibleLocalId {
    fn from(value: AddressOfAccountOrPersona) -> Self {
        match value {
            AddressOfAccountOrPersona::Account(account_address) => {
                Self::from(account_address)
            }
            AddressOfAccountOrPersona::Identity(identity_address) => {
                Self::from(identity_address)
            }
        }
    }
}

impl From<AccountAddress> for NonFungibleLocalId {
    fn from(value: AccountAddress) -> Self {
        Self::bytes(value.node_id().0).expect(
            "NonFungibleLocalId bytes size is always NodeId::Length (30)",
        )
    }
}

impl From<IdentityAddress> for NonFungibleLocalId {
    fn from(value: IdentityAddress) -> Self {
        Self::bytes(value.node_id().0).expect(
            "NonFungibleLocalId bytes size is always NodeId::Length (30)",
        )
    }
}

impl From<ScryptoNonFungibleLocalId> for NonFungibleLocalId {
    fn from(value: ScryptoNonFungibleLocalId) -> Self {
        match value {
            ScryptoNonFungibleLocalId::String(value) => Self::Str {
                value: value.into(),
            },
            ScryptoNonFungibleLocalId::Integer(value) => Self::Integer {
                value: value.value(),
            },
            ScryptoNonFungibleLocalId::Bytes(value) => Self::Bytes {
                value: NonEmptyMax64Bytes::try_from(value.value()).expect(
                    "ScryptoBytesNonFungibleLocalId should have 64 bytes",
                ),
            },
            ScryptoNonFungibleLocalId::RUID(value) => Self::Ruid {
                value: value.value().into(),
            },
        }
    }
}

impl From<NonFungibleLocalId> for ScryptoNonFungibleLocalId {
    fn from(value: NonFungibleLocalId) -> Self {
        match value {
            NonFungibleLocalId::Str { value } => Self::String(value.into()),
            NonFungibleLocalId::Bytes { value } => Self::bytes(value.to_vec()).expect("Should always be able to create Scrypto NonFungibleLocalId from bytes."),
            NonFungibleLocalId::Ruid { value } => Self::ruid(*value.bytes()),
            NonFungibleLocalId::Integer { value } => Self::integer(value),
        }
    }
}

impl FromStr for NonFungibleLocalId {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        ScryptoNonFungibleLocalId::from_str(s)
            .map(Self::from)
            .map_err(|_| CommonError::InvalidNonFungibleLocalIDString)
    }
}

#[cfg(test)]
impl From<&str> for NonFungibleLocalId {
    /// TEST ONLY
    fn from(value: &str) -> Self {
        value.parse().unwrap_or_else(|_| panic!("Test failed since the passed in str is not a valid NonFungibleLocalId: '{}'", value))
    }
}

impl HasSampleValues for NonFungibleLocalId {
    fn sample() -> Self {
        Self::ruid(Exactly32Bytes::sample_dead()).unwrap()
    }

    fn sample_other() -> Self {
        Self::string("foobar").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleLocalId;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn ord() {
        assert!(SUT::integer(1) < SUT::integer(2));
    }

    #[test]
    fn from_str_ok() {
        assert_eq!(
            "<value>".parse::<SUT>().unwrap(),
            SUT::string("value").unwrap()
        );
    }

    #[test]
    fn from_invalid_str_error() {
        assert_eq!(
            "#super invalid string!!!!!#".parse::<SUT>(),
            Err::<SUT, _>(CommonError::InvalidNonFungibleLocalIDString)
        );
    }

    #[test]
    fn display_integer() {
        assert_eq!(format!("{}", SUT::integer(1234)), "#1234#");
    }

    #[test]
    fn display_str() {
        assert_eq!(format!("{}", SUT::string("foo").unwrap()), "<foo>");
    }

    #[test]
    fn display_ruid() {
        assert_eq!(
            format!(
                "{}",
                SUT::ruid(
                    hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
                ).unwrap()
            ),
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}"
        );
    }

    #[test]
    fn display_bytes() {
        assert_eq!(format!("{}", SUT::bytes([0xde, 0xad]).unwrap()), "[dead]");
    }

    #[test]
    fn to_user_facing_string_variant_string() {
        assert_eq!(SUT::string("foo").unwrap().to_user_facing_string(), "foo");
    }

    #[test]
    fn to_user_facing_string_variant_integer() {
        assert_eq!(SUT::integer(1234).to_user_facing_string(), "1234");
    }

    #[test]
    fn to_user_facing_string_variant_bytes() {
        assert_eq!(
            SUT::bytes([0xde, 0xad]).unwrap().to_user_facing_string(),
            "dead"
        );
    }

    #[test]
    fn to_user_facing_string_variant_ruid() {
        assert_eq!(SUT::ruid(
            hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
        ).unwrap().to_user_facing_string(), "deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210");
    }

    #[test]
    fn formatted_raw_variant_string() {
        assert_eq!(
            SUT::string("foo").unwrap().formatted(AddressFormat::Raw),
            "<foo>"
        );
    }

    #[test]
    fn formatted_raw_variant_integer() {
        assert_eq!(SUT::integer(1234).formatted(AddressFormat::Raw), "#1234#");
    }

    #[test]
    fn formatted_raw_variant_bytes() {
        assert_eq!(
            SUT::bytes([0xde, 0xad])
                .unwrap()
                .formatted(AddressFormat::Raw),
            "[dead]"
        );
    }

    #[test]
    fn formatted_raw_variant_ruid() {
        assert_eq!(SUT::ruid(
            hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
        ).unwrap().formatted(AddressFormat::Raw), "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}");
    }

    #[test]
    fn formatted_default_variant_string() {
        assert_eq!(
            SUT::string("foo")
                .unwrap()
                .formatted(AddressFormat::Default),
            "foo"
        );
    }

    #[test]
    fn formatted_default_variant_integer() {
        assert_eq!(
            SUT::integer(1234).formatted(AddressFormat::Default),
            "1234"
        );
    }

    #[test]
    fn formatted_default_variant_bytes() {
        assert_eq!(
            SUT::bytes([0xde, 0xad])
                .unwrap()
                .formatted(AddressFormat::Default),
            "dead"
        );
    }

    #[test]
    fn formatted_default_variant_ruid() {
        assert_eq!(SUT::ruid(
            hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
        ).unwrap().formatted(AddressFormat::Default), "dead...3210");
    }

    #[test]
    fn formatted_full_variant_string() {
        assert_eq!(
            SUT::string("foo").unwrap().formatted(AddressFormat::Full),
            "foo"
        );
    }

    #[test]
    fn formatted_full_variant_integer() {
        assert_eq!(SUT::integer(1234).formatted(AddressFormat::Full), "1234");
    }

    #[test]
    fn formatted_full_variant_bytes() {
        assert_eq!(
            SUT::bytes([0xde, 0xad])
                .unwrap()
                .formatted(AddressFormat::Full),
            "dead"
        );
    }

    #[test]
    fn formatted_full_variant_ruid() {
        assert_eq!(SUT::ruid(
            hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
        ).unwrap().formatted(AddressFormat::Full), "deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210");
    }

    #[test]
    fn from_str_err() {
        assert!(SUT::from_str("no_angle_brackets").is_err());
    }

    #[test]
    fn invalid_local_id_string() {
        assert_eq!(
            SUT::string(""),
            Err::<SUT, _>(CommonError::InvalidNonFungibleLocalIDString)
        );
    }

    #[test]
    fn from_native_ruid() {
        let bytes = Exactly32Bytes::sample_dead();
        let value = SUT::ruid(bytes).unwrap();
        let scrypto = ScryptoNonFungibleLocalId::RUID(
            ScryptoRUIDNonFungibleLocalId::new(*bytes.bytes()),
        );
        assert_eq!(value.clone(), scrypto.clone().into());
        assert_eq!(
            ScryptoNonFungibleLocalId::from(value.clone()),
            scrypto.clone()
        );
        assert_eq!(
            SUT::from_str(value.clone().to_string().as_str()),
            Ok(value)
        );
    }

    fn test_from_bytes<const N: usize>() {
        let bytes = [0xab; N];
        let value = SUT::bytes(bytes).unwrap();
        let scrypto = ScryptoNonFungibleLocalId::Bytes(
            ScryptoBytesNonFungibleLocalId::new(bytes.clone().to_vec())
                .unwrap(),
        );

        assert_eq!(
            ScryptoNonFungibleLocalId::from(value.clone()),
            scrypto.clone()
        );
        assert_eq!(
            SUT::from_str(value.clone().to_string().as_str()),
            Ok(value)
        );
    }

    #[test]
    fn from_bytes() {
        assert_eq!(SUT::bytes([]), Err(CommonError::BytesEmpty));
        test_from_bytes::<1>();
        test_from_bytes::<2>();
        test_from_bytes::<63>();
        test_from_bytes::<64>();
    }

    #[test]
    fn random() {
        let mut set: HashSet<SUT> = HashSet::new();
        let n = 100;
        for _ in 0..n {
            set.insert(SUT::random());
        }
        assert_eq!(set.len(), n);
    }

    #[test]
    fn from_native_str() {
        let value = SUT::string("test").unwrap();
        let scrypto = ScryptoNonFungibleLocalId::String(
            ScryptoStringNonFungibleLocalId::new("test").unwrap(),
        );
        assert_eq!(value.clone(), scrypto.clone().into());
        assert_eq!(
            SUT::from_str(value.clone().to_string().as_str()),
            Ok(value)
        );
    }

    #[test]
    fn from_native_integer() {
        let non_native = SUT::Integer { value: 1234 };
        let native = ScryptoNonFungibleLocalId::Integer(
            ScryptoIntegerNonFungibleLocalId::new(1234),
        );
        assert_eq!(non_native.clone(), native.clone().into());
        assert_eq!(
            SUT::from_str(non_native.clone().to_string().as_str()),
            Ok(non_native)
        );
    }

    #[test]
    fn display_rui() {
        assert_eq!(
            SUT::ruid(
                hex_decode("deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210").unwrap()
            )
                .unwrap()
                .to_string(),
            "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}"
        );
    }

    #[test]
    fn test_derives_account_address() {
        let local_id = SUT::from(
            "[511dc6eee81feec3439609a650807168995ff4bc0c04986e0f089f0bc7fc]",
        );
        let account_address = AccountAddress::try_from_bech32(
            "account_tdx_2_12ywudmhgrlhvxsukpxn9pqr3dzv4la9upszfsms0pz0sh3lu6erxux"
        ).unwrap();

        assert!(local_id.derives_account_address(account_address));
    }

    #[test]
    fn test_byte_id_not_deriving_account_address() {
        let local_id = SUT::from(
            "[511dc6eee81feec3439609a650807168995ff4bc0c04986e0f089f0bc7fc511dc6eee81feec3439609a650807168995ff4bc0c04986e0f089f0bc7fc]",
        );
        let account_address = AccountAddress::try_from_bech32(
            "account_tdx_2_12ywudmhgrlhvxsukpxn9pqr3dzv4la9upszfsms0pz0sh3lu6erxux"
        ).unwrap();

        assert!(!local_id.derives_account_address(account_address));
    }
}
