use crate::prelude::*;

pub trait SargonOsTransactionManifestModify {
    fn modify_transaction_manifest_with_fee_payer<G>(
        &self,
        manifest: TransactionManifest,
        fee_payer_address: AccountAddress,
        fee: Decimal192,
        guarantees: G,
    ) -> Result<TransactionManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>;

    fn modify_transaction_manifest_without_fee_payer<G>(
        &self,
        manifest: TransactionManifest,
        guarantees: G,
    ) -> Result<TransactionManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>;
}

impl SargonOsTransactionManifestModify for SargonOS {
    fn modify_transaction_manifest_with_fee_payer<G>(
        &self,
        manifest: TransactionManifest,
        fee_payer_address: AccountAddress,
        fee: Decimal192,
        guarantees: G,
    ) -> Result<TransactionManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>,
    {
        let summary = manifest.summary()?;
        let proofs_for_entities_requiring_auth =
            self.extract_proofs(&summary)?;

        // Resolve lock fee data
        let account_fee_payer = self.account_by_address(fee_payer_address)?;
        let (lock_fee_data, guarantees_instructions_offset) =
            if let Some(control) =
                account_fee_payer.security_state().as_securified()
            {
                let fee_payer_entity_address =
                    AddressOfAccountOrPersona::from(fee_payer_address);
                let offset = if proofs_for_entities_requiring_auth
                    .contains_key(&fee_payer_entity_address)
                {
                    // The difference of the summarised manifest and the modified one is just
                    // one `lock_fee` instruction. If the fee payer is in the original set of entities
                    // requiring proof then the `create_proof` instruction is already taken into account.
                    1
                } else {
                    // The difference of the summarised manifest and the modified one is the
                    // `create_proof` & `lock_fee` instructions. Since the fee payer is not included
                    // in the original securified set of entities requiring auth, then these 2 instructions
                    // are added to the modified manifest.
                    2
                };

                // Fee payer is securified, access controller proof needs to be prepended
                // before lock fee.
                (
                    LockFeeData::new_with_securified_fee_payer(
                        fee_payer_address,
                        control.access_controller_address,
                        fee,
                    ),
                    offset,
                )
            } else {
                // Fee payer is unsecure, just lock fee instruction is added,
                // thus guarantee instructions need to be offset by 1
                (
                    LockFeeData::new_with_unsecurified_fee_payer(
                        fee_payer_address,
                        fee,
                    ),
                    1,
                )
            };

        // Then add the `lock_fee` instruction with potential `create_proof`s
        let modified_manifest = manifest.modify_add_lock_fee_and_proofs(
            lock_fee_data,
            proofs_for_entities_requiring_auth,
        )?;

        // Lastly add the guarantees with indices offset by `offset`
        modified_manifest.modify_add_guarantees(guarantees.into_iter().map(
            |g| g.offset_instruction_index_by(guarantees_instructions_offset),
        ))
    }

