use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsCommitProvisionalSecurityState {
    async fn commit_provisional_security_state(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<()>;

    async fn remove_provisional_security_state(
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
        let gateway_client = self.gateway_client();

        let access_controller_per_entity = gateway_client
            .fetch_access_controllers_of_entities([entity_address])
            .await?;

        let Some(access_controller_address) =
            access_controller_per_entity.get(&entity_address).cloned()
        else {
            return Err(CommonError::EntityIsNotControlledByAnAccessControllerOnLedger {
                    entity_bech32m_encoded_address: entity_address.to_string(),
                });
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

    async fn remove_provisional_security_state(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<()> {
        self.update_profile_with(|profile| {
            profile.remove_provisional_security_state(entity_address)?;
            Ok(())
        })
        .await
    }
}
