use crate::prelude::*;

/// Implementation of complex signing flow laid out in this
/// [whimsical diagram][flow].
///
/// [flow]: https://whimsical.com/wallet-sargon-signing-flow-QFvU2NAVXFiX1VgNBuvj5g
pub struct SigningManager {
    /// FactorSources in Profile
    factor_sources_in_profile: IndexSet<FactorSource>,
    interactor: Arc<dyn SignInteractor<TransactionIntent>>,

    /// The internal state of the SigningManager
    ///
    /// We start with `None` in ctor, and set it to `Some` in `sign_intent_sets`.
    /// We wanna init this SigninManager only with dependencies and not until
    /// later when we call `sign_intent_sets` we can set the state.
    state: RwLock<Option<SigningManagerState>>,
}

struct SigningManagerState {
    intent_sets: Vec<SecurityShieldApplicationWithTransactionIntents>,
}

impl SigningManagerState {
    fn new(
        intent_sets: impl IntoIterator<
            Item = SecurityShieldApplicationWithTransactionIntents,
        >,
    ) -> Self {
        Self {
            intent_sets: intent_sets.into_iter().collect(),
        }
    }
}

enum ExerciseOutcomeKind {
    AllEntitiesSignedFor,
    NotAllEntitiesSignedFor,
}

// ==============
// === PUBLIC ===
// ==============
impl SigningManager {
    pub fn new(
        factor_sources_in_profile: IndexSet<FactorSource>,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    ) -> Self {
        Self {
            factor_sources_in_profile,
            interactor,
            state: RwLock::new(None),
        }
    }

    /// A "TransactionIntent Set" is a "group" of TransactionsIntents having manifest per variant
    /// of [`RolesExercisableInTransactionManifestCombination`]. For manifests
    /// securifying an unsecurified entity the set will have only one intent.
    ///
    /// From each set we should only submit one to the Ledger, and that is the
    /// "best one" of those which was signed. Successfully signed intent which
    /// can exercise the Confirmation role are better than those using delay completion (
    /// time).
    ///
    /// We are performing 4 passes to the SignaturesCollector, first
    /// using Recovery role, then Confirmation role, then Primary role for
    /// the entities applying the shield, and lastly we sign for the fee payers
    /// using Primary role.
    pub async fn sign_intent_sets(
        &self,
        intent_sets: impl IntoIterator<
            Item = SecurityShieldApplicationWithTransactionIntents,
        >,
    ) -> Result<SigningManagerOutcome> {
        // Init the state
        self.state
            .write()
            .unwrap()
            .replace(SigningManagerState::new(intent_sets));

        // Start with Recovery role
        self.sign_intents_with_recovery_role().await?;

        // Then we sign for the Confirmation role
        self.sign_intents_with_confirmation_role().await?;

        // Then we sign for the Primary role
        self.sign_intents_with_primary_role().await?;

        // Lastly we sign for the fee payers using Primary role
        self.sign_for_fee_payers().await?;

        // Try to get the outcome
        self.outcome()
    }
}

// ===============
// === PRIVATE ===
// ===============
impl SigningManager {
    fn outcome(&self) -> Result<SigningManagerOutcome> {
        let mut state = self.state.write().map_err(|_| CommonError::Unknown)?; // TODO specific error variant
        let _state = state.take().ok_or(CommonError::Unknown)?; // TODO specific error variant
        todo!()
    }

    /// # Panics
    /// Panics if recovery_outcome.role != RoleKind::Recovery
    fn handle_recovery_outcome(
        &self,
        recovery_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(recovery_outcome.role, RoleKind::Recovery);
        self.updating_state(|_s| Err(CommonError::Unknown))?;
        Ok(())
    }

    /// # Panics
    /// Panics if recovery_outcome.role != RoleKind::Confirmation
    fn handle_confirmation_outcome(
        &self,
        confirmation_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(confirmation_outcome.role, RoleKind::Confirmation);
        self.updating_state(|_s| Err(CommonError::Unknown))?;
        Ok(())
    }

