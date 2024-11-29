import Foundation
import SargonUniFFI

extension Exactly65Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newExactly65Bytes(bytes: BagOfBytes(bytes))
	}

	public var data: Data {
		exactly65BytesToBytes(bytes: self)
	}

	public var hex: String {
		exactly65BytesToHex(bytes: self)
	}
}
