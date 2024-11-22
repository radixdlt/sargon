use crate::prelude::*;

/// Describes a manifest that can be statically analyzed
pub trait StaticallyAnalyzableManifest {
    /// Get the summary on a given network
    fn summary(&self, network_id: NetworkID) -> Result<ManifestSummary>;

    /// Get the summary on a given network by validating against reserved instructions
    fn validated_summary(
        &self,
        network_id: NetworkID,
        are_instructions_originating_from_host: bool,
    ) -> Result<ManifestSummary> {
        let summary = self.summary(network_id)?;
        // Transactions created outside of the Wallet are not allowed to use reserved instructions
        if !are_instructions_originating_from_host
            && !summary.reserved_instructions.is_empty()
        {
            return Err(
                CommonError::ReservedInstructionsNotAllowedInManifest {
                    reserved_instructions: summary
                        .reserved_instructions
                        .iter()
                        .map(|i| i.to_string())
                        .collect(),
                },
            );
        }

        Ok(summary)
    }

    /// Just validate the instrctutions on a given network against the reserved instructions
    fn validate_instructions(
        &self,
        network_id: NetworkID,
        are_instructions_originating_from_host: bool,
    ) -> Result<()> {
        self.validated_summary(
            network_id,
            are_instructions_originating_from_host,
        )?;
        Ok(())
    }
}
