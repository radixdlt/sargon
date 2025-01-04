use crate::prelude::*;

// ==================
// Check Security Problems
// ==================
#[uniffi::export]
impl SargonOS {
    /// Returns all the `SecurityProblem`s that are present for the given input.
    pub fn check_security_problems(
        &self,
        input: CheckSecurityProblemsInput,
    ) -> Result<Vec<SecurityProblem>> {
        self.wrapped
            .check_security_problems(input.into_internal())
            .into_iter_result()
    }
}