    /// # Panics
    /// Panics if recovery_outcome.role != RoleKind::Primary
    fn handle_primary_outcome(
        &self,
        primary_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(primary_outcome.role, RoleKind::Primary);
        self.updating_state(|_s| Err(CommonError::Unknown))?;
        Ok(())
    }

    /// # Panics
    /// Panics if fee_payers_outcome.role != RoleKind::Primary (we are spending XRD)
    fn handle_fee_payers_outcome(
        &self,
        fee_payers_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(fee_payers_outcome.role, RoleKind::Primary);
        self.updating_state(|_s| Err(CommonError::Unknown))?;
        Ok(())
    }

    /// # Throws
    /// An error thrown means abort the whole process.
    async fn sign_intent_sets_with_role(
        &self,
        intent_sets: Vec<IntentSetToSign>,
        role: RoleKind,
    ) -> Result<ExerciseRoleOutcome> {
        let purpose = SigningPurpose::SignTX { role_kind: role };

        let mut lookup_address_to_entity =
            HashMap::<AddressOfAccountOrPersona, AccountOrPersona>::new();
        let mut lookup_txid_to_intent_set =
            HashMap::<TransactionIntentHash, IntentSetID>::new();
        let mut lookup_txid_to_variant = HashMap::<
            TransactionIntentHash,
            Option<RolesExercisableInTransactionManifestCombination>,
        >::new();

        let transactions_with_petitions = intent_sets
            .into_iter()
            .flat_map(|set| {
                set.variants
                    .iter()
                    .map(|variant| {
                        let tx = variant.intent.clone();
                        let txid = tx.transaction_intent_hash();
                        // Insert TXID into the lookup so we can group the signatures
                        // of each intent by IntentSetID.
                        lookup_txid_to_intent_set.insert(txid.clone(), set.id);

                        lookup_txid_to_variant
                            .insert(txid.clone(), variant.variant);

                        let entity_requiring_auth = set.entity.clone();
                        lookup_address_to_entity.insert(
                            entity_requiring_auth.address(),
                            entity_requiring_auth.clone(),
                        );

                        SignableWithEntities::new(tx, [entity_requiring_auth])
                    })
                    .collect_vec()
            })
            .collect::<IdentifiedVecOf<_>>();

        let collector = SignaturesCollector::with(
            SigningFinishEarlyStrategy::default(),
            self.factor_sources_in_profile.clone(),
            transactions_with_petitions,
            self.interactor.clone(),
            purpose,
        );

        // Failure is not something we handle, it means the whole process should
        // be aborted by user
        let outcome = collector.collect_signatures().await?;

        // TODO: Split `outcome` into `entities_signed_for` and `entities_not_signed_for`

        let entities_signed_for: Vec<IntentWithSignatures> = outcome
            .successful_transactions()
            .into_iter()
            .map(|signed_tx| {
                let txid = signed_tx.signable_id;
                let signatures_with_inputs = signed_tx.signatures;
                assert!(!signatures_with_inputs.is_empty(), "cannot be empty");
                let owner_address = signatures_with_inputs
                    .first()
                    .unwrap()
                    .owned_factor_instance()
                    .owner;
                assert!(
                    signatures_with_inputs
                        .iter()
                        .all(|s| s.owned_factor_instance().owner
                            == owner_address),
                    "SigningManager expects"
                );

                let entity = lookup_address_to_entity
                    .get(&owner_address)
                    .unwrap()
                    .clone();

                let intent_set_id =
                    *lookup_txid_to_intent_set.get(&txid).unwrap();

                let manifest_variant =
                    *lookup_txid_to_variant.get(&txid).unwrap();

                IntentWithSignatures::new(
                    intent_set_id,
                    txid,
                    entity,
                    signatures_with_inputs
                        .into_iter()
                        .map(|s| s.signature)
                        .collect(),
                    manifest_variant,
                )
            })
            .collect_vec();

        let entities_not_signed_for: Vec<EntityNotSignedFor> = vec![];

        Ok(ExerciseRoleOutcome::new(
            role,
            entities_signed_for,
            entities_not_signed_for,
        ))
    }

