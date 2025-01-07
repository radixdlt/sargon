#![allow(unused)]

use crate::prelude::*;

pub struct TestSignInteractor<S: Signable> {
    pub simulated_user: SimulatedUser<S>,
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
    ) -> Result<SignWithFactorsOutcome<S::ID>> {
        self.simulated_user.spy_on_request_before_handled(
            request.factor_source_kind(),
            request.invalid_transactions_if_neglected.clone(),
        );
        let ids = request
            .per_factor_source
            .keys()
            .cloned()
            .collect::<IndexSet<_>>();

        if self.should_simulate_failure(ids.clone()) {
            return Ok(SignWithFactorsOutcome::failure_with_factors(ids));
        }

        match self
            .simulated_user
            .sign_or_skip(request.invalid_transactions_if_neglected.clone())
        {
            SigningUserInput::Sign => {
                let signatures = request
                    .per_factor_source
                    .iter()
                    .flat_map(|(_, v)| {
                        v.iter()
                            .flat_map(|x| {
                                x.signature_inputs()
                                    .iter()
                                    .map(|y| unsafe { HDSignature::produced_signing_with_input(y.clone())})
                                    .collect_vec()
                            })
                            .collect::<IndexSet<HDSignature<S::ID>>>()
                    })
                    .collect::<IndexSet<HDSignature<S::ID>>>();

                let signatures = signatures
                    .into_iter()
                    .into_group_map_by(|x| x.factor_source_id());
                let response = SignResponse::new(
                    signatures
                        .into_iter()
                        .map(|(k, v)| (k, IndexSet::from_iter(v)))
                        .collect(),
                );

                Ok(SignWithFactorsOutcome::signed(response))
            }

            SigningUserInput::Skip => {
                Ok(SignWithFactorsOutcome::user_skipped_factors(ids))
            }

            SigningUserInput::Reject => Err(CommonError::SigningRejected),
        }
    }
}
