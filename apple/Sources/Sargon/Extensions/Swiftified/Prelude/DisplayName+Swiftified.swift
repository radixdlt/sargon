import Foundation
import SargonUniFFI

extension DisplayName: SargonModel {}
extension DisplayName: SargonStringCodable {}

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
