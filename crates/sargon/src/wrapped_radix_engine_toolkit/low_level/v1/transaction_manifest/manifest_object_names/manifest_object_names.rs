use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ManifestObjectNames {
    pub secret_magic: ManifestObjectNamesSecretMagic,
}

impl Default for ManifestObjectNames {
    fn default() -> Self {
        ManifestObjectNamesSecretMagic::default().into()
    }
}

impl From<ManifestObjectNamesSecretMagic> for ManifestObjectNames {
    fn from(value: ManifestObjectNamesSecretMagic) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl From<ScryptoManifestObjectNames> for ManifestObjectNames {
    fn from(value: ScryptoManifestObjectNames) -> Self {
        Self {
            secret_magic: value.into(),
        }
    }
}

impl From<ManifestObjectNames> for ScryptoManifestObjectNames {
    fn from(value: ManifestObjectNames) -> Self {
        value.secret_magic.into()
    }
}

impl HasSampleValues for ManifestObjectNames {
    fn sample() -> Self {
        ManifestObjectNamesSecretMagic::sample().into()
    }

    fn sample_other() -> Self {
        ManifestObjectNamesSecretMagic::sample_other().into()
    }
}
