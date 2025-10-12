use crate::prelude::*;
use profile_supporting_types::UnsecurifiedAccount;
use radix_engine::vm::*;
use radix_engine::{
    object_modules::role_assignment::*, system::system_db_reader::*,
};
use radix_engine_interface::prelude::*;
use scrypto_test::prelude::{
    v2::AccessControllerV2Substate, FieldContentSource, FieldPayload,
    SubstateDatabaseExtensions,
};

#[cfg(test)]
#[extend::ext]
pub impl<E, D> LedgerSimulator<E, D>
where
    E: NativeVmExtension,
    D: TestDatabase,
{
    // Reads the on Ledger AC substate
    fn read_access_controller_substate(
        &mut self,
        address: AccessControllerAddress,
    ) -> AccessControllerV2StateV2 {
        let payload: AccessControllerV2StateFieldPayload =
            self.component_state(address.scrypto().try_into().unwrap());

        let payload_content = payload.into_content();
        let versioned = payload_content.into_versions();
        match versioned {
            AccessControllerV2StateVersions::V1(_) => panic!("unsupported"), // From<V1> is implemented
            AccessControllerV2StateVersions::V2(newer) => newer,
        }
    }

    // Reads the on ledger AC configured rule set
    fn read_access_controller_rule_set(
        &mut self,
        address: AccessControllerAddress,
    ) -> ScryptoRuleSet {
        ScryptoRuleSet {
            primary_role: self
                .read_access_controller_access_rule(address.clone(), "primary"),
            recovery_role: self.read_access_controller_access_rule(
                address.clone(),
                "recovery",
            ),
            confirmation_role: self.read_access_controller_access_rule(
                address.clone(),
                "confirmation",
            ),
        }
    }

    // Reads the on ledger AC configured access rule for the given role
    fn read_access_controller_access_rule(
        &mut self,
        address: AccessControllerAddress,
        role: &str,
    ) -> ScryptoAccessRule {
        let db_reader = SystemDatabaseReader::new(self.substate_db());
        db_reader
            .read_object_collection_entry::<_, RoleAssignmentAccessRuleEntryPayload>(
                address.scrypto().as_node_id(),
                ModuleId::RoleAssignment,
                ObjectCollectionKey::KeyValue(
                    RoleAssignmentCollection::AccessRuleKeyValue.collection_index(),
                    &ModuleRoleKey::new(ModuleId::Main, role),
                ),
            )
            .ok()
            .flatten()
            .map(|payload| payload.fully_update_and_into_latest_version())
            .expect("Rule should be configured")
    }

    // Create an Access Controller on Ledger for the given account with the given security structure of factor instances
    fn create_access_controller(
        &mut self,
        unsecurified_account: UnsecurifiedAccount,
        security_structure: SecurityStructureOfFactorInstances,
    ) -> AccessControllerAddress {
        let account_address =
            unsecurified_account.address().into_account().unwrap();
        // Preload account with free XRDz to use for fee payment.
        self.load_account_from_faucet(account_address.into());

        let mut securify_account_manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                unsecurified_account.clone().into(),
                security_structure,
            )
            .unwrap();

        securify_account_manifest = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account(
            unsecurified_account.entity.clone(),
            unsecurified_account.clone().into(),
            securify_account_manifest,
            Some(Decimal192::ten())
        );

        securify_account_manifest = securify_account_manifest
            .modify_add_lock_fee(LockFeeData::new_with_unsecurified_fee_payer(
                account_address,
                Decimal192::two(),
            ))
            .unwrap();

        let notarized_tx = self.sign_and_notarize_transaction(
            securify_account_manifest,
            |intent| {
                let signature = unsecurified_account
                    .unsecured_entity_control
                    .transaction_signing
                    .factor_instance()
                    .sign_hash(&intent.transaction_intent_hash().hash);
                SignedIntent::with_signatures(intent, vec![signature.into()])
                    .unwrap()
            },
        );

        let scrypto_ac_address = self
            .execute_notarized_transaction(notarized_tx)
            .expect_commit_success()
            .new_component_addresses()
            .first()
            .copied()
            .unwrap();
        (scrypto_ac_address, NetworkID::Simulator).into()
    }

    // Executes the given AC recovery manifest and asserts that the transaction was successfully committed.
    fn execute_ac_recovery_manifest(
        &mut self,
        manifest: TransactionManifest,
        security_control: SecuredEntityControl,
        sign_with_roles: RolesExercisableInTransactionManifestCombination,
    ) {
        let notarized_tx =
            self.sign_and_notarize_transaction(manifest, |intent| {
                security_control.sign_intent_with_roles(intent, sign_with_roles)
            });

        self.execute_notarized_transaction(notarized_tx)
            .expect_commit_success();
    }

    fn sign_and_notarize_transaction<S>(
        &mut self,
        manifest: TransactionManifest,
        sign_intent: S,
    ) -> ScryptoNotarizedTransaction
    where
        S: FnOnce(TransactionIntent) -> SignedIntent,
    {
        let notary_private_key: Secp256k1PrivateKey =
            Secp256k1PrivateKey::sample();
        let notary_public_key: Secp256k1PublicKey =
            notary_private_key.public_key();

        let current_epoch = self.get_current_epoch();
        let tx_header = TransactionHeader::new(
            NetworkID::Simulator,
            current_epoch,
            current_epoch.after(10).unwrap(),
            10,
            notary_public_key,
            true,
            0,
        );
        let intent =
            TransactionIntent::new(tx_header, manifest, Message::None).unwrap();
        let signed_intent = sign_intent(intent);

        let notary_signature =
            notary_private_key.notarize_hash(&signed_intent.hash());
        let notarized_transaction =
            NotarizedTransaction::new(signed_intent, notary_signature).unwrap();

        ScryptoNotarizedTransaction::from(notarized_transaction)
    }
}

#[cfg(test)]
#[extend::ext]
impl SecuredEntityControl {
    fn sign_intent_with_roles(
        &self,
        intent: TransactionIntent,
        exercised_roles: RolesExercisableInTransactionManifestCombination,
    ) -> SignedIntent {
        let hash = intent.transaction_intent_hash().hash;

        let signatures = match exercised_roles {
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion => vec![self.sign_with_first_recovery_role_factor(&hash)],
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary => vec![
                vec![self.sign_with_first_recovery_role_factor(&hash)],
                self.sign_with_primary_role_factors(&hash)
            ].into_iter().flatten().collect(),
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation => vec![
                self.sign_with_first_recovery_role_factor(&hash),
                self.sign_with_first_confirmation_role_factor(&hash)
            ],
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation => self.sign_with_primary_role_factors(&hash),
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion => panic!("Unsupported"),
        };
        let intent_signatures: Vec<IntentSignature> =
            signatures.into_iter().map(Into::into).collect();

        SignedIntent::with_signatures(intent, intent_signatures)
            .expect("Should succeed building the signed intent")
    }
}
