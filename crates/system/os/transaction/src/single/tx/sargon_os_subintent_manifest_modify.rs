use crate::prelude::*;

#[async_trait::async_trait]
pub trait SargonOsSubintentManifestModify {
    async fn modify_subintent_manifest<G>(
        &self,
        manifest: SubintentManifest,
        guarantees: G,
    ) -> Result<SubintentManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee> + Send;
}

#[async_trait::async_trait]
impl SargonOsSubintentManifestModify for SargonOS {
    async fn modify_subintent_manifest<G>(
        &self,
        manifest: SubintentManifest,
        guarantees: G,
    ) -> Result<SubintentManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee> + Send,
    {
        let summary = manifest.summary()?;
        // Includes the access controllers of external accounts (not in Profile),
        // whose signatures are collected via external signing.
        let proofs_for_entities_requiring_auth =
            self.resolve_proofs(&summary).await?;

        // Add the potential `create_proof`s
        let modified_manifest =
            manifest.modify_add_proofs(proofs_for_entities_requiring_auth)?;

        // Lastly add the guarantees
        modified_manifest.modify_add_guarantees(guarantees)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::single::support::{
        prepare_os_with_entities, prepare_xrd_transfer_transaction,
    };

    #[actix_rt::test]
    async fn test_modify_subintent_manifest_adds_guarantees_in_same_index() {
        let acc1 = Account::sample_at(3);
        let acc2 = Account::sample_at(4);

        let os =
            prepare_os_with_entities([acc1.clone(), acc2.clone()], []).await;

        let subintent = prepare_sunbintent_xrd_transfer_transaction(
            acc1.address(),
            acc2.address(),
        );

        subintent_manifest_eq(
            os.modify_subintent_manifest(subintent, [guarantee(4)])
                .await
                .unwrap(),
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcthvtzcny04t5dnuc0wq5n8hx90eytn6luhmfm5g52rm6wvsnfk")
                "create_proof"
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cdgewxe4wmg69544jrh956srstgvm05z7yzfnasxmnfd938uywgmgd")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
                "lock_fee"
                Decimal("500")
            ;
            CALL_METHOD
                Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("5")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1339")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("5")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12xt3kdtk6x3d9dvsaedx5quz6rxmaqh3qjvlvpku6tfvflpj0nz4qq")
                "deposit"
                Bucket("bucket1")
            ;
            YIELD_TO_PARENT;
            "#,
        )
    }

    #[actix_rt::test]
    async fn test_modify_subintent_manifest_adds_proof_for_external_account_controlled_by_access_controller(
    ) {
        let local_account = Account::sample_mainnet_alice();
        // Not part of the Profile, its signatures are collected via external signing.
        let external_account = Account::sample_at(3).address();
        let access_controller = AccessControllerAddress::sample_mainnet();

        let os = prepare_os_with_entities_and_driver(
            [local_account.clone()],
            MockNetworkingDriver::new_with_bodies(
                200,
                vec![badge_location_response(
                    external_account,
                    Address::from(access_controller),
                )],
            ),
        )
        .await;

        let subintent = prepare_sunbintent_xrd_transfer_transaction(
            external_account,
            local_account.address(),
        );

        let modified = os
            .modify_subintent_manifest(subintent.clone(), [])
            .await
            .unwrap();

        let expected = subintent
            .modify_add_proofs(IndexMap::from_iter([(
                AddressOfAccountOrPersona::from(external_account),
                access_controller,
            )]))
            .unwrap();
        pretty_assertions::assert_eq!(modified, expected);
        assert!(modified.manifest_string().contains(&format!(
            "Address(\"{}\")\n    \"create_proof\"",
            access_controller
        )));
    }