    fn modify_transaction_manifest_without_fee_payer<G>(
        &self,
        manifest: TransactionManifest,
        guarantees: G,
    ) -> Result<TransactionManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>,
    {
        let summary = manifest.summary()?;
        let proofs_for_entities_requiring_auth =
            self.extract_proofs(&summary)?;

        // Add the potential `create_proof`s
        let modified_manifest = manifest.modify_add_lock_fee_and_proofs(
            None,
            proofs_for_entities_requiring_auth,
        )?;

        // Then add the guarantees
        modified_manifest.modify_add_guarantees(guarantees)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::single::support::prepare_xrd_transfer_transaction;

    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_involved_entities_not_included_in_profile() {
        let account = Account::sample_mainnet();

        let os = prepare_os([], []);
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();

        let result = os.await.modify_transaction_manifest_with_fee_payer(
            manifest,
            account.address(),
            Decimal192::five(),
            [],
        );

        assert_eq!(Err(CommonError::UnknownAccount), result);
    }

    #[actix_rt::test]
    async fn test_fee_payer_resolved_as_unsecurified() {
        let account = Account::sample_mainnet();

        let os = prepare_os([account.clone()], []);
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();

        let modified = os
            .await
            .modify_transaction_manifest_with_fee_payer(
                manifest,
                account.address(),
                Decimal192::five(),
                [],
            )
            .unwrap();

        manifest_eq(
            modified,
            r#"
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        )
    }

    #[actix_rt::test]
    async fn test_fee_payer_resolved_as_securified_which_also_requires_proof_verify_proof_added_once(
    ) {
        let fee_payer = Account::sample_at(2);
        let acc_requires_auth = Account::sample_at(3);
        let per_requires_auth = Persona::sample_at(2);

        let os = prepare_os(
            [fee_payer.clone(), acc_requires_auth.clone()],
            [per_requires_auth.clone()],
        );
        let manifest = prepare_manifest(
            [fee_payer.address.clone(), acc_requires_auth.address.clone()],
            [per_requires_auth.address.clone()],
        );

        let modified = os
            .await
            .modify_transaction_manifest_with_fee_payer(
                manifest,
                fee_payer.address(),
                Decimal192::five(),
                [],
            )
            .unwrap();

        println!("{}", modified.instructions_string());

        manifest_eq(
            modified,
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx12xq83qxwf09eqquqfdj8rv004attq5tgxejkn59rwu588jprdy0xcg")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcthvtzcny04t5dnuc0wq5n8hx90eytn6luhmfm5g52rm6wvsnfk")
                "create_proof"
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cdf8qgfmz0fgxap9u5haf4ta2lstc04rcfrljgdnznrpugkqv2wudm")
                "create_proof"
            ;
            SET_METADATA
                Address("account_rdx12xq83qxwf09eqquqfdj8rv004attq5tgxejkn59rwu588jprdy0xcg")
                "owner_keys"
                Enum<143u8>(
                    Array<Enum>(
                        Enum<1u8>(
                            Bytes("f4e18c034e069baee91ada4764fdfcf2438b8f976861df00557d4cc9e7")
                        )
                    )
                )
            ;
            SET_METADATA
                Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
                "owner_keys"
                Enum<143u8>(
                    Array<Enum>(
                        Enum<1u8>(
                            Bytes("f4e18c034e069baee91ada4764fdfcf2438b8f976861df00557d4cc9e7")
                        )
                    )
                )
            ;
            SET_METADATA
                Address("identity_rdx12fczzwcn62phgf099l2d2l2huz7rag7zglujrvc5cc0z9szs2dzevj")
                "owner_keys"
                Enum<143u8>(
                    Array<Enum>(
                        Enum<1u8>(
                            Bytes("f4e18c034e069baee91ada4764fdfcf2438b8f976861df00557d4cc9e7")
                        )
                    )
                )
            ;
            "#,
        )
    }

    #[actix_rt::test]
    async fn test_fee_payer_unsecurified_involved_in_transaction_guarantees_offset_by_one(
    ) {
        let fee_payer = Account::sample_at(0);
        // This account requires auth due to deposit instruction
        let depositing_account = Account::sample_at(3);

        let manifest = prepare_xrd_transfer_transaction(
            fee_payer.address(),
            depositing_account.address(),
        );
        let os =
            prepare_os([fee_payer.clone(), depositing_account.clone()], [])
                .await;

        let modified = os
            .modify_transaction_manifest_with_fee_payer(
                manifest,
                fee_payer.address(),
                Decimal192::five(),
                [guarantee(2)],
            )
            .unwrap();

        println!("{}", modified.instructions_string());

        // The modified manifest should shift the guarantee instruction by 1
        manifest_eq(
            modified,
            r#"
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcthvtzcny04t5dnuc0wq5n8hx90eytn6luhmfm5g52rm6wvsnfk")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
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
                Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
                "deposit"
                Bucket("bucket1")
            ;
            "#,
        )
    }

