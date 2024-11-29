import Foundation
import SargonUniFFI

extension Exactly64Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newExactly64Bytes(bytes: BagOfBytes(bytes))
	}

	public var data: Data {
		exactly64BytesToBytes(bytes: self)
	}

	public var hex: String {
		exactly64BytesToHex(bytes: self)
	}
}
