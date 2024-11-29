import Foundation
import SargonUniFFI

// MARK: - Ed25519Signature + SargonStringCodable
extension Ed25519Signature: SargonStringCodable {}

// MARK: - Ed25519Signature + SignatureProtocol
extension Ed25519Signature: SignatureProtocol {
	public var data: Data {
		bytes.data
	}

	public var hex: String {
		toString()
	}

	public var signature: Signature {
		.ed25519(value: self)
	}
}
