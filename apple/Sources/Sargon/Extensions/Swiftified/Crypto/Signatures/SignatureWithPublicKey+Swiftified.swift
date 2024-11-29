import Foundation
import SargonUniFFI

// MARK: - SignatureWithPublicKey + SargonModel
extension SignatureWithPublicKey: SargonModel {}

// MARK: - SignatureWithPublicKey + CustomStringConvertible
extension SignatureWithPublicKey: CustomStringConvertible {
	public var description: String {
		"""
		signature: \(signature)
		publicKey: \(publicKey)
		"""
	}
}

// MARK: - SignatureWithPublicKey + IntoSignatureProtocol
extension SignatureWithPublicKey: IntoSignatureProtocol {}
