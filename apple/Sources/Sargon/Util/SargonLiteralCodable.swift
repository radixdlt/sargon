import Foundation

// MARK: - SargonLiteralCodable
public protocol SargonLiteralCodable: Codable {
	associatedtype Literal: Codable

	init(jsonAsLiteral: Literal) throws
	func jsonLiteral() -> Literal
}

extension SargonLiteralCodable {
	public init(from decoder: any Decoder) throws {
		let container = try decoder.singleValueContainer()
		let literal = try container.decode(Literal.self)
		try self.init(jsonAsLiteral: literal)
	}

	public func encode(to encoder: any Encoder) throws {
		var container = encoder.singleValueContainer()
		try container.encode(jsonLiteral())
	}
}
