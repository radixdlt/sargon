use crate::prelude::*;

mod compiled_notarized_intent;
mod compiled_transaction_intent;
mod dynamically_analyzable_manifest;
mod execution_summary;
mod intent_signature;
mod manifest_encountered_component_address;
mod manifest_summary;
mod notarized_transaction;
mod notary_signature;
mod sbor_depth_validation;
mod signed_intent;
mod statically_analyzable_manifest;
mod transaction_classes;
mod transaction_hashes;
mod v1;
mod v2;

pub use compiled_notarized_intent::*;
pub use compiled_transaction_intent::*;
pub use dynamically_analyzable_manifest::*;
pub use execution_summary::*;
pub use intent_signature::*;
pub use manifest_encountered_component_address::*;
pub use manifest_summary::*;
pub use notarized_transaction::*;
pub use notary_signature::*;
pub use signed_intent::*;
pub use statically_analyzable_manifest::*;
pub use transaction_classes::*;
pub use transaction_hashes::*;
pub use v1::intent_signatures::*;
pub use v1::*;
pub use v2::*;

pub(crate) fn map_static_analysis_error(
    error: RetManifestAnalysisError,
) -> CommonError {
    error!(
        "Failed to get execution summary from RET, error: {:?}",
        error
    );
    CommonError::FailedToGenerateManifestSummary {
        underlying: format!("{:?}", error),
    }
}
