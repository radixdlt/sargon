use crate::prelude::*;

trait HasTransactionVariantRating {
    /// `0` means best
    fn rating(&self) -> u8;
}
impl HasTransactionVariantRating
    for RolesExercisableInTransactionManifestCombination
{
    fn rating(&self) -> u8 {
        match self {
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation => { assert_eq!(*self, Self::best());0 }, // best
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary => 1,
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion => 2,
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation => 3,
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion => 4,
        }
    }
}
