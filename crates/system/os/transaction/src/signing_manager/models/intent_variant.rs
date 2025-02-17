use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct IntentVariant {
    pub(crate) variant:
        Option<RolesExercisableInTransactionManifestCombination>,
    pub(crate) intent: TransactionIntent,
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
