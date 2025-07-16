use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum NFCTagArculusInteractonPurpose {
    IdentifyingCard,
    ConfiguringCardMnemonic,
    SignTransaction(ArculusCardFactorSource),
    SignPreAuth(ArculusCardFactorSource),
    ProveOwnership(ArculusCardFactorSource),
    DerivingPublicKeys(ArculusCardFactorSource),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NFCTagDriverPurpose {
    Arculus(NFCTagArculusInteractonPurpose),
}

#[cfg(any(test, feature = "mock"))]
use mockall::automock;

/// The `NFCTagDriver` trait defines the interface for a driver that can communicate with an NFC tag.
#[cfg_attr(any(test, feature = "mock"), automock)]
#[async_trait::async_trait]
pub trait NFCTagDriver: Send + Sync + std::fmt::Debug {
    /// Starts a session with the NFC tag. The host will start the session and keep it in the active state until the session is ended.
    async fn start_session(&self, purpose: NFCTagDriverPurpose) -> Result<()>;

    /// Ends the session with the NFC tag. The host will end the session and the NFC tag will no longer be in the active state.
    async fn end_session(&self, with_failure: Option<CommonError>);

    /// Sends a command to the NFC tag and receives a response.
    async fn send_receive(&self, command: BagOfBytes) -> Result<BagOfBytes>;

    /// Sends a chain of commands to the NFC tag and receives a response.
    async fn send_receive_command_chain(
        &self,
        commands: Vec<BagOfBytes>,
    ) -> Result<BagOfBytes>;

    async fn set_message(&self, message: String);
}
