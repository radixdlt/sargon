use crate::prelude::*;
use sargon::ArculusCardFactorSource as InternalArculusCardFactorSource;
use sargon::BagOfBytes as InternalBagOfBytes;
use sargon::NFCTagArculusInteractonPurpose as InternalNFCTagArculusInteractonPurpose;
use sargon::NFCTagDriver as InternalNFCTagDriver;
use sargon::NFCTagDriverPurpose as InternalNFCTagDriverPurpose;
use sargon::Result as InternalResult;

#[derive(Debug, Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum NFCTagArculusInteractonPurpose {
    IdentifyingCard,
    ConfiguringCardMnemonic,
    RestoringCardPin(ArculusCardFactorSource),
    SignTransaction(ArculusCardFactorSource),
    SignPreAuth(ArculusCardFactorSource),
    ProveOwnership(ArculusCardFactorSource),
    DerivingPublicKeys(ArculusCardFactorSource),
    VerifyingPin(ArculusCardFactorSource),
    ConfiguringCardPin(ArculusCardFactorSource),
}

#[derive(Debug, Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum NFCTagDriverPurpose {
    Arculus(NFCTagArculusInteractonPurpose),
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NFCTagDriver: Send + Sync + std::fmt::Debug {
    async fn start_session(&self, purpose: NFCTagDriverPurpose) -> Result<()>;
    async fn end_session(&self, with_failure: Option<CommonError>);

    async fn send_receive(&self, command: BagOfBytes) -> Result<BagOfBytes>;
    async fn send_receive_command_chain(
        &self,
        commands: Vec<BagOfBytes>,
    ) -> Result<BagOfBytes>;

    async fn set_progress(&self, progress: u8);
}

#[derive(Debug)]
pub struct NFCTagDriverAdapter {
    pub wrapped: Arc<dyn NFCTagDriver>,
}

#[async_trait::async_trait]
impl InternalNFCTagDriver for NFCTagDriverAdapter {
    async fn start_session(
        &self,
        purpose: InternalNFCTagDriverPurpose,
    ) -> InternalResult<()> {
        self.wrapped
            .start_session(purpose.into())
            .await
            .into_internal_result()
    }

    async fn end_session(&self, with_failure: Option<sargon::CommonError>) {
        self.wrapped
            .end_session(with_failure.map(CommonError::from))
            .await
    }

    async fn send_receive(
        &self,
        command: InternalBagOfBytes,
    ) -> InternalResult<InternalBagOfBytes> {
        self.wrapped
            .send_receive(command.into())
            .await
            .into_internal_result()
    }

    async fn send_receive_command_chain(
        &self,
        commands: Vec<InternalBagOfBytes>,
    ) -> InternalResult<InternalBagOfBytes> {
        self.wrapped
            .send_receive_command_chain(commands.into_type())
            .await
            .into_internal_result()
    }

    async fn set_progress(&self, progress: u8) {
        self.wrapped.set_progress(progress).await;
    }
}
