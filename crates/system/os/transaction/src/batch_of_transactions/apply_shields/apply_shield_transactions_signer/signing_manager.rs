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

#[derive(Clone, Debug, PartialEq, Eq)]
struct SigningManagerState {
    per_set_state: IndexMap<IntentSetID, IntentSetState>,
}
impl SigningManagerState {
    fn update_with_exercise_role_outcome(
        &mut self,
        outcome: ExerciseRoleOutcome,
    ) {
        self.update_with_entities_signed_for(outcome.entities_signed_for);

        self.update_with_entities_not_signed_for(
            outcome.entities_not_signed_for,
        );
    }

    fn update_with_entities_signed_for(
        &mut self,
        entities_signed_for: EntitiesSignedFor,
    ) {
        entities_signed_for
            .0
            .into_iter()
            .for_each(|entity_signed_for| {
                self.update_with_intent_with_signatures(entity_signed_for);
            })
    }

    fn update_with_entities_not_signed_for(
        &mut self,
        entities_not_signed_for: EntitiesNotSignedFor,
    ) {
        entities_not_signed_for.0.into_iter().for_each(
            |entity_not_signed_for| {
                self.update_with_entity_not_signed_for(entity_not_signed_for);
            },
        )
    }

    fn update_with_entity_not_signed_for(
        &mut self,
        not_signed: EntityNotSignedFor,
    ) {
        todo!("Neglected factor logic goes here? ")
    }

    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: IntentWithSignatures,
    ) {
        let key = intent_with_signatures.part_of_intent_set;
        let existing = self
            .per_set_state
            .get_mut(&key)
            .expect("Should have created");
        existing.update_with_intent_with_signatures(intent_with_signatures);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct IntentSetState {
    intent_set_id: IntentSetID,
    internal_state: IntentSetInternalState,
}
impl IntentSetState {
    fn new(
        intent_set_id: IntentSetID,
        shield_application: SecurityShieldApplicationWithTransactionIntents,
    ) -> Self {
        Self {
            intent_set_id,
            internal_state: IntentSetInternalState::from(shield_application),
        }
    }

    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: IntentWithSignatures,
    ) {
        assert_eq!(
            self.intent_set_id,
            intent_with_signatures.part_of_intent_set
        );
        self.internal_state
            .update_with_intent_with_signatures(intent_with_signatures);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum IntentSetInternalState {
    Unsecurified(UnsecurifiedIntentSetInternalState),
    Securified(SecurifiedIntentSetInternalState),
}
impl IntentSetInternalState {
    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: IntentWithSignatures,
    ) {
        match self {
            Self::Unsecurified(unsec) => {
                unsec.update_with_intent_with_signatures(intent_with_signatures)
            }
            Self::Securified(sec) => {
                sec.update_with_intent_with_signatures(intent_with_signatures)
            }
        }
    }
}
impl From<SecurityShieldApplicationWithTransactionIntents>
    for IntentSetInternalState
{
    fn from(
        shield_application: SecurityShieldApplicationWithTransactionIntents,
    ) -> Self {
        match shield_application {
            SecurityShieldApplicationWithTransactionIntents::ForSecurifiedEntity(sec) => {
                Self::Securified(SecurifiedIntentSetInternalState::from(sec))
            },
            SecurityShieldApplicationWithTransactionIntents::ForUnsecurifiedEntity(unsec) => {
                Self::Unsecurified(UnsecurifiedIntentSetInternalState::from(unsec))
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct UnsecurifiedIntentSetInternalState {
    account_paying_for_transaction: Immutable<ApplicationInputPayingAccount>,
    entity_applying_shield: Immutable<AnyUnsecurifiedEntity>,
    transaction_intent: Immutable<TransactionIntent>,

    signatures: IntentVariantSignaturesForRoleState,
}
impl UnsecurifiedIntentSetInternalState {
    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: IntentWithSignatures,
    ) {
        assert_eq!(intent_with_signatures.intent, *self.transaction_intent);
        assert_eq!(
            intent_with_signatures.entity.address(),
            self.entity_applying_shield.address()
        );

        self.signatures
            .update_with_intent_with_signatures(intent_with_signatures);
    }
    fn new(
        account_paying_for_transaction: impl Into<
            Immutable<ApplicationInputPayingAccount>,
        >,
        entity_applying_shield: impl Into<Immutable<AnyUnsecurifiedEntity>>,
        transaction_intent: impl Into<Immutable<TransactionIntent>>,
    ) -> Self {
        Self {
            account_paying_for_transaction: account_paying_for_transaction
                .into(),
            entity_applying_shield: entity_applying_shield.into(),
            transaction_intent: transaction_intent.into(),
            // For unsecurified entities we only have Primary role.
            signatures: IntentVariantSignaturesForRoleState::new(
                RoleKind::Primary,
            ),
        }
    }
}
impl From<SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent>
    for UnsecurifiedIntentSetInternalState
{
    fn from(
        application_with_intent: SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent,
    ) -> Self {
        Self::new(
            application_with_intent.paying_account(),
            application_with_intent.entity_applying_shield(),
            application_with_intent.transaction_intent(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct IntentVariantSignaturesPerRoleState(
    IndexMap<RoleKind, IntentVariantSignaturesForRoleState>,
);
impl IntentVariantSignaturesPerRoleState {
    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: IntentWithSignatures,
    ) {
        let state_for_role = self
            .0
            .get_mut(&intent_with_signatures.role_kind)
            .expect("Should have created empty state for each role.");

        state_for_role
            .update_with_intent_with_signatures(intent_with_signatures);
    }

    fn new(variant: RolesExercisableInTransactionManifestCombination) -> Self {
        Self::_new_with_roles(variant.exercisable_roles())
    }
    fn _new_with_roles(roles: impl IntoIterator<Item = RoleKind>) -> Self {
        Self(
            roles
                .into_iter()
                .map(|role| {
                    (role, IntentVariantSignaturesForRoleState::new(role))
                })
                .collect::<IndexMap<_, _>>(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct IntentVariantState {
    intent: Immutable<TransactionIntent>,
    variant: Immutable<RolesExercisableInTransactionManifestCombination>,
    /// The `role` of the values must match the key...
    signatures_per_role: IntentVariantSignaturesPerRoleState,
}

impl IntentVariantState {
    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: IntentWithSignatures,
    ) {
        assert_eq!(intent_with_signatures.intent, *self.intent);
        let variant =
            intent_with_signatures.variant.expect("Should have variant");
        assert_eq!(variant, *self.variant);

        self.signatures_per_role
            .update_with_intent_with_signatures(intent_with_signatures)
    }
    fn new(
        intent: impl Into<Immutable<TransactionIntent>>,
        variant: impl Into<
            Immutable<RolesExercisableInTransactionManifestCombination>,
        >,
    ) -> Self {
        let variant = variant.into();
        let variant_ = *variant;
        Self {
            variant,
            intent: intent.into(),
            signatures_per_role: IntentVariantSignaturesPerRoleState::new(
                variant_,
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct IntentVariantSignaturesForRoleState {
    role: RoleKind,
    signatures_per_entity:
        IndexMap<AddressOfAccountOrPersona, IndexSet<SignatureWithPublicKey>>,
}
impl IntentVariantSignaturesForRoleState {
    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: IntentWithSignatures,
    ) {
        assert_eq!(intent_with_signatures.role_kind, self.role);
        self.signatures_per_entity.append_or_insert_to(
            intent_with_signatures.entity.address(),
            intent_with_signatures.signatures,
        );
    }
    fn new(role: RoleKind) -> Self {
        Self {
            role,
            signatures_per_entity: IndexMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SecurifiedIntentSetInternalState {
    account_paying_for_transaction: Immutable<ApplicationInputPayingAccount>,
    entity_applying_shield: Immutable<AnySecurifiedEntity>,
    initiate_with_recovery_complete_with_primary: IntentVariantState,
    initiate_with_recovery_complete_with_confirmation: IntentVariantState,
    initiate_with_recovery_delayed_completion: IntentVariantState,
    initiate_with_primary_complete_with_confirmation: IntentVariantState,
    initiate_with_primary_delayed_completion: IntentVariantState,
}
impl SecurifiedIntentSetInternalState {
    fn get_variant_state(
        &mut self,
        variant: RolesExercisableInTransactionManifestCombination,
    ) -> &mut IntentVariantState {
        match variant {
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary => {
                &mut self.initiate_with_recovery_complete_with_primary
            },
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation => {
                &mut self.initiate_with_recovery_complete_with_confirmation
            },
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion => {
                &mut self.initiate_with_recovery_delayed_completion
            },
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation => {
                &mut self.initiate_with_primary_complete_with_confirmation
            },
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion => {
                &mut self.initiate_with_primary_delayed_completion
            },
        }
    }

    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: IntentWithSignatures,
    ) {
        assert_eq!(
            intent_with_signatures.entity.address(),
            self.entity_applying_shield.address()
        );
        let variant = intent_with_signatures
            .variant
            .expect("Should have variant for securified");
        let variant_state = self.get_variant_state(variant);
        variant_state
            .update_with_intent_with_signatures(intent_with_signatures);
    }

    fn new(
        account_paying_for_transaction: impl Into<
            Immutable<ApplicationInputPayingAccount>,
        >,
        entity_applying_shield: impl Into<Immutable<AnySecurifiedEntity>>,
        initiate_with_recovery_complete_with_primary: IntentVariantState,
        initiate_with_recovery_complete_with_confirmation: IntentVariantState,
        initiate_with_recovery_delayed_completion: IntentVariantState,
        initiate_with_primary_complete_with_confirmation: IntentVariantState,
        initiate_with_primary_delayed_completion: IntentVariantState,
    ) -> Self {
        Self {
            account_paying_for_transaction: account_paying_for_transaction
                .into(),
            entity_applying_shield: entity_applying_shield.into(),
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            initiate_with_primary_delayed_completion,
        }
    }
}
impl From<SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents>
    for SecurifiedIntentSetInternalState
{
    fn from(
        shield_application: SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents,
    ) -> Self {
        Self::new(
            shield_application.paying_account(),
            shield_application.entity_applying_shield(),
            IntentVariantState::new(
                shield_application.initiate_with_recovery_complete_with_primary(),
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary
            ),
            IntentVariantState::new(
                shield_application.initiate_with_recovery_complete_with_confirmation(),
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation
            ),
            IntentVariantState::new(
                shield_application.initiate_with_recovery_delayed_completion(),
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion
            ),
            IntentVariantState::new(
                shield_application.initiate_with_primary_complete_with_confirmation(),
                RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation
            ),
            IntentVariantState::new(
                shield_application.initiate_with_primary_delayed_completion(),
                RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion
            ),
        )
    }
}

impl SigningManagerState {
    fn new(
        intent_sets: impl IntoIterator<
            Item = SecurityShieldApplicationWithTransactionIntents,
        >,
    ) -> Self {
        Self {
            per_set_state: intent_sets
                .into_iter()
                .map(|shield_application| {
                    let intent_set_id = IntentSetID::new();
                    let value =
                        IntentSetState::new(intent_set_id, shield_application);
                    (intent_set_id, value)
                })
                .collect::<IndexMap<IntentSetID, _>>(),
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
    /// # Throws
    /// An error thrown means abort the whole process.
    async fn sign_intent_sets_with_role(
        &self,
        intent_sets: Vec<IntentSetToSign>,
        role_kind: RoleKind,
    ) -> Result<ExerciseRoleOutcome> {
        let purpose = SigningPurpose::SignTX { role_kind };

        // TODO should probably move these lookup tables into fields of `SigningManager` and
        // change how we construct the SigningManager.
        let mut lookup_address_to_entity =
            HashMap::<AddressOfAccountOrPersona, AccountOrPersona>::new();
        let mut lookup_txid_to_intent_set =
            HashMap::<TransactionIntentHash, IntentSetID>::new();
        let mut lookup_txid_to_variant = HashMap::<
            TransactionIntentHash,
            Option<RolesExercisableInTransactionManifestCombination>,
        >::new();
        let mut lookup_intent_by_txid =
            HashMap::<TransactionIntentHash, TransactionIntent>::new();

        let transactions_with_petitions = intent_sets
            .into_iter()
            .flat_map(|set| {
                set.variants
                    .iter()
                    .map(|variant| {
                        let tx = variant.intent.clone();
                        let txid = tx.transaction_intent_hash();

                        lookup_intent_by_txid.insert(txid.clone(), tx.clone());

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
                    "SigningManager expects a single entity to sign for per role."
                );

                let entity = lookup_address_to_entity
                    .get(&owner_address)
                    .unwrap()
                    .clone();

                let intent_set_id =
                    *lookup_txid_to_intent_set.get(&txid).unwrap();

                let manifest_variant =
                    *lookup_txid_to_variant.get(&txid).unwrap();

                let intent = lookup_intent_by_txid.get(&txid).unwrap().clone();

                IntentWithSignatures::new(
                    intent_set_id,
                    intent,
                    entity,
                    signatures_with_inputs
                        .into_iter()
                        .map(|s| s.signature)
                        .collect(),
                        role_kind,
                    manifest_variant,
                )
            })
            .collect_vec();

        let entities_not_signed_for: Vec<EntityNotSignedFor> = outcome
            .failed_transactions_outcomes()
            .into_iter()
            .map(|o| {
                let txid = o.signable_id;
                let intent = lookup_intent_by_txid.get(&txid).unwrap().clone();

                let per_entity_neglected_factor_sources =
                    o.per_entity_neglected_factors.clone();
                assert_eq!(
                    per_entity_neglected_factor_sources.len(),
                    1,
                    "Should have one entity"
                ); // TODO add support for multiple entities
                let (owner_address, neglected_factors) =
                    per_entity_neglected_factor_sources
                        .into_iter()
                        .next()
                        .expect("Should have one entity");

                let entity = lookup_address_to_entity
                    .get(&owner_address)
                    .unwrap()
                    .clone();

                let manifest_variant =
                    *lookup_txid_to_variant.get(&txid).unwrap();

                EntityNotSignedFor::new(
                    intent,
                    entity,
                    role_kind,
                    neglected_factors,
                    manifest_variant,
                )
            })
            .collect_vec();

        Ok(ExerciseRoleOutcome::new(
            role_kind,
            entities_signed_for,
            entities_not_signed_for,
        ))
    }

    /// # Panics
    /// Panics if recovery_outcome.role != RoleKind::Recovery
    fn handle_recovery_outcome(
        &self,
        recovery_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(recovery_outcome.role, RoleKind::Recovery);
        self.updating_state(|state| {
            state.update_with_exercise_role_outcome(recovery_outcome);
        })?;
        Ok(())
    }

    /// # Panics
    /// Panics if recovery_outcome.role != RoleKind::Confirmation
    fn handle_confirmation_outcome(
        &self,
        confirmation_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(confirmation_outcome.role, RoleKind::Confirmation);
        self.updating_state(|state| {
            state.update_with_exercise_role_outcome(confirmation_outcome);
        })?;
        Ok(())
    }

    /// # Panics
    /// Panics if recovery_outcome.role != RoleKind::Primary
    fn handle_primary_outcome(
        &self,
        primary_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(primary_outcome.role, RoleKind::Primary);
        self.updating_state(|state| {
            state.update_with_exercise_role_outcome(primary_outcome);
        })?;
        Ok(())
    }

    /// # Panics
    /// Panics if fee_payers_outcome.role != RoleKind::Primary (we are spending XRD)
    fn handle_fee_payers_outcome(
        &self,
        fee_payers_outcome: ExerciseRoleOutcome,
    ) -> Result<()> {
        assert_eq!(fee_payers_outcome.role, RoleKind::Primary);
        self.updating_state(|state| {
            state.update_with_exercise_role_outcome(fee_payers_outcome);
        })?;
        Ok(())
    }

    fn try_updating_state<R>(
        &self,
        f: impl FnOnce(&mut SigningManagerState) -> Result<R>,
    ) -> Result<R> {
        let mut state_holder =
            self.state.write().map_err(|_| CommonError::Unknown)?; // TODO specific error variant
        if let Some(state) = state_holder.as_mut() {
            f(state)
        } else {
            unreachable!("State should be Some");
        }
    }

    fn updating_state<R>(
        &self,
        f: impl FnOnce(&mut SigningManagerState) -> R,
    ) -> Result<R> {
        self.try_updating_state(|state| Ok(f(state)))
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

    fn outcome(&self) -> Result<SigningManagerOutcome> {
        let mut state = self.state.write().map_err(|_| CommonError::Unknown)?; // TODO specific error variant
        let _state = state.take().ok_or(CommonError::Unknown)?; // TODO specific error variant
        todo!()
    }
}

// ==================
// ==== TO SIGN =====
// ==================

/// An ID generated for the purpose of being able to identify which "set" a
/// TransactionIntent belongs to.
#[derive(Clone, Copy, PartialEq, Eq, StdHash, derive_more::Debug)]
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

    intent: TransactionIntent,

    part_of_intent_set: IntentSetID,

    /// Must match the owner inside `signatures.map(|s| s.input.owned_factor_instance.owner`
    entity: AccountOrPersona,

    signatures: IndexSet<SignatureWithPublicKey>,

    /// None `intent` is the single intent of an IntentSet. If the intent
    /// is one of the many variants of an intentset then this variable must
    /// be `Some`.
    ///
    /// If Some() then its exercisable roles must "contain" `role_kind`.
    variant: Option<RolesExercisableInTransactionManifestCombination>,

    role_kind: RoleKind,
}
impl IntentWithSignatures {
    pub(crate) fn new(
        part_of_intent_set: IntentSetID,
        intent: TransactionIntent,
        entity: AccountOrPersona,
        signatures: IndexSet<SignatureWithPublicKey>,
        role_kind: RoleKind,
        variant: Option<RolesExercisableInTransactionManifestCombination>,
    ) -> Self {
        if let Some(variant) = variant.as_ref() {
            assert!(variant.exercisable_roles().contains(&role_kind))
        }
        Self {
            hidden: HiddenConstructor,
            part_of_intent_set,
            intent,
            entity,
            signatures,
            role_kind,
            variant,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EntityNotSignedFor {
    intent: TransactionIntent,
    entity: AccountOrPersona,
    role_kind: RoleKind,
    neglected_factor_sources: IndexSet<NeglectedFactor>,
    variant: Option<RolesExercisableInTransactionManifestCombination>,
}

impl EntityNotSignedFor {
    fn new(
        intent: TransactionIntent,
        entity: AccountOrPersona,
        role_kind: RoleKind,
        neglected_factor_sources: IndexSet<NeglectedFactor>,
        variant: Option<RolesExercisableInTransactionManifestCombination>,
    ) -> Self {
        if let Some(variant) = variant.as_ref() {
            assert!(variant.exercisable_roles().contains(&role_kind))
        }
        Self {
            intent,
            entity,
            neglected_factor_sources,
            role_kind,
            variant,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Deref)]
struct EntitiesSignedFor(Vec<IntentWithSignatures>); // want IndexSet, but Item is not StdHash.
impl From<Vec<IntentWithSignatures>> for EntitiesSignedFor {
    fn from(v: Vec<IntentWithSignatures>) -> Self {
        Self(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Deref)]
struct EntitiesNotSignedFor(Vec<EntityNotSignedFor>); // want IndexSet, but Item is not StdHash.
impl From<Vec<EntityNotSignedFor>> for EntitiesNotSignedFor {
    fn from(v: Vec<EntityNotSignedFor>) -> Self {
        Self(v)
    }
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
    entities_signed_for: EntitiesSignedFor,

    entities_not_signed_for: EntitiesNotSignedFor,
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

        assert!(entities_signed_for.iter().all(|e| e.role_kind == role_kind));
        assert!(entities_not_signed_for
            .iter()
            .all(|e| e.role_kind == role_kind));

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
            entities_signed_for: entities_signed_for.into(),
            entities_not_signed_for: entities_not_signed_for.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedIntentSet {
    intents: Vec<IntentWithSignatures>, // Want IndexSet but TransactionIntent is not `std::hash::Hash`
}
impl SignedIntentSet {
    pub fn get_best_signed_intent(self) -> Result<SignedIntent> {
        let first =
            self.intents.first().ok_or(CommonError::Unknown).cloned()?; // TODO specific error variant

        let from = |item: IntentWithSignatures| -> Result<SignedIntent> {
            let intent = item.intent.clone();
            let signatures = item
                .signatures
                .into_iter()
                .map(IntentSignature::from)
                .collect_vec();

            SignedIntent::new(intent, IntentSignatures::new(signatures))
        };

        if self.intents.len() == 1 {
            from(first)
        } else {
            assert!(self.intents.iter().all(|i| i.variant.is_some()));

            let rated_by_tx_variant = self
                .intents
                .into_iter()
                .sorted_by_key(|i| i.variant.unwrap().rating())
                .collect_vec();
            let best = rated_by_tx_variant.first().unwrap().clone();
            from(best)
        }
    }
}

trait HasTransactionVariantRating {
    /// `0` means best
    fn rating(&self) -> u8;
}
impl HasTransactionVariantRating
    for RolesExercisableInTransactionManifestCombination
{
    fn rating(&self) -> u8 {
        match self {
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation => { assert_eq!(*self, Self::best());0 }, // best
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary => 1,
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion => 2,
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation => 3,
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion => 4,
        }
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
