/// This struct hold many different variants of manifest applying shield, one
/// variant per variant in `RolesExercisableInTransactionManifestCombination` enum.
///
/// Later when we want to sign these manifests using the `SignaturesCollector`,
/// which currently (2025-01-16) can only be used with `1` Role at a time (later
/// we might change this). Meaning we need to do `3` passes to the  SignaturesCollector, to sign the different manifests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractSecurityShieldApplicationForSecurifiedEntityWithPayload<
    Entity,
    Payload, // TransactionManifest or TransactionIntent
> {
    /// Information containing the entity applying the shield.
    pub entity: Entity,

    /// Payload variant for `R + P` roles combination.
    pub initiate_with_recovery_complete_with_primary: Payload,

    /// Payload variant for `R + C` roles combination.
    pub initiate_with_recovery_complete_with_confirmation: Payload,

    /// Payload variant for `R + T` (time) role combination.
    pub initiate_with_recovery_delayed_completion: Payload,

    /// Payload variant for `P + C` roles combination.
    pub initiate_with_primary_complete_with_confirmation: Payload,

    /// Payload variant for `P + T` (time) role combination.
    pub initiate_with_primary_delayed_completion: Payload,
}

impl<Entity, Payload>
    AbstractSecurityShieldApplicationForSecurifiedEntityWithPayload<
        Entity,
        Payload,
    >
{
    pub fn new(
        entity: Entity,
        initiate_with_recovery_complete_with_primary: Payload,
        initiate_with_recovery_complete_with_confirmation: Payload,
        initiate_with_recovery_delayed_completion: Payload,
        initiate_with_primary_complete_with_confirmation: Payload,
        initiate_with_primary_delayed_completion: Payload,
    ) -> Self {
        Self {
            entity,
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            initiate_with_primary_delayed_completion,
        }
    }
}

impl<Entity, Payload: Clone>
    AbstractSecurityShieldApplicationForSecurifiedEntityWithPayload<
        Entity,
        Payload,
    >
{
    pub fn initiate_with_recovery_complete_with_primary(&self) -> Payload {
        self.initiate_with_recovery_complete_with_primary.clone()
    }

    pub fn initiate_with_recovery_complete_with_confirmation(&self) -> Payload {
        self.initiate_with_recovery_complete_with_confirmation
            .clone()
    }

    pub fn initiate_with_recovery_delayed_completion(&self) -> Payload {
        self.initiate_with_recovery_delayed_completion.clone()
    }

    pub fn initiate_with_primary_complete_with_confirmation(&self) -> Payload {
        self.initiate_with_primary_complete_with_confirmation
            .clone()
    }

    pub fn initiate_with_primary_delayed_completion(&self) -> Payload {
        self.initiate_with_primary_delayed_completion.clone()
    }
}
