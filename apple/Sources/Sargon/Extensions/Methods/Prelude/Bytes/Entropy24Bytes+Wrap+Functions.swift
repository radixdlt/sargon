import Foundation
import SargonUniFFI

extension Entropy24Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy24BytesFromBytes(bytes: Data(bytes))
	}

	public var data: Data {
		entropy24BytesToBytes(bytes: self)
	}
}
