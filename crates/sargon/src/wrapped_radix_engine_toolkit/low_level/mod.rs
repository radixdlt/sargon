mod address_conversion;
mod compiled_notarized_intent;
mod compiled_transaction_intent;
mod dynamically_analyzable_manifest;
mod execution_summary;
mod intent_signature;
mod manifest_summary;
mod notarized_transaction;
mod notary_signature;
mod public_key_hash;
mod sbor_depth_validation;
mod signed_intent;
mod statically_analyzable_manifest;
mod transaction_classes;
mod transaction_hashes;
mod v1;
mod v2;

pub use address_conversion::*;
pub use compiled_notarized_intent::*;
pub use compiled_transaction_intent::*;
pub use dynamically_analyzable_manifest::*;
pub use execution_summary::*;
pub use intent_signature::*;
pub use manifest_summary::*;
pub use notarized_transaction::*;
pub use notary_signature::*;
pub use public_key_hash::*;
pub(crate) use sbor_depth_validation::*;
pub use signed_intent::*;
pub use statically_analyzable_manifest::*;
pub use transaction_classes::*;
pub use transaction_hashes::*;
pub use v1::intent_signatures::*;
pub use v1::*;
pub use v2::*;

pub(crate) fn map_static_analysis_error(
    error: radix_transactions::manifest::static_resource_movements::StaticResourceMovementsError,
) -> crate::prelude::CommonError {
    crate::prelude::error!(
        "Failed to get execution summary from RET, error: {:?}",
        error
    );
    crate::prelude::CommonError::FailedToGenerateManifestSummary {
        underlying: format!("{:?}", error),
    }
}