    #[actix_rt::test]
    async fn test_fee_payer_securified_involved_in_transaction_guarantees_offset_by_one(
    ) {
        let fee_payer = Account::sample_at(2);
        // This account requires auth due to deposit instruction
        let depositing_account = Account::sample_at(3);

        let manifest = prepare_xrd_transfer_transaction(
            fee_payer.address(),
            depositing_account.address(),
        );
        let os =
            prepare_os([fee_payer.clone(), depositing_account.clone()], [])
                .await;

        let modified = os
            .modify_transaction_manifest_with_fee_payer(
                manifest,
                fee_payer.address(),
                Decimal192::five(),
                [guarantee(3)],
            )
            .unwrap();

        // The modified manifest should shift the guarantee instruction by 1
        manifest_eq(
            modified,
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx12xq83qxwf09eqquqfdj8rv004attq5tgxejkn59rwu588jprdy0xcg")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcthvtzcny04t5dnuc0wq5n8hx90eytn6luhmfm5g52rm6wvsnfk")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx12xq83qxwf09eqquqfdj8rv004attq5tgxejkn59rwu588jprdy0xcg")
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
                Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
                "deposit"
                Bucket("bucket1")
            ;
            "#,
        )
    }

    #[actix_rt::test]
    async fn test_fee_payer_securified_not_involved_in_transaction_guarantees_offset_by_two(
    ) {
        let fee_payer = Account::sample_at(4);
        let withdrawing_account = Account::sample_at(2);
        // This account requires auth due to deposit instruction
        let depositing_account = Account::sample_at(3);

        let manifest = prepare_xrd_transfer_transaction(
            withdrawing_account.address(),
            depositing_account.address(),
        );
        let os = prepare_os(
            [
                fee_payer.clone(),
                withdrawing_account.clone(),
                depositing_account.clone(),
            ],
            [],
        )
        .await;

        let modified = os
            .modify_transaction_manifest_with_fee_payer(
                manifest,
                fee_payer.address(),
                Decimal192::five(),
                [guarantee(3)],
            )
            .unwrap();

        // The modified manifest should shift the guarantee instruction by 2
        manifest_eq(
            modified,
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1cdgewxe4wmg69544jrh956srstgvm05z7yzfnasxmnfd938uywgmgd")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx12xt3kdtk6x3d9dvsaedx5quz6rxmaqh3qjvlvpku6tfvflpj0nz4qq")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g")
                "create_proof"
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcthvtzcny04t5dnuc0wq5n8hx90eytn6luhmfm5g52rm6wvsnfk")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx12xq83qxwf09eqquqfdj8rv004attq5tgxejkn59rwu588jprdy0xcg")
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
                Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
                "deposit"
                Bucket("bucket1")
            ;
            "#,
        )
    }

    #[actix_rt::test]
    async fn test_modify_without_fee_payer() {
        let withdrawing_account = Account::sample_at(2);
        // This account requires auth due to deposit instruction
        let depositing_account = Account::sample_at(3);

        let manifest = prepare_xrd_transfer_transaction(
            withdrawing_account.address(),
            depositing_account.address(),
        );
        let os = prepare_os(
            [withdrawing_account.clone(), depositing_account.clone()],
            [],
        )
        .await;

        let modified = os
            .modify_transaction_manifest_without_fee_payer(
                manifest,
                [guarantee(3)],
            )
            .unwrap();

        // The modified manifest should shift the guarantee instruction by 1
        manifest_eq(
            modified,
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g")
                "create_proof"
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcthvtzcny04t5dnuc0wq5n8hx90eytn6luhmfm5g52rm6wvsnfk")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx12xq83qxwf09eqquqfdj8rv004attq5tgxejkn59rwu588jprdy0xcg")
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
                Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
                "deposit"
                Bucket("bucket1")
            ;
            "#,
        )
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

    fn prepare_manifest(
        account_addresses_req_auth: impl IntoIterator<Item = AccountAddress>,
        identity_addresses_req_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> TransactionManifest {
        let mut manifest_builder = ScryptoTransactionManifestBuilder::new();

        for address in account_addresses_req_auth {
            manifest_builder = manifest_builder.set_metadata(
                address.scrypto(),
                MetadataKey::OwnerKeys,
                ScryptoMetadataValue::PublicKeyHashArray(vec![
                    PublicKeyHash::sample().into(),
                ]),
            )
        }

        for address in identity_addresses_req_auth {
            manifest_builder = manifest_builder.set_metadata(
                address.scrypto(),
                MetadataKey::OwnerKeys,
                ScryptoMetadataValue::PublicKeyHashArray(vec![
                    PublicKeyHash::sample().into(),
                ]),
            )
        }

        TransactionManifest::sargon_built(manifest_builder, NetworkID::Mainnet)
    }

    async fn prepare_os(
        accounts: impl IntoIterator<Item = Account>,
        personas: impl IntoIterator<Item = Persona>,
    ) -> Arc<SUT> {
        let req = SargonOS::boot_test();
        let os =
            actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
                .await
                .unwrap()
                .unwrap();

        let accounts = accounts.into_iter().collect_vec();
        let personas = personas.into_iter().collect_vec();

        os.update_profile_with(|profile| {
            profile.networks.insert(ProfileNetwork::new(
                NetworkID::Mainnet,
                accounts.clone(),
                personas.clone(),
                AuthorizedDapps::default(),
                ResourcePreferences::default(),
            ));
            profile.factor_sources.extend(FactorSource::sample_all());
            Ok(())
        })
        .await
        .unwrap();
        os
    }
}
