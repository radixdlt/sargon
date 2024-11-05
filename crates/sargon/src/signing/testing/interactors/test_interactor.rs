#![allow(unused)]

use crate::prelude::*;

#[async_trait::async_trait]
pub trait IsTestInteractor: Sync {
    fn simulated_user(&self) -> SimulatedUser;

    fn should_simulate_failure(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        self.simulated_user()
            .simulate_failure_if_needed(factor_source_ids)
    }
}
