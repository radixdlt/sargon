use crate::prelude::*;

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq, Hash)]

pub enum PasswordBasedKeyDerivationScheme {
    Version1(PasswordBasedKeyDerivationSchemeVersion1),
}

impl std::fmt::Display for PasswordBasedKeyDerivationScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<PasswordBasedKeyDerivationSchemeVersion>
    for PasswordBasedKeyDerivationScheme
{
    fn from(value: PasswordBasedKeyDerivationSchemeVersion) -> Self {
        match value {
            PasswordBasedKeyDerivationSchemeVersion::Version1 => {
                Self::Version1(
                    PasswordBasedKeyDerivationSchemeVersion1::default(),
                )
            }
        }
    }
}
impl VersionedAlgorithm for PasswordBasedKeyDerivationScheme {
    type Version = PasswordBasedKeyDerivationSchemeVersion;
    fn version(&self) -> Self::Version {
        match self {
            PasswordBasedKeyDerivationScheme::Version1(_) => {
                PasswordBasedKeyDerivationSchemeVersion1::version()
            }
        }
    }
    fn description(&self) -> String {
        match self {
            PasswordBasedKeyDerivationScheme::Version1(_) => {
                PasswordBasedKeyDerivationSchemeVersion1::description()
            }
        }
    }
}

pub trait VersionedKeyDerivation<V> {
    fn version() -> V;
    fn description() -> String;
}

pub trait VersionedPasswordBasedKeyDerivation<
    V = PasswordBasedKeyDerivationSchemeVersion,
>: VersionedKeyDerivation<V>
{
    fn kdf(password: String) -> Exactly32Bytes;
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

impl Serialize for PasswordBasedKeyDerivationSchemeVersion1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer
            .serialize_struct("PasswordBasedKeyDerivationSchemeVersion1", 2)?;
        state.serialize_field("description", &Self::description())?;
        state.serialize_field("version", &Self::version())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for PasswordBasedKeyDerivationSchemeVersion1 {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            version: PasswordBasedKeyDerivationSchemeVersion,
        }
        let version = Wrapper::deserialize(deserializer).map(|w| w.version)?;
        match version {
            PasswordBasedKeyDerivationSchemeVersion::Version1 => {
                Ok(PasswordBasedKeyDerivationSchemeVersion1::default())
            }
        }
    }
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
#[debug("{}", Self::description())]
#[display("{}", Self::description())]
pub struct PasswordBasedKeyDerivationSchemeVersion1 {}

impl VersionedKeyDerivation<PasswordBasedKeyDerivationSchemeVersion>
    for PasswordBasedKeyDerivationSchemeVersion1
{
    fn description() -> String {
        Self::DESCRIPTION.to_owned()
    }

    fn version() -> PasswordBasedKeyDerivationSchemeVersion {
        PasswordBasedKeyDerivationSchemeVersion::Version1
    }
}

impl
    VersionedPasswordBasedKeyDerivation<PasswordBasedKeyDerivationSchemeVersion>
    for PasswordBasedKeyDerivationSchemeVersion1
{
    fn kdf(password: String) -> Exactly32Bytes {
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
    type SUT = PasswordBasedKeyDerivationSchemeVersion1;

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
    fn kdf() {
        let test = |pwd: &str, exp: &str| {
            let key = SUT::kdf(pwd.to_owned());
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
