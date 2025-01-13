use crate::prelude::*;

pub struct SecurityCenterClient;

impl SecurityCenterClient {
    pub fn check_security_problems(
        input: CheckSecurityProblemsInput,
    ) -> Vec<SecurityProblem> {
        let mut problems = Vec::new();

        let is_cloud_profile_sync_enabled =
            *input.is_cloud_profile_sync_enabled;

        let has_problem_3 = || {
            if input.unrecoverable_entities.is_empty() {
                return None;
            }
            Some(input.unrecoverable_entities)
        };

        let has_problem_5 = || {
            if !is_cloud_profile_sync_enabled {
                return false;
            }
            let Some(cloud_backup) = input.last_cloud_backup else {
                return true;
            };
            *cloud_backup.is_failed
        };

        let has_problem_6 = || {
            !is_cloud_profile_sync_enabled && input.last_manual_backup.is_none()
        };

        let has_problem_7 = || {
            !is_cloud_profile_sync_enabled
                && input
                    .last_manual_backup
                    .as_ref()
                    .is_some_and(|backup| !*backup.is_current)
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
    fn problem_3() {
        // Test without unrecoverable entities, we don't have Problem3
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled::sample(),
            AddressesOfEntitiesInBadState::empty(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.iter().any(|problem| matches!(
            problem,
            SecurityProblem::Problem3 { .. }
        )));

        // Test with unrecoverable entities, we have Problem3 for the specified addresses
        let addresses = AddressesOfEntitiesInBadState::sample();
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled::sample(),
            addresses.clone(),
            AddressesOfEntitiesInBadState::empty(),
            BackupResult::sample(),
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(result.contains(&SecurityProblem::Problem3 { addresses }));
    }

    #[test]
    fn problem_5() {
        let failed_backup = BackupResult::new(
            IsBackupResultCurrent(true),
            IsBackupResultFailed(true),
        );
        let success_backup = BackupResult::new(
            IsBackupResultCurrent(true),
            IsBackupResultFailed(false),
        );

        // Test with cloud profile sync disabled, we don't have Problem5
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(false),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            failed_backup.clone(),
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem5));

        // Test with cloud profile sync enabled and success backup, we don't have Problem5
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(true),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            success_backup.clone(),
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem5));

        // Test with cloud profile sync enabled and no backup, we have Problem5
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(true),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            None,
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(result.contains(&SecurityProblem::Problem5));

        // Test with cloud profile sync enabled and failed backup, we have Problem5
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(true),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            failed_backup.clone(),
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(result.contains(&SecurityProblem::Problem5));
    }

    #[test]
    fn problem_6() {
        // Test with cloud profile sync enabled and last manual backup, we don't have Problem6
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(true),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem6));

        // Test with cloud profile sync enabled and no last manual backup, we don't have Problem6
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(true),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            None,
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem6));

        // Test with cloud profile sync disabled and with last manual backup, we don't have Problem6
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(false),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem6));

        // Test with cloud profile sync disabled and no last manual backup, we have Problem6
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(false),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            None,
        );

        let result = SUT::check_security_problems(input);
        assert!(result.contains(&SecurityProblem::Problem6));
    }

    #[test]
    fn problem_7() {
        let current_backup = BackupResult::new(
            IsBackupResultCurrent(true),
            IsBackupResultFailed(false),
        );
        let outdated_backup = BackupResult::new(
            IsBackupResultCurrent(false),
            IsBackupResultFailed(false),
        );

        // Test with cloud profile sync enabled and no manual backup, we don't have Problem7
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(true),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            None,
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem7));

        // Test with cloud profile sync enabled and current manual backup, we don't have Problem7
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(true),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            current_backup.clone(),
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem7));

        // Test with cloud profile sync enabled and outdated manual backup, we don't have Problem7
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(true),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            outdated_backup.clone(),
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem7));

        // 99dasdasdasdasd

        // Test with cloud profile sync disabled and no manual backup, we don't have Problem7
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(false),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            None,
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem7));

        // Test with cloud profile sync disabled and current manual backup, we don't have Problem7
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(false),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            current_backup.clone(),
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.contains(&SecurityProblem::Problem7));

        // Test with cloud profile sync disabled and outdated manual backup, we have Problem7
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled(false),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::sample(),
            BackupResult::sample(),
            outdated_backup.clone(),
        );

        let result = SUT::check_security_problems(input);
        assert!(result.contains(&SecurityProblem::Problem7));
    }

    #[test]
    fn problem_9() {
        // Test without unrecoverable entities, we don't have Problem9
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled::sample(),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::empty(),
            BackupResult::sample(),
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(!result.iter().any(|problem| matches!(
            problem,
            SecurityProblem::Problem9 { .. }
        )));

        // Test with unrecoverable entities, we have Problem3 for the specified addresses
        let addresses = AddressesOfEntitiesInBadState::sample();
        let input = CheckSecurityProblemsInput::new(
            IsCloudProfileSyncEnabled::sample(),
            addresses.clone(),
            AddressesOfEntitiesInBadState::empty(),
            BackupResult::sample(),
            BackupResult::sample(),
        );

        let result = SUT::check_security_problems(input);
        assert!(result.contains(&SecurityProblem::Problem3 { addresses }));
    }
}
