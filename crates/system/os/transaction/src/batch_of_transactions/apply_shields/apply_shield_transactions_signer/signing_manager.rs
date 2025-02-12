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
    async fn sign_intents_with_role(
        &self,
        intents: IndexSet<IntentToSign>,
        role: RoleKind,
    ) -> Result<ExerciseRoleOutcome> {
        let purpose = SigningPurpose::SignTX { role_kind: role };

        let transactions_with_petitions = intents
            .iter()
            .into_iter()
            .map(|t| {
                SignableWithEntities::new(t.intent.clone(), t.entities.clone())
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
        let _outcome = collector.collect_signatures().await?;

        // TODO: Split `outcome` into `entities_signed_for` and `entities_not_signed_for`

        let entities_signed_for: Vec<IntentWithSignatures> = vec![];

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
        let intents: IndexSet<IntentToSign> = IndexSet::new(); // TODO: Get intents from state
        let outcome = self
            .sign_intents_with_role(intents, RoleKind::Recovery)
            .await?;
        self.handle_recovery_outcome(outcome)
    }

    async fn sign_intents_with_confirmation_role(&self) -> Result<()> {
        let intents: IndexSet<IntentToSign> = IndexSet::new(); // TODO: Get intents from state
        let outcome = self
            .sign_intents_with_role(intents, RoleKind::Confirmation)
            .await?;
        self.handle_confirmation_outcome(outcome)
    }

    async fn sign_intents_with_primary_role(&self) -> Result<()> {
        let intents: IndexSet<IntentToSign> = IndexSet::new(); // TODO: Get intents from state
        let outcome = self
            .sign_intents_with_role(intents, RoleKind::Primary)
            .await?;
        self.handle_primary_outcome(outcome)
    }

    async fn sign_for_fee_payers(&self) -> Result<()> {
        let intents: IndexSet<IntentToSign> = IndexSet::new(); // TODO: Get intents from state

        // We are goign to spend the fee paying accouts XRD
        // so we use Primary role
        let role = RoleKind::Primary;

        let outcome = self.sign_intents_with_role(intents, role).await?;

        self.handle_fee_payers_outcome(outcome)
    }
}

// ==================
// ==== TO SIGN =====
// ==================
#[derive(Debug, Clone, PartialEq, Eq)]
struct IntentToSign {
    intent: TransactionIntent,

    /// For shield applying manifests this Vec contains a single entity, either
    /// the entity applying the shield or the fee payer - we are doing
    /// four passes to the SignaturesCollector, one for each role for the
    /// entities applying the shield, then a fourth pass for the fee payer
    /// of each transaction exercising its Primary role.
    entities: Vec<AccountOrPersona>,

    variant: Option<RolesExercisableInTransactionManifestCombination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IntentSetToSign {
    intents: Vec<IntentToSign>, // Want IndexSet but TransactionIntent is not `std::hash::Hash`
}

// =================
// ==== SIGNED =====
// =================

type SignatureWithContext = HDSignature<TransactionIntentHash>;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub(crate) struct IntentWithSignatures {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    hidden: HiddenConstructor,

    /// TransactionIntentHash of this intent MUST match `signatures.map(|s| s.input.payload_id)`
    intent: TransactionIntent,

    /// Must match the owner inside `signatures.map(|s| s.input.owned_factor_instance.owner`
    entity: AccountOrPersona,

    /// `signatures.map(|s| s.input.owned_factor_instance.owner` must match owner `entity`
    /// `signatures.map(|s| s.input.payload_id)` must match TX hash of `intent`
    signatures: IndexSet<SignatureWithContext>,

    /// None `intent` is the single intent of an IntentSet. If the intent
    /// is one of the many variants of an intentset then this variable must
    /// be `Some`.
    variant: Option<RolesExercisableInTransactionManifestCombination>,
}
impl IntentWithSignatures {
    /// # Panics
    /// Panics if there is a discrepancy betwen TX hash of `intent and `signatures`.
    pub(crate) fn new(
        intent: TransactionIntent,
        entity: AccountOrPersona,
        signatures: IndexSet<SignatureWithContext>,
        variant: Option<RolesExercisableInTransactionManifestCombination>,
    ) -> Self {
        assert!(signatures
            .iter()
            .all(|s| *s.payload_id() == intent.transaction_intent_hash()));
        Self {
            hidden: HiddenConstructor,
            intent,
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
    intents: Vec<IntentToSign>, // Want IndexSet but TransactionIntent is not `std::hash::Hash`
}
impl SignedIntentSet {
    pub fn get_best_signed_intent(&self) -> SignedIntent {
        todo!()
    }
}

struct SigningManagerOutcome {
    successfully_signed_intent_sets: Vec<SignedIntentSet>,
    failed_intent_sets: Vec<SignedIntentSet>,
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
