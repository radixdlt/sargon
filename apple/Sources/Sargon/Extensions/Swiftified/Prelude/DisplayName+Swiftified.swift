import Foundation
import SargonUniFFI

// MARK: - DisplayName + SargonModel
extension DisplayName: SargonModel {}

// MARK: - DisplayName + SargonStringCodable
extension DisplayName: SargonStringCodable {}

// MARK: - DisplayName + CustomStringConvertible
extension DisplayName: CustomStringConvertible {
	public var description: String {
		value
	}
}

#if DEBUG
extension DisplayName: ExpressibleByStringLiteral {
	public init(stringLiteral name: String) {
		try! self.init(validating: name)
	}
}
#endif
