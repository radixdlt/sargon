use crate::prelude::*;

pub struct TestTransactionSignInteractor;

impl TestTransactionSignInteractor {

    async fn sign_mono(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        request: &SignRequest<TransactionIntent>,
        transactions_to_sign: &IndexSet<TransactionSignRequestInput<TransactionIntent>>,
    ) -> SignWithFactorsOutcome<TransactionIntentHash> {
        if request.invalid_transactions_if_neglected.is_empty() {
            return SignWithFactorsOutcome::Neglected(
                NeglectedFactors::new(
                    NeglectFactorReason::UserExplicitlySkipped,
                    IndexSet::just(factor_source_id),
                ),
            );
        }

        let signatures = transactions_to_sign
            .iter()
            .map(|per_transaction| {
                per_transaction
                    .signature_inputs()
                    .iter()
                    .map(|x| HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(x.clone()))
                    .collect::<IndexSet<_>>()
            })
            .flatten()
            .collect::<IndexSet<HDSignature<TransactionIntentHash>>>();

        SignWithFactorsOutcome::Signed {
            produced_signatures: SignResponse::with_signatures(
                signatures,
            ),
        }
    }

}

#[async_trait::async_trait]
impl SignInteractor<TransactionIntent> for TestTransactionSignInteractor {
    async fn sign(&self, request: SignRequest<TransactionIntent>) -> SignWithFactorsOutcome<TransactionIntentHash> {
        let mut signatures = IndexSet::<HDSignature<TransactionIntentHash>>::new();

        for (factor_source_id, inputs) in request.per_factor_source.iter() {
            let result = self
                .sign_mono(factor_source_id.clone(), &request, inputs)
                .await;

            match result {
                SignWithFactorsOutcome::Signed {
                    produced_signatures,
                } => {
                    signatures.extend(
                        produced_signatures
                            .signatures
                            .into_iter()
                            .flat_map(|(_, xs)| xs)
                            .collect::<IndexSet<_>>(),
                    );
                }
                SignWithFactorsOutcome::Neglected(_) => {
                    return SignWithFactorsOutcome::Neglected(
                        NeglectedFactors::new(
                            NeglectFactorReason::UserExplicitlySkipped,
                            request.factor_source_ids(),
                        ),
                    );
                }
            }
        }
        SignWithFactorsOutcome::signed(SignResponse::with_signatures(signatures))
    }
}