    fn updating_state(
        &self,
        f: impl FnOnce(&mut SigningManagerState) -> Result<()>,
    ) -> Result<()> {
        let mut state_holder =
            self.state.write().map_err(|_| CommonError::Unknown)?; // TODO specific error variant
        if let Some(state) = state_holder.as_mut() {
            f(state)
        } else {
            unreachable!("State should be Some");
        }
    }

    async fn sign_intents_with_recovery_role(&self) -> Result<()> {
        let intent_sets: Vec<IntentSetToSign> = vec![]; // TODO: Get intent_sets from state
        let outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Recovery)
            .await?;
        self.handle_recovery_outcome(outcome)
    }

    async fn sign_intents_with_confirmation_role(&self) -> Result<()> {
        let intent_sets: Vec<IntentSetToSign> = vec![]; // TODO: Get intent_sets from state
        let outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Confirmation)
            .await?;
        self.handle_confirmation_outcome(outcome)
    }

    async fn sign_intents_with_primary_role(&self) -> Result<()> {
        let intent_sets: Vec<IntentSetToSign> = vec![]; // TODO: Get intent_sets from state
        let outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Primary)
            .await?;
        self.handle_primary_outcome(outcome)
    }

    async fn sign_for_fee_payers(&self) -> Result<()> {
        let intent_sets: Vec<IntentSetToSign> = vec![]; // TODO: Get intent_sets from state

        // We are goign to spend the fee paying accouts XRD
        // so we use Primary role
        let role = RoleKind::Primary;

        let outcome =
            self.sign_intent_sets_with_role(intent_sets, role).await?;

        self.handle_fee_payers_outcome(outcome)
    }
}

// ==================
// ==== TO SIGN =====
// ==================

/// An ID generated for the purpose of being able to identify which "set" a
/// TransactionIntent belongs to.
#[derive(Clone, Copy, PartialEq, Eq, derive_more::Debug)]
pub struct IntentSetID(Uuid);
impl Default for IntentSetID {
    fn default() -> Self {
        Self::new()
    }
}

impl IntentSetID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// A "set" of TransactionIntents to sign, and entities to sign for.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
struct IntentSetToSign {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    hidden: HiddenConstructor,

    // An ID generated for the purpose of being able to identify which "set" a
    // TransactionIntent belongs to.
    id: IntentSetID,

    /// Will be a single one for unsecurified entities
    variants: Vec<IntentVariant>,

