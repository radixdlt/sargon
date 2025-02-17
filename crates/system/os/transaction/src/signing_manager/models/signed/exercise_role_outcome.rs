use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExerciseRoleOutcome {
    #[allow(dead_code)]
    #[doc(hidden)]
    hidden: HiddenConstructor,

    pub(crate) role: RoleKind,

    /// The `entities_signed_for.filter_map(|e| e.variant)` must "contain" `role`, e.g.
    /// if role is ROLE_PRIMARY_ROLE then variant cannot be
    /// RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation
    /// which does not "contain" Primary.
    pub(crate) entities_signed_for: EntitiesSignedFor,

    pub(crate) entities_not_signed_for: EntitiesNotSignedFor,
}

impl ExerciseRoleOutcome {
    /// # Panics
    /// Panics if there is a discrepancy between the entities_signed_for variant and `role_kind``.
    pub fn new(
        role_kind: RoleKind,
        entities_signed_for: Vec<EntitySignedFor>,
        entities_not_signed_for: Vec<EntityNotSignedFor>,
    ) -> Self {
        assert!(
            entities_signed_for
                .iter()
                .filter_map(|e| e.variant())
                .all(|v| v.can_exercise_role(role_kind)),
            "Discrepancy! Mismatch between Role and TransactionManifest variant"
        );

        assert!(entities_signed_for
            .iter()
            .all(|e| e.role_kind() == role_kind));
        assert!(entities_not_signed_for
            .iter()
            .all(|e| e.role_kind() == role_kind));

        assert!(
            entities_not_signed_for
                .iter()
                .filter_map(|e| e.variant())
                .all(|v| v.can_exercise_role(role_kind)),
            "Discrepancy! Mismatch between Role and TransactionManifest variant"
        );

        assert!(
            entities_signed_for
            .iter()
            .map(|e| e.entity.address())
            .collect::<HashSet<_>>()
            .intersection(
                &entities_not_signed_for
                .iter()
                .map(|e| e.entity.address())
                .collect::<HashSet<_>>()
            ).collect_vec().is_empty(),
            "Discrepancy! entities_signed_for and entities_not_signed_for have common entities"
        );

        Self {
            hidden: HiddenConstructor,
            role: role_kind,
            entities_signed_for: entities_signed_for.into(),
            entities_not_signed_for: entities_not_signed_for.into(),
        }
    }
}
