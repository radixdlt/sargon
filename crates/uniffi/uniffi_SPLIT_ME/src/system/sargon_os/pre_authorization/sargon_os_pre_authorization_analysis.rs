use sargon::OsAnalysePreAuthPreview;
use sargon::PreAuthPreviewOptions as InternalPreAuthPreviewOptions;

use crate::prelude::*;

/// Host-controlled options for `analyse_pre_auth_preview_with_options`.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
    uniffi::Record,
)]
pub struct PreAuthPreviewOptions {
    /// Allow `create_proof` on AccessControllers that do not secure any entity
    /// in this wallet's Profile, i.e. multisig accounts controlled by an
    /// external AccessController. Every other AccessController method, every
    /// other reserved instruction, and any role-assignment change stays
    /// forbidden. Off by default.
    pub allow_external_access_controller_proofs: bool,
}

#[uniffi::export]
impl SargonOS {
    /// Performs initial transaction analysis for a given raw manifest, including:
    /// 1. Creating the SubintentManifest.
    /// 2. Validating if the manifest is open or enclosed.
    /// 3. If open, the manifest with its summary is returned.
    /// 4. If enclosed, it extracts the transaction signers and then transaction preview GW request is executed.
    /// 3. The execution summary is created with the manifest and receipt.
    ///
    ///     Maps relevant errors to ensure proper handling by the hosts.
    pub async fn analyse_pre_auth_preview(
        &self,
        instructions: String,
        blobs: Blobs,
        nonce: Nonce,
        notary_public_key: PublicKey,
    ) -> Result<PreAuthToReview> {
        self.wrapped
            .analyse_pre_auth_preview(
                instructions,
                blobs.into_internal(),
                nonce.into_internal(),
                notary_public_key.into_internal(),
            )
            .await
            .into_result()
    }

    /// Same as `analyse_pre_auth_preview`, with host-controlled options.
    ///
    /// With `PreAuthPreviewOptions::default()` the behaviour is identical to
    /// `analyse_pre_auth_preview`.
    pub async fn analyse_pre_auth_preview_with_options(
        &self,
        instructions: String,
        blobs: Blobs,
        nonce: Nonce,
        notary_public_key: PublicKey,
        options: PreAuthPreviewOptions,
    ) -> Result<PreAuthToReview> {
        self.wrapped
            .analyse_pre_auth_preview_with_options(
                instructions,
                blobs.into_internal(),
                nonce.into_internal(),
                notary_public_key.into_internal(),
                options.into_internal(),
            )
            .await
            .into_result()
    }
}
