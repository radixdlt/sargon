use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsTransferWithdrawProofs {
    async fn transfer_withdraw_proofs(
        &self,
        from_account: AccountAddress,
        resources: Vec<ResourceAddress>,
    ) -> Result<TransferWithdrawProofs>;
}

#[async_trait::async_trait]
impl OsTransferWithdrawProofs for SargonOS {
    /// What withdrawing `resources` from `from_account` needs: the proofs of the badges the
    /// restricted ones ask for (to give to the transfer manifest), and which resources are
    /// restricted by which badges (to show to the person).
    async fn transfer_withdraw_proofs(
        &self,
        from_account: AccountAddress,
        resources: Vec<ResourceAddress>,
    ) -> Result<TransferWithdrawProofs> {
        self.gateway_client()
            .fetch_transfer_withdraw_proofs(from_account, resources)
            .await
    }
}