    #[actix_rt::test]
    async fn test_modify_subintent_manifest_does_not_add_proof_for_unsecurified_external_account(
    ) {
        let local_account = Account::sample_mainnet_alice();
        let external_account = Account::sample_at(3).address();

        let os = prepare_os_with_entities_and_driver(
            [local_account.clone()],
            MockNetworkingDriver::new_with_bodies(
                200,
                // The owner badge is held by the account itself, i.e. not securified.
                vec![badge_location_response(
                    external_account,
                    Address::from(external_account),
                )],
            ),
        )
        .await;

        let subintent = prepare_sunbintent_xrd_transfer_transaction(
            external_account,
            local_account.address(),
        );

        let modified = os
            .modify_subintent_manifest(subintent.clone(), [])
            .await
            .unwrap();

        pretty_assertions::assert_eq!(
            modified.manifest_string(),
            subintent.manifest_string()
        );
        assert!(!modified.manifest_string().contains("create_proof"));
    }

    #[actix_rt::test]
    async fn test_modify_subintent_manifest_fails_if_access_controller_of_external_account_cannot_be_resolved(
    ) {
        let local_account = Account::sample_mainnet_alice();
        let external_account = Account::sample_at(3).address();

        let os = prepare_os_with_entities_and_driver(
            [local_account.clone()],
            MockNetworkingDriver::new_always_failing(),
        )
        .await;

        let subintent = prepare_sunbintent_xrd_transfer_transaction(
            external_account,
            local_account.address(),
        );

        let result = os.modify_subintent_manifest(subintent, []).await;

        assert!(result.is_err());
    }

    async fn prepare_os_with_entities_and_driver(
        accounts: impl IntoIterator<Item = Account>,
        networking_driver: MockNetworkingDriver,
    ) -> Arc<SargonOS> {
        let os = actix_rt::time::timeout(
            SARGON_OS_TEST_MAX_ASYNC_DURATION,
            SargonOS::boot_test_with_networking_driver(Arc::new(
                networking_driver,
            )),
        )
        .await
        .unwrap()
        .unwrap();

        let accounts = accounts.into_iter().collect_vec();
        os.update_profile_with(|profile| {
            profile.networks.insert(ProfileNetwork::new(
                NetworkID::Mainnet,
                accounts.clone(),
                [],
                AuthorizedDapps::default(),
                ResourcePreferences::default(),
                MFAFactorInstances::default(),
            ));
            profile.factor_sources.extend(FactorSource::sample_all());
            Ok(())
        })
        .await
        .unwrap();
        os
    }

    fn badge_location_response(
        account: AccountAddress,
        owner: Address,
    ) -> BagOfBytes {
        let body = StateNonFungibleLocationResponse {
            ledger_state: LedgerState::sample_stokenet(),
            // Not inspected by the client, only the items are.
            resource_address: ResourceAddress::sample_mainnet(),
            non_fungible_ids: vec![StateNonFungibleLocationResponseItem {
                non_fungible_id: NonFungibleLocalId::from(account),
                is_burned: false,
                last_updated_at_state_version: 0,
                owning_vault_address: VaultAddress::sample_mainnet(),
                owning_vault_parent_ancestor_address: Some(owner),
                owning_vault_global_ancestor_address: Some(owner),
            }],
        };
        BagOfBytes::from(serde_json::to_vec(&body).unwrap())
    }

    fn guarantee(index: u64) -> TransactionGuarantee {
        TransactionGuarantee::new(
            1339,
            0,
            index,
            ResourceAddress::xrd_on_network(NetworkID::Mainnet),
            18,
        )
    }

    fn prepare_sunbintent_xrd_transfer_transaction(
        from: AccountAddress,
        to: AccountAddress,
    ) -> SubintentManifest {
        let manifest = prepare_xrd_transfer_transaction(from, to);

        let mut instructions_str = manifest.instructions.instructions_string();
        instructions_str.insert_str(
            0,
        r#"
CALL_METHOD
    Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
    "lock_fee"
    Decimal("500")
;
        "#);
        instructions_str.push_str(
            r#"
YIELD_TO_PARENT;
            "#,
        );

        let instructions =
            InstructionsV2::new(instructions_str, NetworkID::Mainnet).unwrap();

        SubintentManifest::with_instructions_and_blobs_and_children(
            instructions,
            Blobs::default(),
            ChildSubintentSpecifiers::default(),
        )
    }
}
