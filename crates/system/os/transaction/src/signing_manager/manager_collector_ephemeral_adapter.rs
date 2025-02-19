use crate::prelude::*;

/// An ephemeral context used to translate input/output to/from SignaturesCollector
/// to the state of a SigningManager.
pub(crate) struct ManagerCollectorEphemeralAdapter {
    role_kind: Immutable<RoleKind>,

    lookup_address_to_entity:
        Immutable<HashMap<AddressOfAccountOrPersona, AccountOrPersona>>,

    lookup_txid_to_intent_set:
        Immutable<HashMap<EntityInTxCompoundKey, IntentSetID>>,

    lookup_txid_to_variant: Immutable<
        HashMap<
            TransactionIntentHash,
            Option<RolesExercisableInTransactionManifestCombination>,
        >,
    >,

    lookup_intent_by_txid:
        Immutable<HashMap<EntityInTxCompoundKey, TransactionIntent>>,

    transactions_with_petitions:
        Immutable<IdentifiedVecOf<SignableWithEntities<TransactionIntent>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, StdHash)]
pub struct CompoundKey<Left, Right>
where
    Left: StdHash,
    Right: StdHash,
{
    pub left: Left,
    pub right: Right,
}
impl<Left: StdHash, Right: StdHash> CompoundKey<Left, Right> {
    pub fn new(left: Left, right: Right) -> Self {
        Self { left, right }
    }
}
pub type EntityInTxCompoundKey =
    CompoundKey<AddressOfAccountOrPersona, TransactionIntentHash>;

impl ManagerCollectorEphemeralAdapter {
    pub(crate) fn new(
        role_kind: RoleKind,
        intent_sets: Vec<IntentSetToSign>,
    ) -> Self {
        // TODO should probably move these lookup tables into fields of `SigningManager` and
        // change how we construct the SigningManager.
        let mut lookup_address_to_entity =
            HashMap::<AddressOfAccountOrPersona, AccountOrPersona>::new();
        let mut lookup_txid_to_intent_set =
            HashMap::<EntityInTxCompoundKey, IntentSetID>::new();
        let mut lookup_txid_to_variant = HashMap::<
            TransactionIntentHash,
            Option<RolesExercisableInTransactionManifestCombination>,
        >::new();
        let mut lookup_intent_by_txid =
            HashMap::<EntityInTxCompoundKey, TransactionIntent>::new();

        let transactions_with_petitions = intent_sets
            .into_iter()
            .flat_map(|set| {
                set.variants
                    .iter()
                    .map(|variant| {
                        let tx = variant.intent.clone();
                        let txid = tx.transaction_intent_hash();

                        let entity_requiring_auth = set.entity.clone();

                        let compound_key = EntityInTxCompoundKey::new(
                            entity_requiring_auth.address(),
                            txid.clone(),
                        );

                        // Insert TXID into the lookup so we can group the signatures
                        // of each intent by IntentSetID.
                        let existed = lookup_txid_to_intent_set
                            .insert(compound_key, set.intent_set_id);
                        assert_eq!(
                            existed, None,
                            "Duplicate TXID for IntentSetID"
                        );

                        let existed = lookup_txid_to_variant
                            .insert(txid.clone(), variant.variant);
                        assert_eq!(existed, None, "Duplicate TXID for Variant");

                        let existed = lookup_intent_by_txid
                            .insert(compound_key, tx.clone());
                        assert_eq!(existed, None, "Duplicate Intent for TXID");

                        lookup_address_to_entity.insert(
                            entity_requiring_auth.address(),
                            entity_requiring_auth.clone(),
                        );

                        SignableWithEntities::new(tx, [entity_requiring_auth])
                    })
                    .collect_vec()
            })
            .collect::<IdentifiedVecOf<_>>();

        Self {
            role_kind: role_kind.into(),
            transactions_with_petitions: transactions_with_petitions.into(),
            lookup_address_to_entity: lookup_address_to_entity.into(),
            lookup_txid_to_intent_set: lookup_txid_to_intent_set.into(),
            lookup_txid_to_variant: lookup_txid_to_variant.into(),
            lookup_intent_by_txid: lookup_intent_by_txid.into(),
        }
    }

    pub(crate) fn transactions_with_petitions(
        &self,
    ) -> IdentifiedVecOf<SignableWithEntities<TransactionIntent>> {
        (*self.transactions_with_petitions).clone()
    }

    fn get_context(
        &self,
        txid: TransactionIntentHash,
        entity: AddressOfAccountOrPersona,
    ) -> EntitySigningContext {
        let compound_key = EntityInTxCompoundKey::new(entity, txid);
        let intent_set_id =
            *self.lookup_txid_to_intent_set.get(&compound_key).unwrap();

        EntitySigningContext::new(intent_set_id, *self.role_kind)
    }

    pub(crate) fn exercise_role_outcome(
        &self,
        signatures_collector_outcome: SignaturesOutcome<TransactionIntentHash>,
    ) -> Result<ExerciseRoleOutcome> {
        let entities_signed_for: Vec<EntitySignedFor> = signatures_collector_outcome
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

                let entity = self.lookup_address_to_entity
                    .get(&owner_address)
                    .unwrap()
                    .clone();
                let compound_key = EntityInTxCompoundKey::new(owner_address, txid.clone());
                let intent = self.lookup_intent_by_txid.get(&compound_key).unwrap().clone();

                EntitySignedFor::new(
                    self.get_context(txid, owner_address),
                    intent,
                    entity,
                    signatures_with_inputs
                        .into_iter()
                        .map(|s| s.signature)
                        .collect(),
                )
            })
            .collect_vec();

        let entities_not_signed_for: Vec<EntityNotSignedFor> =
            signatures_collector_outcome
                .failed_transactions_outcomes()
                .into_iter()
                .map(|o| {
                    let txid = o.signable_id;

                    let per_entity_neglected_factor_sources =
                        o.per_entity_neglected_factors.clone();
                    assert_eq!(
                        per_entity_neglected_factor_sources.len(),
                        1,
                        "Should have exactly one entity"
                    ); // TODO add support for multiple entities
                    let (owner_address, neglected_factors) =
                        per_entity_neglected_factor_sources
                            .into_iter()
                            .next()
                            .expect("Already validate to have at least entity");

                    let entity = self
                        .lookup_address_to_entity
                        .get(&owner_address)
                        .unwrap()
                        .clone();

                    let compound_key =
                        EntityInTxCompoundKey::new(owner_address, txid.clone());
                    let intent = self
                        .lookup_intent_by_txid
                        .get(&compound_key)
                        .unwrap()
                        .clone();

                    EntityNotSignedFor::new(
                        self.get_context(txid, owner_address),
                        intent,
                        entity,
                        neglected_factors,
                    )
                })
                .collect_vec();

        Ok(ExerciseRoleOutcome::new(
            *self.role_kind,
            entities_signed_for,
            entities_not_signed_for,
        ))
    }
}
