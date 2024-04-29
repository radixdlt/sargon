import Foundation
import SargonUniFFI

extension Entropy32Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy32BytesFromBytes(bytes: Data(bytes))
	}

	public var data: Data {
		entropy32BytesToBytes(bytes: self)
	}
}
