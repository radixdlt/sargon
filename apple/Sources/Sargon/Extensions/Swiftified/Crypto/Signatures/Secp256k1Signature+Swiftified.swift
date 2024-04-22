import Foundation
import SargonUniFFI

extension Secp256k1Signature: SignatureProtocol {
	public var data: Data {
		bytes.data
	}
	
	public var hex: String {
		toString()
	}
	
	public var signature: Signature {
		.secp256k1(value: self)
	}
}
