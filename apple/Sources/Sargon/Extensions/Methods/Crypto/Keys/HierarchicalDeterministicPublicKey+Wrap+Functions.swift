import Foundation
import SargonUniFFI

extension HierarchicalDeterministicPublicKey {
	public func isValidSignature(
		_ intoSignature: IntoSignatureProtocol,
		for hashedMessage: Hash
	) -> Bool {
		hierarchicalDeterministicPublicKeyIsValidSignatureForHash(
			key: self,
			signature: intoSignature.signature,
			hash: hashedMessage
		)
	}
}
