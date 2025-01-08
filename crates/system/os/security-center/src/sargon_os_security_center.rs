use crate::prelude::*;

pub trait OsCheckSecurityProblems {
    fn check_security_problems(
        &self,
        input: CheckSecurityProblemsInput,
    ) -> Result<Vec<SecurityProblem>>;
}

// ==================
// Check Security Problems
// ==================
impl OsCheckSecurityProblems for SargonOS {
    /// Returns all the `SecurityProblem`s that are present for the given input.
    fn check_security_problems(
        &self,
        input: CheckSecurityProblemsInput,
    ) -> Result<Vec<SecurityProblem>> {
        Ok(SecurityCenterClient::check_security_problems(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn check_problems() {
        let os = boot().await;

        let input = CheckSecurityProblemsInput::sample();
        let result = os.check_security_problems(input.clone()).unwrap();
        assert_eq!(
            result,
            SecurityCenterClient::check_security_problems(input)
        );
    }

    async fn boot() -> Arc<SargonOS> {
        let req = SUT::boot_test_with_networking_driver(Arc::new(
            MockNetworkingDriver::new_always_failing(),
        ));

        actix_rt::time::timeout(SARGON_OS_TEST_MAX_ASYNC_DURATION, req)
            .await
            .unwrap()
            .unwrap()
    }
}
