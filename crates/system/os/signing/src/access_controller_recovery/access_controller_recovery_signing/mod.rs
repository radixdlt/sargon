pub mod signable_intents_builder;
pub mod signature_collector_factory;
pub mod signatures_collector_orchestrator;

pub use signable_intents_builder::*;
pub use signature_collector_factory::*;
pub use signatures_collector_orchestrator::*;

use crate::prelude::*;

pub async fn sign_access_controller_recovery_transaction(
    os: &SargonOS,
    base_transaction_intent: TransactionIntent,
    ac_address: AccessControllerAddress,
) -> Result<SignedIntent> {
    let profile = os.profile()?;
    let gw_client = os.gateway_client()?;

    let (fee_paying_account_address, fee) = base_transaction_intent
        .extract_fee_payer_info()
        .expect("Shoud have a fee payer configured");

    let fee_payer_xrd_balance = gw_client
        .xrd_balance_of_account_or_zero(fee_paying_account_address)
        .await?;
    let fee_payer_account =
        profile.account_by_address(fee_paying_account_address)?;
    let lock_fee_data = LockFeeData::new_with_account(
        fee_payer_account,
        fee,
        fee_payer_xrd_balance,
    );

    let ac_state_details = gw_client
        .fetch_access_controller_details(ac_address)
        .await?;

    let factory = SignaturesCollectorFactory::new(
        base_transaction_intent,
        os.sign_transactions_interactor(),
        profile,
        lock_fee_data,
        ac_state_details,
    )?;

    SignaturesCollectorOrchestrator::new(factory).sign().await
}