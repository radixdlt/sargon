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
    let gw_client = os.gateway_client();

    let fee_payer = AccessControllerTransactionFeePayer::of(
        &base_transaction_intent,
        &profile,
    )?;

    let ac_state_details = gw_client
        .fetch_access_controller_details(ac_address)
        .await?;

    let factory = SignaturesCollectorFactory::new(
        base_transaction_intent,
        os.sign_transactions_interactor(),
        profile,
        fee_payer,
        ac_state_details,
    )?;

    SignaturesCollectorOrchestrator::new(factory).sign().await
}
