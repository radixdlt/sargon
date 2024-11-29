import Foundation
import SargonUniFFI

extension Exactly32Bytes {
	public func jsonStringLiteral() -> String {
		exactly32BytesToJsonString(exactly32Bytes: self)
	}

	public init(
		jsonStringLiteral: String
	) throws {
		self = try newExactly32BytesFromJsonString(
			jsonString: jsonStringLiteral
		)
	}

	public init(bytes: some DataProtocol) throws {
		self = try newExactly32Bytes(bytes: BagOfBytes(bytes))
	}

	public var data: Data {
		exactly32BytesToBytes(bytes: self)
	}

	public var hex: String {
		exactly32BytesToHex(bytes: self)
	}
}
