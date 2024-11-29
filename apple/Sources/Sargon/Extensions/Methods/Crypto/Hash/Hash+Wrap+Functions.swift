import Foundation
import SargonUniFFI

extension DataProtocol {
	public func hash() -> Hash {
		SargonUniFFI.hash(data: Data(self))
	}
}

extension Hash {
	public var bytes: Exactly32Bytes {
		hashGetBytes(hash: self)
	}

	public init(string: String) throws {
		self = try newHashFromString(string: string)
	}

	public init(bytes32: Exactly32Bytes) {
		self = newHashFromBytes(bytes: bytes32)
	}
}
