import Foundation
import SargonUniFFI

extension Entropy16Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy16BytesFromBytes(bytes: Data(bytes))
	}

	public var data: Data {
		entropy16BytesToBytes(bytes: self)
	}
}
