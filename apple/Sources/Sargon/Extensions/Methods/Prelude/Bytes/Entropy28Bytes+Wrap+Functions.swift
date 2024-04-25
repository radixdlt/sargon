import Foundation
import SargonUniFFI

extension Entropy28Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy28BytesFromBytes(bytes: Data(bytes))
	}
	
	public var data: Data {
		entropy28BytesToBytes(bytes: self)
	}
}
