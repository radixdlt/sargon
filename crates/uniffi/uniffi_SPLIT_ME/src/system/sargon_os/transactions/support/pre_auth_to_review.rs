use crate::prelude::*;
use sargon::PreAuthEnclosedManifest as InternalPreAuthEnclosedManifest;
use sargon::PreAuthOpenManifest as InternalPreAuthOpenManifest;
use sargon::PreAuthToReview as InternalPreAuthToReview;

/// This is the result of the Pre-Auth preview analysis.
/// It contains all the information needed to compute and display the transaction details to the user.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum PreAuthToReview {
    Open(PreAuthOpenManifest),
    Enclosed(PreAuthEnclosedManifest),
}

/// Pre-Auth analysis open manifest, which contains multiple interactions with the parent manifest,
/// thus its preview can be computed only based on the static analysis manifest summary
#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct PreAuthOpenManifest {
    pub manifest: SubintentManifest,
    pub summary: ManifestSummary,
}

/// Pre-Auth analysis enclosed manifest, which does not contain any interactions with the parent manifest,
/// thus its preview can be computed as if it would have been a standalone transaction.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct PreAuthEnclosedManifest {
    pub manifest: SubintentManifest,
    pub summary: ExecutionSummary,
}
