use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsSigner: Send + Sync {
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload>;
}

pub struct ApplyShieldTransactionsSignerImpl {
    signing_manager: SigningManager,
}

impl ApplyShieldTransactionsSignerImpl {
    pub fn new(os: &SargonOS) -> Result<Self> {
        os.profile()
            .map(|profile| SigningManager {
                profile,
                interactor: os.sign_transactions_interactor(),
            })
            .map(|signing_manager| Self { signing_manager })
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsSigner for ApplyShieldTransactionsSignerImpl {
    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        let notary_manager = NotaryManager {
            keys_for_intents: payload_to_sign.notary_keys,
        };
        let intent_sets = payload_to_sign.applications_with_intents;
        let signed_sets =
            self.signing_manager.sign_intent_sets(intent_sets).await?;

        let signed_intents = signed_sets
            .into_iter()
            .map(|signed_set| signed_set.get_best_signed_intent())
            .collect_vec();

        let notarized_transactions = notary_manager.notarize(signed_intents)?;

        Ok(ApplySecurityShieldSignedPayload {
            notarized_transactions,
        })
    }
}

pub struct SigningManager {
    interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    profile: Profile, // TODO: Remove this AND requirement of it from SignaturesCollector
}

struct IntentToSign {
    intent: TransactionIntent,
    entities: Vec<AddressOfAccountOrPersona>, // often one, or two (payer != entity)
    variant: Option<RolesExercisableInTransactionManifestCombination>,
}

struct IntentWithSignatures {
    intent: TransactionIntent,
    signatures: IndexSet<SignatureWithPublicKey>,
}

struct IntentsToSign {
    intents: Vec<IntentToSign>,
}

impl SigningManager {
    
    fn do_sign_intents_with_role(
        &self,
        intents: Vec<IntentToSign>,
        role: RoleKind,
    ) -> Vec<IntentWithSignatures> {
        todo!()
    }

    fn sign_intents_with_role(
        &self,
        intents: &[SecurityShieldApplicationWithTransactionIntents],
        role: RoleKind,
    ) -> Vec<IntentWithSignatures> {
        todo!()
    }

    fn sign_intents_with_primary_role(
        &self,
        intents: &[SecurityShieldApplicationWithTransactionIntents],
    ) -> Vec<IntentWithSignatures> {
       self.sign_intents_with_role(intents, RoleKind::Primary)
    }

    fn sign_intents_with_recovery_role(
        &self,
        intents: &[SecurityShieldApplicationWithTransactionIntents],
    ) -> Vec<IntentWithSignatures> {
       self.sign_intents_with_role(intents, RoleKind::Recovery)
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
        let with_primary = self.sign_intents_with_primary_role(&intent_sets);
        todo!()
    }
}

pub struct SignedIntentSet {
    intents: IndexSet<IntentToSign>,
}
impl SignedIntentSet {
    pub fn get_best_signed_intent(&self) -> SignedIntent {
        todo!()
    }
}

pub struct NotaryManager {
    keys_for_intents: IndexMap<TransactionIntentHash, Ed25519PrivateKey>,
}
impl NotaryManager {
    pub fn notarize(
        self,
        signed_intents: impl IntoIterator<Item = SignedIntent>,
    ) -> Result<Vec<NotarizedTransaction>> {
        let signed_intents = signed_intents.into_iter().collect_vec();
        let mut key_for_intent = self.keys_for_intents;
        signed_intents
            .into_iter()
            .map(|signed_intent| {
                let intent = signed_intent.intent();
                let private_key = key_for_intent
                    .swap_remove(&intent.transaction_intent_hash())
                    .ok_or_else(|| CommonError::Unknown)?;
                let notary_signature =
                    private_key.notarize_hash(&signed_intent.hash());
                NotarizedTransaction::new(signed_intent, notary_signature)
            })
            .collect::<Result<Vec<_>>>()
    }
}
