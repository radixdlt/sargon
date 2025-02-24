use crate::{CommonError, RetManifestAnalysisError};
use addresses::error;

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
