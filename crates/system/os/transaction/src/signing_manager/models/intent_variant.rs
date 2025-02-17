use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
struct IntentVariant {
    variant: Option<RolesExercisableInTransactionManifestCombination>,
    intent: TransactionIntent,
}
impl IntentVariant {
    pub fn new(
        variant: impl Into<Option<RolesExercisableInTransactionManifestCombination>>,
        intent: TransactionIntent,
    ) -> Self {
        Self {
            variant: variant.into(),
            intent,
        }
    }
}
