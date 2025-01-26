use crate::prelude::*;

use sargon::BatchOfTransactionsApplyingSecurityShield as InternalBatchOfTransactionsApplyingSecurityShield;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct BatchOfTransactionsApplyingSecurityShield {
    /// The ID of security shield being applied
    pub shield_id: SecurityStructureID,

    /// The address of the entity for which we apply the security shield.
    pub entity_address: AddressOfAccountOrPersona,

    /// This Vec will contain a single TransactionManifest if entity identified
    /// by entity_address is unsecurified, but if it is securified it will contain
    /// `RolesExercisableInTransactionManifestCombination::all().len()` many
    /// TransactionManifests
    pub transactions: Vec<UnvalidatedTransactionManifest>,
}
