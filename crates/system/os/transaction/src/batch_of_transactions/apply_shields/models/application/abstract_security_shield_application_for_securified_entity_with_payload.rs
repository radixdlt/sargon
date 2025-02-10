/// This struct hold `4` different combinations of manifests for different
/// combinations of roles.
///
/// Later when we want to sign these manifests using the `SignaturesCollector`,
/// which currently (2025-01-16) can only be used with `1` Role at a time (later
/// we might change this). Meaning we need to do `3` passes to the  SignaturesCollector, to sign the different manifests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractSecurityShieldApplicationForSecurifiedEntityWithPayload<
    Entity,
    Payload,
> {
    pub entity: Entity,

    pub initiate_with_recovery_complete_with_primary: Payload,
    pub initiate_with_recovery_complete_with_confirmation: Payload,
    pub initiate_with_recovery_delayed_completion: Payload,
    pub initiate_with_primary_complete_with_confirmation: Payload,
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
