import Foundation
import SargonUniFFI

extension WalletToDappInteractionProofOfOwnership {
	public init(intentSignatureOfOwner: IntentSignatureOfOwner) {
		self = newWalletToDappInteractionProofOfOwnershipFromIntentSignatureOfOwner(intentSignatureOfOwner: intentSignatureOfOwner)
	}
}
