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
        let hk = Hkdf::<Sha256>::new(None, &ikm);

        let mut okm = [0u8; 32];
        hk.expand(&[], &mut okm).unwrap();
        Exactly32Bytes::from(&okm)
    }
}

impl PbHkdfSha256 {
    pub const DESCRIPTION: &'static str =
        "HKDFSHA256-with-UTF8-encoding-of-password-no-salt-no-info";
}
