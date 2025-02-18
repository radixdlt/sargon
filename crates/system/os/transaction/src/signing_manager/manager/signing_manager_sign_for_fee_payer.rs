use crate::prelude::*;

impl SigningManager {
    pub(super) async fn sign_for_fee_payers(
        &self,
        signed_intents: Vec<SignedIntentWithContext>,
    ) -> Result<SigningManagerOutcome> {
        let role_kind = RoleKind::Primary;

        let payer_by_tx_id = |intent_set_id: IntentSetID,
                              txid: TransactionIntentHash|
         -> Result<Account> {
            let state = self._get_state();
            let s = state.per_set_state.get(&intent_set_id).unwrap();
            let txids = s.internal_state.transaction_intent_hashes();
            assert!(txids.contains(&txid));
            Ok(s.internal_state.paying_account())
        };

        // We are NOT signing intent SETs but we piggy back
        // on the existing code above, and inlay a single intent into a set
        // to be able to use the same code.
        let intent_sets = signed_intents
            .iter()
            .map(|si| {
                let intent_set_id = si.intent_set_id();
                let txid = si.signed_intent.intent.transaction_intent_hash();
                let entity = payer_by_tx_id(intent_set_id, txid)?;
                Ok(IntentSetToSign::single_intent(
                    IntentSetID::new(),
                    role_kind,
                    IntentVariant::new(None, si.signed_intent.intent.clone()),
                    entity.into(),
                ))
            })
            .collect::<Result<Vec<IntentSetToSign>>>()?;

        let mut signed_intents = signed_intents
            .into_iter()
            .map(|si| (si.intent_set_id(), si.signed_intent))
            .collect::<IndexMap<IntentSetID, SignedIntent>>();

        let exercise_role_outcome = self
            .sign_intent_sets_with_role(intent_sets, RoleKind::Primary)
            .await?;

        assert!(exercise_role_outcome.entities_not_signed_for.is_empty());

        let signed_with_payers = exercise_role_outcome.entities_signed_for;
        signed_with_payers
            .0
            .into_iter()
            .for_each(|signed_with_payer| {
                let signed_intent = signed_intents
                    .get_mut(&signed_with_payer.intent_set_id())
                    .expect("Should have signed intent");
                signed_intent.add_fee_payer_signatures(
                    signed_with_payer.intent_signatures(),
                );
            });

        Ok(SigningManagerOutcome(
            signed_intents.values().cloned().collect_vec(),
        ))
    }
}
