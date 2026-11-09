use sargon::OsTransferWithdrawProofs;
use sargon::RestrictedWithdrawal as InternalRestrictedWithdrawal;
use sargon::TransferWithdrawProofs as InternalTransferWithdrawProofs;

use crate::prelude::*;

/// A transferred resource whose withdrawal is restricted: it needs proofs of some badges.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct RestrictedWithdrawal {
    /// The transferred resource.
    pub resource: ResourceAddress,
    /// The badges its withdraw rule asks proofs of. Empty when nobody may withdraw it.
    pub badges: Vec<ResourceAddress>,
    /// Whether the account holds what the rule asks for.
    pub is_satisfied: bool,
}

/// What the withdrawals of a transfer need: the proofs to create before withdrawing (give them
/// to `manifest_per_asset_transfers`), and the restricted resources with their badges.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct TransferWithdrawProofs {
    pub proofs: Vec<ResourceSpecifier>,
    pub restricted_withdrawals: Vec<RestrictedWithdrawal>,
}

// ==================
// Transfer withdraw proofs (Public)
// ==================
#[uniffi::export]
impl SargonOS {
    /// What withdrawing `resources` from `from_account` needs: reads the withdraw rule of each
    /// resource and what the account holds of the badges the restricted ones ask for.
    pub async fn transfer_withdraw_proofs(
        &self,
        from_account: AccountAddress,
        resources: Vec<ResourceAddress>,
    ) -> Result<TransferWithdrawProofs> {
        self.wrapped
            .transfer_withdraw_proofs(
                from_account.into_internal(),
                resources.into_internal(),
            )
            .await
            .into_result()
    }
}
