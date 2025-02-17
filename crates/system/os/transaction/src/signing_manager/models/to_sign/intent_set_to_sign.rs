use crate::prelude::*;

/// A "set" of TransactionIntents to sign, and entities to sign for.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
struct IntentSetToSign {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    hidden: HiddenConstructor,

    role_kind: RoleKind,

    // An ID generated for the purpose of being able to identify which "set" a
    // TransactionIntent belongs to.
    intent_set_id: IntentSetID,

    /// Will be a single one for unsecurified entities
    variants: Vec<IntentVariant>,

    /// For shield applying manifests this Vec contains a single entity, either
    /// the entity applying the shield or the fee payer
    entity: AccountOrPersona, // TODO: Generalization - in future change to support multiple entities
}
impl IntentSetToSign {
    pub fn maybe_from(
        intent_set_state: &IntentSetState,
        role_kind: RoleKind,
    ) -> Option<Self> {
        if !intent_set_state.can_exercise_role(role_kind) {
            return None;
        }

        match &intent_set_state.internal_state {
            IntentSetInternalState::Securified(sec) => Some(Self::new(
                intent_set_state.intent_set_id,
                role_kind,
                sec.variants_for_role(role_kind)
                    .into_iter()
                    .map(|variant: &IntentVariantState| {
                        IntentVariant::new(
                            *variant.variant,
                            (*variant.intent).clone(),
                        )
                    })
                    .collect_vec(),
                sec.entity_applying_shield.entity.clone(),
            )),
            IntentSetInternalState::Unsecurified(unsec) => {
                assert_eq!(role_kind, RoleKind::Primary);
                Some(Self::single_intent(
                    intent_set_state.intent_set_id,
                    role_kind,
                    IntentVariant::new(
                        None,
                        (*unsec.transaction_intent).clone(),
                    ),
                    unsec.entity_applying_shield.entity.clone(),
                ))
            }
        }
    }

    pub fn single_intent(
        intent_set_id: IntentSetID,
        role_kind: RoleKind,
        variant: IntentVariant,
        entity: AccountOrPersona,
    ) -> Self {
        Self::new(intent_set_id, role_kind, vec![variant], entity)
    }

    pub fn new(
        intent_set_id: IntentSetID,
        role_kind: RoleKind,
        variants: Vec<IntentVariant>,
        entity: AccountOrPersona,
    ) -> Self {
        Self {
            hidden: HiddenConstructor,
            role_kind,
            intent_set_id,
            variants,
            entity,
        }
    }
}
