use crate::prelude::*;
use sargon::WalletToDappInteractionProofOfOwnership as InternalWalletToDappInteractionProofOfOwnership;

/// A proof of ownership of either an `Account` or a `Persona`.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum WalletToDappInteractionProofOfOwnership {
    Account(WalletToDappInteractionAccountProof),
    Persona(WalletToDappInteractionPersonaProof),
}

#[uniffi::export]
pub fn new_wallet_to_dapp_interaction_proof_of_ownership_from_intent_signature_of_owner(
    intent_signature_of_owner: IntentSignatureOfOwner,
) -> WalletToDappInteractionProofOfOwnership {
    InternalWalletToDappInteractionProofOfOwnership::from((
        intent_signature_of_owner.owner.into_internal(),
        intent_signature_of_owner
            .intent_signature
            .value
            .into_internal(),
    ))
    .into()
}
