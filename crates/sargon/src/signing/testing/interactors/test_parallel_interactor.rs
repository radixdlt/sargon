#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;

pub(crate) struct TestSigningParallelInteractor {
    simulated_user: SimulatedUser,
}

impl TestSigningParallelInteractor {
    pub(crate) fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[async_trait::async_trait]
impl IsTestInteractor for TestSigningParallelInteractor {
    fn simulated_user(&self) -> SimulatedUser {
        self.simulated_user.clone()
    }
}

#[async_trait::async_trait]
impl PolyFactorSignInteractor<TransactionIntent>
    for TestSigningParallelInteractor
{
    async fn sign(
        &self,
        request: PolyFactorSignRequest<TransactionIntent>,
    ) -> SignWithFactorsOutcome<TransactionIntent> {
        self.simulated_user.spy_on_request_before_handled(
            request.clone().factor_source_kind(),
            request.clone().invalid_transactions_if_neglected,
        );
        let ids = request
            .per_factor_source
            .keys()
            .cloned()
            .collect::<IndexSet<_>>();

        if self.should_simulate_failure(ids.clone()) {
            return SignWithFactorsOutcome::failure_with_factors(ids);
        }

        match self
            .simulated_user
            .sign_or_skip(request.invalid_transactions_if_neglected)
        {
            SigningUserInput::Sign => {
                let signatures = request
                    .per_factor_source
                    .iter()
                    .flat_map(|(_, v)| {
                        v.per_transaction
                            .iter()
                            .flat_map(|x| {
                                x.signature_inputs()
                                    .iter()
                                    .map(|y| HDSignature::produced_signing_with_input(y.clone()))
                                    .collect_vec()
                            })
                            .collect::<IndexSet<HDSignature<TransactionIntent>>>()
                    })
                    .collect::<IndexSet<HDSignature<TransactionIntent>>>();

                let signatures = signatures
                    .into_iter()
                    .into_group_map_by(|x| x.factor_source_id());
                let response = SignResponse::new(
                    signatures
                        .into_iter()
                        .map(|(k, v)| (k, IndexSet::from_iter(v)))
                        .collect(),
                );

                SignWithFactorsOutcome::signed(response)
            }

            SigningUserInput::Skip => {
                SignWithFactorsOutcome::user_skipped_factors(ids)
            }
        }
    }
}
