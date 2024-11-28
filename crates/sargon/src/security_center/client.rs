use crate::prelude::*;

pub struct SecurityCenterClient;

impl SecurityCenterClient {
    pub fn check_security_problems(
        input: CheckSecurityProblemsInput,
    ) -> Vec<SecurityProblem> {
        let mut problems = Vec::new();

        let has_problem_3 = || {
            if input.unrecoverable_entities.is_empty() {
                return None;
            }
           Some(input.unrecoverable_entities)
        };

        let has_problem_5 = || {
            if !*input.is_cloud_profile_sync_enabled.clone() {
                return false;
            }
            let Some(cloud_backup) = input.last_cloud_backup else {
                return true;
            };
            cloud_backup.is_failed
        };

        let has_problem_6 = || {
            !*input.is_cloud_profile_sync_enabled.clone() && input.last_manual_backup.is_none()
        };

        let has_problem_7 = || {
            !*input.is_cloud_profile_sync_enabled.clone()
                && input.last_manual_backup.as_ref().map_or(false, |backup| !backup.is_current)
        };

        let has_problem_9 = || {
            if input.without_control_entities.is_empty() {
                return None;
            }
            Some(input.without_control_entities)
        };

        if let Some(addresses) = has_problem_3() {
            problems.push(SecurityProblem::Problem3 { addresses });
        }

        if has_problem_5() {
            problems.push(SecurityProblem::Problem5);
        }

        if has_problem_6() {
            problems.push(SecurityProblem::Problem6);
        }

        if has_problem_7() {
            problems.push(SecurityProblem::Problem7);
        }

        if let Some(addresses) = has_problem_9() {
            problems.push(SecurityProblem::Problem9 { addresses });
        }

        problems
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityCenterClient;

    #[test]
    fn problem_5() {
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled::sample(),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::empty(),
            BackupResult::sample(),
            None,
        );

        let result = SUT::check_security_problems(input);

        // assert!(result.contains(&SecurityProblem::Problem5));
    }


}
