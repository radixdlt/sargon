use crate::prelude::*;
use sargon::WalletToDappInteractionProofOfOwnershipRequestResponseItem as InternalWalletToDappInteractionProofOfOwnershipRequestResponseItem;

/// A response with the list of proofs of ownership for `Accounts`/`Personas`
/// and the challenge that was signed.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct WalletToDappInteractionProofOfOwnershipRequestResponseItem {
    pub challenge: DappToWalletInteractionAuthChallengeNonce,

    pub proofs: Vec<WalletToDappInteractionProofOfOwnership>,
}