import Foundation
import SargonUniFFI

extension WalletToDappInteractionAuthProof {
	public init(intentSignatureOfOwner: IntentSignatureOfOwner) {
		self = newWalletToDappInteractionAuthProofFromIntentSignatureOfOwner(intentSignatureOfOwner: intentSignatureOfOwner)
	}
}
