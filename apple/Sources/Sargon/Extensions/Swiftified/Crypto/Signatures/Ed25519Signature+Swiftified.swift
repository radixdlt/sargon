import Foundation
import SargonUniFFI

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
