import Foundation
import SargonUniFFI

extension SignatureWithPublicKey: SargonModel {}

extension SignatureWithPublicKey: CustomStringConvertible {
	public var description: String {
		"""
		signature: \(signature)
		publicKey: \(publicKey)
		"""
	}
}
