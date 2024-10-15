use crate::prelude::*;
use sargon::WalletToDappInteractionProofOfOwnership as InternalWalletToDappInteractionProofOfOwnership;

/// A proof of ownership of either an `Account` or a `Persona`.
#[derive(Debug, Clone, PartialEq, InternalConversionV2, uniffi::Enum)]
pub enum WalletToDappInteractionProofOfOwnership {
    Account(WalletToDappInteractionAccountProof),

    Persona(WalletToDappInteractionPersonaProof),
}