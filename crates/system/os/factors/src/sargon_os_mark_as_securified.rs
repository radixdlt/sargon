use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsMarkAsSecurified {
    /// Marks the entity as securified and returns the mutated entity.
    /// Does not save into profile.
    fn mark_entity_as_securified(
        &self,
        access_controller_address: AccessControllerAddress,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona>;
}

impl OsMarkAsSecurified for SargonOS {
    fn mark_entity_as_securified(
        &self,
        access_controller_address: AccessControllerAddress,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona> {
        let mut entity = self.entity_by_address(entity_address)?;

        let transaction_signing = entity
            .security_state()
            .as_unsecured()
            .ok_or(CommonError::SecurityStateSecurifiedButExpectedUnsecurified)
            .map(|security_state| security_state.transaction_signing.clone())?;

        let security_structure_of_factor_instances = entity
            .get_provisional()
            .and_then(|p| p.into_factor_instances_derived().ok())
            .ok_or(CommonError::EntityHasNoProvisionalSecurityConfigSet)?;

        let secured_entity_control = SecuredEntityControl::new(
            transaction_signing,
            access_controller_address,
            security_structure_of_factor_instances,
        )?;

        entity.set_provisional(None);
        entity.set_security_state(EntitySecurityState::Securified {
            value: secured_entity_control,
        })?;

        Ok(entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_unsecurified_account_with_provisional_is_marked_as_securified(
    ) {
        // ARRANGE
        let sut = SUT::fast_boot().await;
        let shield_id = add_unsafe_shield(&sut).await.unwrap();
        let network = NetworkID::Mainnet;
        let account_address = sut
            .create_and_save_new_account_with_main_bdfs(
                network,
                DisplayName::sample(),
            )
            .await
            .map(|a| AddressOfAccountOrPersona::from(a.address))
            .unwrap();

        sut.apply_security_shield_with_id_to_entities(
            shield_id,
            IndexSet::just(account_address.clone()),
        )
        .await
        .unwrap();
        let access_controller_address = AccessControllerAddress::sample();

        // ACT
        let updated_entity = sut
            .mark_entity_as_securified(
                access_controller_address,
                account_address.clone(),
            )
            .unwrap();

        // ASSERT
        assert_eq!(updated_entity.get_provisional(), None);
        let security_state = updated_entity.security_state();
        let securified = security_state.as_securified().unwrap();
        assert_eq!(
            securified.access_controller_address,
            access_controller_address
        );
        assert_eq!(
            securified.security_structure.security_structure_id,
            shield_id
        )
    }
}
