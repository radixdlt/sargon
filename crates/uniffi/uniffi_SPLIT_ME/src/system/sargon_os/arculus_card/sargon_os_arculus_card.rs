use crate::prelude::*;
use sargon::ArculusCardInfo as InternalArculusCardInfo;
use sargon::OsArculusCard;

use sargon::AuthIntent as InternalAuthIntent;
use sargon::AuthIntentHash as InternalAuthIntentHash;
use sargon::Subintent as InternalSubintent;
use sargon::SubintentHash as InternalSubintentHash;
use sargon::TransactionIntent as InternalTransactionIntent;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct ArculusCardInfo {
    pub firmware_version: String,
    pub gguid: String,
}

#[uniffi::export]
impl SargonOS {
    async fn arculus_get_card_info(&self) -> Result<ArculusCardInfo> {
        self.wrapped.arculus_get_card_info().await.into_result()
    }

    pub async fn arculus_configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic,
        pin: String,
    ) -> Result<FactorSourceIDFromHash> {
        self.wrapped
            .arculus_configure_card_with_mnemonic(mnemonic.into_internal(), pin)
            .await
            .into_result()
    }

    pub async fn arculus_card_derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: Vec<DerivationPath>,
    ) -> Result<Vec<HierarchicalDeterministicFactorInstance>> {
        self.wrapped
            .arculus_card_derive_public_keys(
                factor_source.into_internal(),
                paths
                    .into_iter()
                    .map(|path| path.into_internal())
                    .collect::<sargon::IndexSet<_>>(),
            )
            .await
            .map(|keys| {
                keys.into_iter()
                    .collect::<Vec<sargon::HierarchicalDeterministicFactorInstance>>(
                    )
            })
            .into_iter_result()
    }

    pub async fn arculus_card_sign_transaction(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
        per_transaction: Vec<TransactionSignRequestInputOfTransactionIntent>,
    ) -> Result<Vec<HDSignatureOfTransactionIntentHash>> {
        self.wrapped
            .arculus_card_sign(
                factor_source.id.into_internal(),
                NFCTagArculusInteractonPurpose::SignTransaction(factor_source)
                    .into_internal(),
                pin,
                per_transaction.into_internal(),
            )
            .await
            .into_iter_result()
    }

    pub async fn arculus_card_sign_subintent(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
        per_transaction: Vec<TransactionSignRequestInputOfSubintent>,
    ) -> Result<Vec<HDSignatureOfSubintentHash>> {
        self.wrapped
            .arculus_card_sign(
                factor_source.id.into_internal(),
                NFCTagArculusInteractonPurpose::SignPreAuth(factor_source)
                    .into_internal(),
                pin,
                per_transaction.into_internal(),
            )
            .await
            .into_iter_result()
    }

    pub async fn arculus_card_sign_auth(
        &self,
        factor_source: ArculusCardFactorSource,
        pin: String,
        per_transaction: Vec<TransactionSignRequestInputOfAuthIntent>,
    ) -> Result<Vec<HDSignatureOfAuthIntentHash>> {
        self.wrapped
            .arculus_card_sign(
                factor_source.id.into_internal(),
                NFCTagArculusInteractonPurpose::ProveOwnership(factor_source)
                    .into_internal(),
                pin,
                per_transaction.into_internal(),
            )
            .await
            .into_iter_result()
    }

    pub async fn arculus_card_reset(&self) -> Result<()> {
        self.wrapped.arculus_card_reset().await.into_result()
    }
}
