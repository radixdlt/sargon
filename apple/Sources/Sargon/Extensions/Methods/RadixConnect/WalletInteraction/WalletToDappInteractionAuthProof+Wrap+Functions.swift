import Foundation
import SargonUniFFI

extension WalletToDappInteractionAuthProof {
	public init(signatureWithPublicKey: SignatureWithPublicKey) {
		self = newWalletToDappInteractionAuthProofFromSignatureWithPublicKey(signatureWithPublicKey: signatureWithPublicKey)
	}

	public init(intentSignatureOfOwner: IntentSignatureOfOwner) {
		self.init(signatureWithPublicKey: intentSignatureOfOwner.intentSignature.signatureWithPublicKey)
	}
}
