use crate::prelude::*;

impl GatewayClient {
    /// Returns detailed information for collection of entities. Aggregate resources globally by default.
    pub(crate) async fn state_entity_details(
        &self,
        request: StateEntityDetailsRequest,
    ) -> Result<StateEntityDetailsResponse> {
        self.post("state/entity/details", request, res_id).await
    }
}
