use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct KnownManifestObjectNames {
    pub(crate) secret_magic: KnownManifestObjectNamesSecretMagic,
}

impl From<KnownManifestObjectNamesSecretMagic> for KnownManifestObjectNames {
    fn from(value: KnownManifestObjectNamesSecretMagic) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl From<ScryptoKnownManifestObjectNames> for KnownManifestObjectNames {
    fn from(value: ScryptoKnownManifestObjectNames) -> Self {
        Self {
            secret_magic: value.into(),
        }
    }
}

impl From<KnownManifestObjectNames> for ScryptoKnownManifestObjectNames {
    fn from(value: KnownManifestObjectNames) -> Self {
        value.secret_magic.into()
    }
}

impl HasSampleValues for KnownManifestObjectNames {
    fn sample() -> Self {
        KnownManifestObjectNamesSecretMagic::sample().into()
    }

    fn sample_other() -> Self {
        KnownManifestObjectNamesSecretMagic::sample_other().into()
    }
}
