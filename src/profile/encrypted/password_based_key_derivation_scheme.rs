use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum PasswordBasedKeyDerivationScheme {
    Version1(PasswordBasedKeyDerivationSchemeVersion1),
}

#[cfg(not(tarpaulin_include))] // false negative
impl Serialize for PasswordBasedKeyDerivationScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer
            .serialize_struct("PasswordBasedKeyDerivationScheme", 2)?;
        state.serialize_field("description", &self.description())?;
        state.serialize_field("version", &self.version())?;
        state.end()
    }
}

#[cfg(not(tarpaulin_include))] // false negative
impl<'de> Deserialize<'de> for PasswordBasedKeyDerivationScheme {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            version: PasswordBasedKeyDerivationSchemeVersion,
        }
        Wrapper::deserialize(deserializer)
            .and_then(|w| Self::try_from(w.version).map_err(de::Error::custom))
    }
}

impl PasswordBasedKeyDerivationScheme {
    pub fn version1() -> Self {
        Self::Version1(PasswordBasedKeyDerivationSchemeVersion1::default())
    }
}
impl Default for PasswordBasedKeyDerivationScheme {
    fn default() -> Self {
        Self::version1()
    }
}

impl VersionedPasswordBasedKeyDerivation for PasswordBasedKeyDerivationScheme {
    fn kdf(&self, password: String) -> Exactly32Bytes {
        match self {
            PasswordBasedKeyDerivationScheme::Version1(scheme) => {
                scheme.kdf(password)
            }
        }
    }
}

impl std::fmt::Display for PasswordBasedKeyDerivationScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PasswordBasedKeyDerivationScheme: {} ({})",
            self.version(),
            self.description()
        )
    }
}

impl TryFrom<PasswordBasedKeyDerivationSchemeVersion>
    for PasswordBasedKeyDerivationScheme
{
    type Error = CommonError;
    fn try_from(
        value: PasswordBasedKeyDerivationSchemeVersion,
    ) -> Result<Self> {
        match value {
            PasswordBasedKeyDerivationSchemeVersion::Version1 => {
                Ok(Self::Version1(
                    PasswordBasedKeyDerivationSchemeVersion1::default(),
                ))
            }
        }
    }
}
impl VersionOfAlgorithm for PasswordBasedKeyDerivationScheme {
    type Version = PasswordBasedKeyDerivationSchemeVersion;
    fn version(&self) -> Self::Version {
        match self {
            Self::Version1(scheme) => scheme.version(),
        }
    }
    fn description(&self) -> String {
        match self {
            PasswordBasedKeyDerivationScheme::Version1(scheme) => {
                scheme.description()
            }
        }
    }
}

pub trait VersionedPasswordBasedKeyDerivation: VersionOfAlgorithm {
    fn kdf(&self, password: String) -> Exactly32Bytes;
}

/// The KDF algorithm used to derive the decryption key from a user provided password.
#[repr(u32)]
#[derive(
    Serialize_repr,
    Deserialize_repr,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
pub enum PasswordBasedKeyDerivationSchemeVersion {
    Version1 = 1,
}

/// A simple `HKDF` based scheme using UTF8 encoding of the password as input.
#[derive(
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
)]
#[debug("{}", self.description())]
#[display("{}", self.description())]
pub struct PasswordBasedKeyDerivationSchemeVersion1 {}

impl VersionOfAlgorithm for PasswordBasedKeyDerivationSchemeVersion1 {
    type Version = PasswordBasedKeyDerivationSchemeVersion;

    fn description(&self) -> String {
        Self::DESCRIPTION.to_owned()
    }

    fn version(&self) -> Self::Version {
        Self::Version::Version1
    }
}

impl VersionedPasswordBasedKeyDerivation
    for PasswordBasedKeyDerivationSchemeVersion1
{
    fn kdf(&self, password: String) -> Exactly32Bytes {
        use hkdf::Hkdf;
        use k256::sha2::Sha256;

        // Input Key Material
        let ikm = password.bytes().collect::<Vec<u8>>();
        let hk = Hkdf::<Sha256>::new(None, &ikm);

        let mut okm = [0u8; 32];
        hk.expand(&[], &mut okm).unwrap();
        Exactly32Bytes::from(&okm)
    }
}

impl PasswordBasedKeyDerivationSchemeVersion1 {
    pub const DESCRIPTION: &'static str =
        "HKDFSHA256-with-UTF8-encoding-of-password-no-salt-no-info";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PasswordBasedKeyDerivationScheme;

    #[test]
    fn json_() {
        let model = SUT::default();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
			"version": 1,
			"description": "HKDFSHA256-with-UTF8-encoding-of-password-no-salt-no-info"
		}
        "#,
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::default()), "PasswordBasedKeyDerivationScheme: Version1 (HKDFSHA256-with-UTF8-encoding-of-password-no-salt-no-info)");
    }

    #[test]
    fn kdf() {
        let sut = SUT::default();
        let test = |pwd: &str, exp: &str| {
            let key = sut.kdf(pwd.to_owned());
            assert_eq!(key.to_hex(), exp);
        };
        test(
            "Radix Rules!",
            "042f5ea1b7b384432fc6f8b8fdf413d59efbb30489c0e01aa0267e9c04aceee7",
        );

        // RFC 5869 test case 3: https://datatracker.ietf.org/doc/html/rfc5869#appendix-A.3
        test(
            &String::from_utf8(hex_decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap()).unwrap(), 
            &"8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8"[..64]
        );

        // We probably wont allow empty password in UI, but here is a unit test for it anyway...
        test(
            "",
            "eb70f01dede9afafa449eee1b1286504e1f62388b3f7dd4f956697b0e828fe18",
        );
    }
}
