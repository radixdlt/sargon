import Foundation
import SargonUniFFI

extension Signature {
	public init(bytes: some DataProtocol) throws {
		self = try newSignatureFromBytes(bytes: Data(bytes))
	}
	
	public func toString() -> String {
		signatureToString(signature: self)
	}
	
	public func toBytes() -> Data {
		signatureToBytes(signature: self)
	}
}
