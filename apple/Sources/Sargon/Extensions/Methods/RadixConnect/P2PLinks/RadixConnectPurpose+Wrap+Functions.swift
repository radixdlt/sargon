import Foundation
import SargonUniFFI

extension RadixConnectPurpose {
	public init(string: String) {
		self = newRadixConnectPurposeFromString(string: string)
	}

	public init(jsonStringLiteral: String) throws {
		self = try newRadixConnectPurposeFromJsonString(jsonString: jsonStringLiteral)
	}

	public func jsonStringLiteral() -> String {
		radixConnectPurposeToJsonString(radixConnectPurpose: self)
	}
}
