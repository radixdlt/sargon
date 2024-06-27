use crate::prelude::*;

use hkdf::Hkdf;
use k256::sha2::Sha256;

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
pub struct PbHkdfSha256 {}

impl PbHkdfSha256 {
    pub fn derive_key(
        ikm: impl AsRef<[u8]>,
        salt: Option<&[u8]>,
        info: Option<&[u8]>,
    ) -> Exactly32Bytes {
        let mut okm = [0u8; 32]; // 32-byte buffer for the symmetric key

        let hkdf = Hkdf::<Sha256>::new(salt, ikm.as_ref());
        hkdf.expand(info.unwrap_or(&[]), &mut okm).unwrap();

        Exactly32Bytes::from(&okm)
    }
}

impl VersionOfAlgorithm for PbHkdfSha256 {
    type Version = PasswordBasedKeyDerivationSchemeVersion;

    fn description(&self) -> String {
        Self::DESCRIPTION.to_owned()
    }

    fn version(&self) -> Self::Version {
        Self::Version::Version1
    }
}

impl VersionedPasswordBasedKeyDerivation for PbHkdfSha256 {
    fn kdf(&self, password: impl AsRef<str>) -> Exactly32Bytes {
        // Input Key Material
        let ikm = password.as_ref().bytes().collect::<Vec<u8>>();
        Self::derive_key(ikm, None, None)
    }
}

impl PbHkdfSha256 {
    pub const DESCRIPTION: &'static str =
        "HKDFSHA256-with-UTF8-encoding-of-password-no-salt-no-info";
}
