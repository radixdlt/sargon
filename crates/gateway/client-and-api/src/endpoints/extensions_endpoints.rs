use crate::prelude::*;

impl GatewayClient {
    /// Split the requirements into chunks and query for entities that have ever used
    /// a given requirement (resource or non-fungible global ID) in their access rules
    /// (blueprint authentication templates, owner roles, or role assignments)
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/EntitiesByRoleRequirementLookup
    pub async fn fetch_entities_by_role_requirement_lookup(
        &self,
        requirements: Vec<RoleRequirement>,
    ) -> Result<EntitiesByRoleRequirementLookupResponse> {
        let requirement_chunks = requirements
            .into_iter()
            .chunks(GATEWAY_CHUNK_ENTITIES_BY_ROLE_REQUIREMENT_LOOKUP as usize)
            .into_iter()
            .map(|c| c.collect_vec())
            .collect_vec();

        let mut result = Vec::<EntitiesByRoleRequirementLookupItem>::new();
        for chunk in requirement_chunks {
            let chunk_request = EntitiesByRoleRequirementLookupRequest {
                requirements: chunk,
            };
            let chunk_response: EntitiesByRoleRequirementLookupResponse = self
                .entities_by_role_requirement_lookup(chunk_request)
                .await?;
            result.extend(chunk_response.items);
        }

        Ok(EntitiesByRoleRequirementLookupResponse::new(result))
    }

    /// Query for entities that have ever used a given requirement
    /// (resource or non-fungible global ID) in their access rules
    /// (blueprint authentication templates, owner roles, or role assignments)
    ///
    /// See [the Gateway API docs for details][doc].
    ///
    /// [doc]: https://radix-babylon-gateway-api.redoc.ly/#operation/EntitiesByRoleRequirementLookup
    async fn entities_by_role_requirement_lookup(
        &self,
        request: EntitiesByRoleRequirementLookupRequest,
    ) -> Result<EntitiesByRoleRequirementLookupResponse> {
        self.post(
            Self::PATH_ENTITIES_BY_ROLE_REQUIREMENT_LOOKUP,
            request,
            res_id,
        )
        .await
    }
}
