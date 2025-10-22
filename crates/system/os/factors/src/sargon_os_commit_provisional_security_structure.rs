use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsCommitProvisionalSecurityState {
    async fn commit_provisional_security_state(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl OsCommitProvisionalSecurityState for SargonOS {
    async fn commit_provisional_security_state(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<()> {
        let (gateway_client, network_id) = self.gateway_client_on()?;

        // Fetch ancestor addresses
        let badge_owner_per_entity = gateway_client
            .fetch_entities_badge_owners(network_id, vec![entity_address.clone()])
            .await?;

        let maybe_badge_owner = badge_owner_per_entity
            .get(&entity_address)
            .unwrap_or(&None);

            let Some(access_controller_address) = maybe_badge_owner.and_then(|a| {
                a.as_access_controller().cloned()
            }) else {
                return Err(CommonError::EntityIsNotControlledByAnAccessControllerOnLedger {
                    entity_bech32m_encoded_address: entity_address.to_string(),
                })
            };

        let entity = self.entity_by_address(entity_address)?;

        self.update_profile_with(|profile| {
            if entity.is_securified() {
                profile.commit_provisional_security_state(entity_address)?;
            } else {
                profile.mark_entity_as_securified(
                    access_controller_address,
                    entity_address,
                )?;
            }
           
            Ok(())
        })
        .await
    }

    
}
