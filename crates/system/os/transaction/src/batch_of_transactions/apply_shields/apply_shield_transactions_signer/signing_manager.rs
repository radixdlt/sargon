use crate::prelude::*;

/// Implementation of complex signing flow laid out in this
/// [whimsical diagram][flow].
///
/// [flow]: https://whimsical.com/wallet-sargon-signing-flow-QFvU2NAVXFiX1VgNBuvj5g
pub struct SigningManager {
    /// FactorSources in Profile
    factor_sources_in_profile: IndexSet<FactorSource>,
    interactor: Arc<dyn SignInteractor<TransactionIntent>>,
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
    pub async fn sign_intent_sets(
        &self,
        intent_sets: impl IntoIterator<
            Item = SecurityShieldApplicationWithTransactionIntents,
        >,
    ) -> Result<Vec<SignedIntentSet>> {
        let intent_sets = intent_sets.into_iter().collect_vec();
        let sign_with_recovery =
            self.sign_intents_with_recovery_role(&intent_sets).await?;
        todo!()
    }
}

// ==============
// === PRIVATE ===
// ==============
impl SigningManager {
    async fn do_sign_intents_with_role(
        &self,
        intents: Vec<IntentToSign>,
        role: RoleKind,
    ) -> Result<IndexSet<IntentWithSignatures>> {
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

        let outcome = collector.collect_signatures().await?;

        todo!()
    }

    async fn sign_intents_with_role(
        &self,
        intents: &[SecurityShieldApplicationWithTransactionIntents],
        role: RoleKind,
    ) -> Result<IndexSet<IntentWithSignatures>> {
        todo!()
    }

    async fn sign_intents_with_primary_role(
        &self,
        intents: &[SecurityShieldApplicationWithTransactionIntents],
    ) -> Result<IndexSet<IntentWithSignatures>> {
        self.sign_intents_with_role(intents, RoleKind::Primary)
            .await
    }

    async fn sign_intents_with_recovery_role(
        &self,
        intents: &[SecurityShieldApplicationWithTransactionIntents],
    ) -> Result<IndexSet<IntentWithSignatures>> {
        self.sign_intents_with_role(intents, RoleKind::Recovery)
            .await
    }

    async fn sign_intents_with_confirmation_role(
        &self,
        intents: &[SecurityShieldApplicationWithTransactionIntents],
    ) -> Result<IndexSet<IntentWithSignatures>> {
        self.sign_intents_with_role(intents, RoleKind::Confirmation)
            .await
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IntentToSign {
    intent: TransactionIntent,
    entities: Vec<AccountOrPersona>, // often one, or two (payer != entity)
    variant: Option<RolesExercisableInTransactionManifestCombination>,
}

type SignatureWithContext = HDSignature<TransactionIntentHash>;

#[derive(Clone, PartialEq, Eq)]
struct IntentWithSignatures {
    intent: IntentToSign,
    signatures: IndexSet<SignatureWithContext>,
    neglected_factor_sources: IndexSet<NeglectedFactor>, // TODO Needed?
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IntentsToSign {
    intents: Vec<IntentToSign>,
}

pub struct SignedIntentSet {
    intents: IndexSet<IntentToSign>,
}
impl SignedIntentSet {
    pub fn get_best_signed_intent(&self) -> SignedIntent {
        todo!()
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
