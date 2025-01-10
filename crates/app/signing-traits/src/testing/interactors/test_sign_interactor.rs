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
                let per_factor_outcome = IndexMap::from_iter(request
                    .per_factor_source
                    .iter()
                    .map(|(id, input)| {
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

                        (*id, signatures)
                    }));

                SignResponse::signed(per_factor_outcome)
            }

            SigningUserInput::Skip => {
                Ok(SignResponse::user_skipped_factors(ids))
            }

            SigningUserInput::Reject => Err(CommonError::SigningRejected),
        }
    }
}
