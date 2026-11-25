use crate::prelude::*;

impl SargonOS {
    /// A transaction that sets the ROLA key of the securified entity at
    /// `address` to the one of its security structure.
    ///
    /// Securifying an entity or updating its shield doesn't set its ROLA key,
    /// so that those transactions only operate on access controllers and their
    /// review shows all they do. The host makes this transaction once the new
    /// security structure is in place, i.e. once securifying or the recovery
    /// is committed, or once a timed recovery is confirmed. It's signed with
    /// the primary role.
    pub async fn make_set_rola_key_manifest(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        let entity = self.profile()?.entity_by_address(address)?;
        let securified_entity = AnySecurifiedEntity::new(entity)?;
        let rola_key = securified_entity
            .securified_entity_control
            .security_structure
            .authentication_signing_factor_instance
            .clone();
        Ok(TransactionManifest::set_rola_key_of_securified_entity(
            &securified_entity,
            &rola_key,
        ))
    }

    /// A transaction that tops up the recovery fee vault of the access
    /// controller of the securified entity at `address` with `amount` XRD
    /// withdrawn from the account at `payer`.
    ///
    /// Securifying an entity or updating its shield doesn't top up the
    /// vault, so that those transactions only operate on access controllers
    /// and their review shows all they do.
    pub async fn make_top_up_access_controller_xrd_vault_manifest(
        &self,
        address: AddressOfAccountOrPersona,
        payer: AccountAddress,
        amount: Decimal192,
    ) -> Result<TransactionManifest> {
        let profile = self.profile()?;
        let securified_entity =
            AnySecurifiedEntity::new(profile.entity_by_address(address)?)?;
        let payer = profile.account_by_address(payer)?;
        Ok(TransactionManifest::top_up_access_controller_xrd_vault(
            securified_entity.access_controller_address(),
            &payer,
            amount,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn os_with(entity: Account) -> Arc<SargonOS> {
        let os = SargonOS::fast_boot().await;
        os.update_profile_with(|profile| {
            profile.networks.update_with(NetworkID::Mainnet, |network| {
                network.accounts =
                    Accounts::from_iter([entity.clone(), Account::sample()]);
            });
            Ok(())
        })
        .await
        .unwrap();
        os
    }

    #[actix_rt::test]
    async fn rola_key_is_set_to_the_security_structure_one() {
        let entity = SecurifiedAccount::sample().entity;
        let os = os_with(entity.clone()).await;

        let manifest = os
            .make_set_rola_key_manifest(entity.address.into())
            .await
            .unwrap();

        let rola_key = entity
            .security_state()
            .as_securified()
            .unwrap()
            .security_structure
            .authentication_signing_factor_instance
            .clone();
        assert_eq!(
            manifest,
            TransactionManifest::set_rola_key_of_securified_entity(
                &AnySecurifiedEntity::new(entity.into()).unwrap(),
                &rola_key,
            )
        );
    }

    #[actix_rt::test]
    async fn top_up_contributes_to_the_entitys_access_controller() {
        let entity = SecurifiedAccount::sample().entity;
        let os = os_with(entity.clone()).await;

        let manifest = os
            .make_top_up_access_controller_xrd_vault_manifest(
                entity.address.into(),
                Account::sample().address,
                Decimal192::ten(),
            )
            .await
            .unwrap();

        assert_eq!(
            manifest,
            TransactionManifest::top_up_access_controller_xrd_vault(
                SecurifiedAccount::sample().access_controller_address(),
                &Account::sample(),
                Decimal192::ten(),
            )
        );
    }

    #[actix_rt::test]
    async fn unsecurified_entities_have_no_access_controller() {
        let entity = Account::sample_other();
        let os = os_with(entity.clone()).await;

        assert!(os
            .make_set_rola_key_manifest(entity.address.into())
            .await
            .is_err());
        assert!(os
            .make_top_up_access_controller_xrd_vault_manifest(
                entity.address.into(),
                Account::sample().address,
                Decimal192::ten(),
            )
            .await
            .is_err());
    }
}
