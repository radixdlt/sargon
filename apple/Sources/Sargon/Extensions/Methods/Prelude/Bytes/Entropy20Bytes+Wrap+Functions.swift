import Foundation
import SargonUniFFI

extension Entropy20Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy20BytesFromBytes(bytes: Data(bytes))
	}

	public var data: Data {
		entropy20BytesToBytes(bytes: self)
	}
}
