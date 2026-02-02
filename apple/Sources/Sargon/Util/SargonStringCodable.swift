import Foundation

// MARK: - SargonStringCodable
/// A type in Sargon which is always JSON encoded as a String literal,
/// e.g. DepositRule
public protocol SargonStringCodable: SargonLiteralCodable where Literal == String {
	init(jsonStringLiteral: String) throws
	func jsonStringLiteral() -> String
}

// We piggyback on `SargonLiteralCodable` and always use that,
// making it fast for us to add support for e.g. Int literals.

// MARK: SargonLiteralCodable
extension SargonStringCodable {
	public init(jsonAsLiteral: Literal) throws {
		try self.init(jsonStringLiteral: jsonAsLiteral)
	}

	public func jsonLiteral() -> Literal {
		jsonStringLiteral()
	}
}