    /// For shield applying manifests this Vec contains a single entity, either
    /// the entity applying the shield or the fee payer
    entity: AccountOrPersona, // TODO: Generalization - in future change to support multiple entities
}
impl IntentSetToSign {
    pub fn new(variants: Vec<IntentVariant>, entity: AccountOrPersona) -> Self {
        Self {
            hidden: HiddenConstructor,
            id: IntentSetID::new(),
            variants,
            entity,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct IntentVariant {
    variant: Option<RolesExercisableInTransactionManifestCombination>,
    intent: TransactionIntent,
}

// =================
// ==== SIGNED =====
// =================

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub(crate) struct IntentWithSignatures {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    hidden: HiddenConstructor,

    transaction_intent_hash: TransactionIntentHash,

    part_of_intent_set: IntentSetID,

    /// Must match the owner inside `signatures.map(|s| s.input.owned_factor_instance.owner`
    entity: AccountOrPersona,

    signatures: IndexSet<SignatureWithPublicKey>,

    /// None `intent` is the single intent of an IntentSet. If the intent
    /// is one of the many variants of an intentset then this variable must
    /// be `Some`.
    variant: Option<RolesExercisableInTransactionManifestCombination>,
}
impl IntentWithSignatures {
    pub(crate) fn new(
        part_of_intent_set: IntentSetID,
        transaction_intent_hash: TransactionIntentHash,
        entity: AccountOrPersona,
        signatures: IndexSet<SignatureWithPublicKey>,
        variant: Option<RolesExercisableInTransactionManifestCombination>,
    ) -> Self {
        Self {
            hidden: HiddenConstructor,
            part_of_intent_set,
            transaction_intent_hash,
            entity,
            signatures,
            variant,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EntityNotSignedFor {
    intent: TransactionIntent,
    entity: AccountOrPersona,
    variant: Option<RolesExercisableInTransactionManifestCombination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExerciseRoleOutcome {
    #[allow(dead_code)]
    #[doc(hidden)]
    hidden: HiddenConstructor,

    role: RoleKind,

    /// The `entities_signed_for.filter_map(|e| e.variant)` must "contain" `role`, e.g.
    /// if role is ROLE_PRIMARY_ROLE then variant cannot be
    /// RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation
    /// which does not "contain" Primary.
    entities_signed_for: Vec<IntentWithSignatures>, // want IndexSet, but Item is not StdHash.

    entities_not_signed_for: Vec<EntityNotSignedFor>, // want IndexSet, but Item is not StdHash.
}

impl ExerciseRoleOutcome {
    fn kind(&self) -> ExerciseOutcomeKind {
        if self.entities_not_signed_for.is_empty() {
            ExerciseOutcomeKind::AllEntitiesSignedFor
        } else {
            ExerciseOutcomeKind::NotAllEntitiesSignedFor
        }
    }

    /// # Panics
    /// Panics if there is a discrepancy between the entities_signed_for variant and `role_kind``.
    pub fn new(
        role_kind: RoleKind,
        entities_signed_for: Vec<IntentWithSignatures>,
        entities_not_signed_for: Vec<EntityNotSignedFor>,
    ) -> Self {
        assert!(
            entities_signed_for
                .iter()
                .filter_map(|e| e.variant)
                .all(|v| v.can_exercise_role(role_kind)),
            "Discrepancy! Mismatch beween Role and TransactionManifest variant"
        );

        assert!(
            entities_not_signed_for
                .iter()
                .filter_map(|e| e.variant)
                .all(|v| v.can_exercise_role(role_kind)),
            "Discrepancy! Mismatch beween Role and TransactionManifest variant"
        );

        assert!(
            entities_signed_for
            .iter()
            .map(|e| e.entity.address())
            .collect::<HashSet<_>>()
            .intersection(
                &entities_not_signed_for
                .iter()
                .map(|e| e.entity.address())
                .collect::<HashSet<_>>()
            ).collect_vec().is_empty(),
            "Discrepancy! entities_signed_for and entities_not_signed_for have common entities"
        );

        Self {
            hidden: HiddenConstructor,
            role: role_kind,
            entities_signed_for,
            entities_not_signed_for,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedIntentSet {
    intents: Vec<IntentWithSignatures>, // Want IndexSet but TransactionIntent is not `std::hash::Hash`
}
impl SignedIntentSet {
    pub fn get_best_signed_intent(&self) -> SignedIntent {
        todo!()
    }
}

pub struct SigningManagerOutcome {
    successfully_signed_intent_sets: Vec<SignedIntentSet>,
    failed_intent_sets: Vec<SignedIntentSet>,
}
impl SigningManagerOutcome {
    // TODO: Implement support for handling of failed transactions, i.e. submit the successful ones even if some failed and do SOMETHING with the failed ones
    pub fn validate_all_intent_sets_signed(
        self,
    ) -> Result<Vec<SignedIntentSet>> {
        if self.failed_intent_sets.is_empty() {
            Ok(self.successfully_signed_intent_sets)
        } else {
            Err(CommonError::Unknown) // TODO specific error variant
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type SUT = SigningManager;

    #[actix_rt::test]
    async fn test() {
        // let sut = SUT::new(profile, interactor)
    }
}
