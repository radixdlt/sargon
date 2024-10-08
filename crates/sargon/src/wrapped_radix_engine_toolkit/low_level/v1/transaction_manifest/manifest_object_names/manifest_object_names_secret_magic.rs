use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum, Default)]
pub enum ManifestObjectNamesSecretMagic {
    #[default]
    Unknown,
    Known(KnownManifestObjectNames),
}

impl From<ScryptoManifestObjectNames> for ManifestObjectNamesSecretMagic {
    fn from(value: ScryptoManifestObjectNames) -> Self {
        match value {
            ScryptoManifestObjectNames::Unknown => {
                ManifestObjectNamesSecretMagic::Unknown
            }
            ScryptoManifestObjectNames::Known(scrypto_known) => {
                ManifestObjectNamesSecretMagic::Known(scrypto_known.into())
            }
        }
    }
}

impl From<ManifestObjectNamesSecretMagic> for ScryptoManifestObjectNames {
    fn from(value: ManifestObjectNamesSecretMagic) -> Self {
        match value {
            ManifestObjectNamesSecretMagic::Unknown => {
                ScryptoManifestObjectNames::Unknown
            }
            ManifestObjectNamesSecretMagic::Known(known) => {
                ScryptoManifestObjectNames::Known(known.into())
            }
        }
    }
}

impl HasSampleValues for ManifestObjectNamesSecretMagic {
    fn sample() -> Self {
        Self::Unknown
    }

    fn sample_other() -> Self {
        Self::Known(KnownManifestObjectNames::sample())
    }
}
