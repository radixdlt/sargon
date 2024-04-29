import Foundation
import SargonUniFFI

extension Signature: SignatureProtocol {
	public var data: Data {
		toBytes()
	}

	public var hex: String {
		toString()
	}

	public var signature: Signature {
		self
	}
}
