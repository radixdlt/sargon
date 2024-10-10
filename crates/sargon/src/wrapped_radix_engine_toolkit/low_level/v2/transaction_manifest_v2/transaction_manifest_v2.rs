use crate::prelude::*;
use radix_common::prelude::ManifestBucket;
use radix_transactions::manifest::KnownManifestObjectNames;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record, derive_more::Display)]
#[display("{}", self.manifest_string())]
pub struct TransactionManifestV2 {
    secret_magic: TransactionManifestSecretMagicV2,
}

impl TransactionManifestV2 {
    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
        blobs: Blobs,
        children: ChildIntents,
    ) -> Result<Self> {
        InstructionsV2::new(instructions_string, network_id).map(
            |instructions| Self {
                secret_magic: TransactionManifestSecretMagicV2 {
                    instructions,
                    blobs,
                    children,
                },
            },
        )
    }

    pub fn with_instructions_and_blobs_and_children(
        instructions: InstructionsV2,
        blobs: Blobs,
        children: ChildIntents,
    ) -> Self {
        Self {
            secret_magic: TransactionManifestSecretMagicV2::new(
                instructions,
                blobs,
                children,
            ),
        }
    }
}

impl TransactionManifestV2 {
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            secret_magic: TransactionManifestSecretMagicV2 {
                instructions: InstructionsV2::empty(network_id),
                blobs: Blobs::default(),
                children: ChildIntents::empty(),
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
            object_names: Default::default(),
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
            ),
        };

        // Verify that the manifest can be decompiled and that the instructions are from a validated notarized transaction
        _ = manifest_v2_string_from(scrypto_manifest.clone(), network_id)?;

        assert_eq!(
            value.scrypto_manifest().instructions,
            scrypto_manifest.instructions
        );
        assert_eq!(value.scrypto_manifest().blobs, scrypto_manifest.blobs);
        assert_eq!(
            value.scrypto_manifest().children,
            scrypto_manifest.children
        );
        Ok(value)
    }
}

pub fn manifest_v2_string_from(
    scrypto_manifest: ScryptoTransactionManifestV2,
    network_id: NetworkID,
) -> Result<String, CommonError> {
    let network_definition = network_id.network_definition();
    scrypto_decompile(&scrypto_manifest, &network_definition).map_err(|e| {
        CommonError::InvalidManifestFailedToDecompile {
            underlying: format!("{:?}", e),
        }
    })
}

impl TransactionManifestV2 {
    pub fn sargon_built(
        builder: ScryptoTransactionManifestV2Builder,
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

    pub(crate) fn children(&self) -> &ChildIntents {
        &self.secret_magic.children
    }

    pub fn manifest_string(&self) -> String {
        manifest_v2_string_from(self.scrypto_manifest(), self.secret_magic.instructions.network_id).expect("Should never fail, because should never have allowed invalid manifest.")
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
