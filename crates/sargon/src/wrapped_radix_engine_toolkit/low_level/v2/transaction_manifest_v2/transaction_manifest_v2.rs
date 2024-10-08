use crate::prelude::*;
use radix_common::prelude::ManifestBucket;
use radix_transactions::manifest::KnownManifestObjectNames;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record, derive_more::Display)]
#[display("{}", self.instructions_string())] // TODO: add blobs, children, object_names to Display
pub struct TransactionManifestV2 {
    secret_magic: TransactionManifestSecretMagicV2,
}

impl TransactionManifestV2 {
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            secret_magic: TransactionManifestSecretMagicV2 {
                instructions: InstructionsV2::empty(network_id),
                blobs: Blobs::default(),
                children: ChildIntents::empty(),
                object_names: ManifestObjectNames::default(),
            },
        }
    }
}

impl From<TransactionManifestSecretMagicV2> for TransactionManifestV2 {
    fn from(value: TransactionManifestSecretMagicV2) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl TransactionManifestV2 {
    pub(crate) fn scrypto_manifest(&self) -> ScryptoTransactionManifestV2 {
        ScryptoTransactionManifestV2 {
            instructions: self.instructions().clone(),
            blobs: self.secret_magic.blobs.clone().into(),
            children: self.secret_magic.children.clone().into(),
            object_names: self.secret_magic.object_names.clone().into(),
        }
    }
}

impl From<TransactionManifestV2> for ScryptoTransactionManifestV2 {
    fn from(value: TransactionManifestV2) -> Self {
        value.scrypto_manifest()
    }
}

impl TryFrom<(ScryptoTransactionManifestV2, NetworkID)>
    for TransactionManifestV2
{
    type Error = CommonError;
    fn try_from(
        value: (ScryptoTransactionManifestV2, NetworkID),
    ) -> Result<Self> {
        let scrypto_manifest = value.0;
        let network_id = value.1;
        let instructions = InstructionsV2::try_from((
            scrypto_manifest.clone().instructions.as_ref(),
            network_id,
        ))?;
        let value = Self {
            secret_magic: TransactionManifestSecretMagicV2::new(
                instructions,
                scrypto_manifest.blobs.clone(),
                (scrypto_manifest.children.clone(), network_id).into(),
                scrypto_manifest.object_names.clone().into(),
            ),
        };
        assert_eq!(value.scrypto_manifest(), scrypto_manifest);
        Ok(value)
    }
}

impl TransactionManifestV2 {
    pub fn sargon_built(
        builder: ScryptoManifestV2Builder,
        network_id: NetworkID,
    ) -> Self {
        let scrypto_manifest = builder.build();
        Self::try_from((scrypto_manifest, network_id)).expect(
            "Sargon should not build manifest with too nested SBOR depth.",
        )
    }
}

impl TransactionManifestV2 {
    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstructionV2> {
        self.secret_magic.instructions()
    }

    pub(crate) fn blobs(&self) -> &Blobs {
        &self.secret_magic.blobs
    }

    pub(crate) fn object_names(&self) -> &ManifestObjectNames {
        &self.secret_magic.object_names
    }

    pub fn instructions_string(&self) -> String {
        self.secret_magic.instructions.instructions_string()
    }
}

impl HasSampleValues for TransactionManifestV2 {
    fn sample() -> Self {
        TransactionManifestSecretMagicV2::sample().into()
    }

    fn sample_other() -> Self {
        TransactionManifestSecretMagicV2::sample_other().into()
    }
}
