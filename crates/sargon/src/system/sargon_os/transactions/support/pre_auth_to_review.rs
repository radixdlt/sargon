use crate::prelude::*;

pub enum PreAuthToReview {
    Open(PreAuthOpenManifest),
    Enclosed(PreAuthEnclosedManifest)
}

pub struct PreAuthOpenManifest {
    pub manifest: SubintentManifest,
    pub summary: ManifestSummary
}

pub struct PreAuthEnclosedManifest {
    pub manifest: SubintentManifest,
    pub summary: ExecutionSummary
}