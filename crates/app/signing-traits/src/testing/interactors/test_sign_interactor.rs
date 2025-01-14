#![allow(unused)]

use crate::prelude::*;

pub struct TestSignInteractor<S: Signable> {
    pub(crate) simulated_user: SimulatedUser<S>,
}

impl<S: Signable> TestSignInteractor<S> {
    pub fn new(simulated_user: SimulatedUser<S>) -> Self {
        Self { simulated_user }
    }
}

impl<S: Signable> IsTestInteractor<S> for TestSignInteractor<S> {
    fn simulated_user(&self) -> SimulatedUser<S> {
        self.simulated_user.clone()
    }
}

#[async_trait::async_trait]
impl<S: Signable> SignInteractor<S> for TestSignInteractor<S> {
    async fn sign(
        &self,
        request: SignRequest<S>,
    ) -> Result<SignResponse<S::ID>> {
        self.simulated_user.spy_on_request_before_handled(
            request.factor_source_kind(),
            request
                .invalid_transactions_if_all_factors_neglected()
                .clone(),
        );
        let ids = request
            .per_factor_source
            .keys()
            .cloned()
            .collect::<IndexSet<_>>();

        if self.should_simulate_failure(ids.clone()) {
            return Ok(SignResponse::failure_with_factors(ids));
        }

        match self.simulated_user.sign_or_skip(
            request
                .invalid_transactions_if_all_factors_neglected()
                .clone(),
        ) {
            SigningUserInput::Sign => {
                let mut outcomes = IndexMap::<
                    FactorSourceIDFromHash,
                    FactorOutcome<S::ID>,
                >::new();

                for (id, input) in request.per_factor_source.clone() {
                    if self
                        .simulated_user
                        .simulate_skip_if_needed(IndexSet::just(id))
                    {
                        let outcome = FactorOutcome::skipped(id);
                        outcomes.insert(id, outcome);
                        continue;
                    }

                    match self.simulated_user.sign_or_skip(
                        request.invalid_transactions_if_factor_neglected(&id),
                    ) {
                        SigningUserInput::Sign => {
                            let signatures = input.per_transaction
                                .iter()
                                .flat_map(|x| {
                                    x.signature_inputs()
                                        .iter()
                                        .map(|y|
                                            unsafe {
                                                HDSignature::produced_signing_with_input(y.clone())
                                            }
                                        )
                                        .collect_vec()
                                })
                                .collect::<IndexSet<HDSignature<S::ID>>>();

                            let outcome = FactorOutcome::signed(signatures)?;
                            outcomes.insert(id, outcome);
                        }
                        SigningUserInput::Skip => {
                            let outcome = FactorOutcome::skipped(id);
                            outcomes.insert(id, outcome);
                        }
                        SigningUserInput::Reject => {
                            return Err(CommonError::SigningRejected);
                        }
                    }
                }

                SignResponse::new_from_outcomes(outcomes)
            }

            SigningUserInput::Skip => {
                Ok(SignResponse::user_skipped_factors(ids))
            }

            SigningUserInput::Reject => Err(CommonError::SigningRejected),
        }
    }
}
