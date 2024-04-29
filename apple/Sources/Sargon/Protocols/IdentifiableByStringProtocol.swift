import Foundation
import SargonUniFFI

#if DEBUG
public protocol BaseIdentifiableByStringProtocol: SargonModel & ExpressibleByStringLiteral {}
#else
public protocol BaseIdentifiableByStringProtocol: SargonModel {}
#endif // DEBUG

// MARK: - IdentifiableByStringProtocol
public protocol IdentifiableByStringProtocol: BaseIdentifiableByStringProtocol & Codable & CustomStringConvertible & Identifiable where Self.ID == String {
	init(_ string: String) throws

	/// A non user facing, raw, string representation of the value.
	func toRawString() -> String

	func formatted(_ format: AddressFormat) -> String
}

extension IdentifiableByStringProtocol {
	public var description: String {
		toRawString()
	}
}

extension IdentifiableByStringProtocol where Self: Identifiable, Self.ID == String {
	public var id: String {
		toRawString()
	}
}

extension IdentifiableByStringProtocol where Self: Codable {
	public func encode(to encoder: Encoder) throws {
		var container = encoder.singleValueContainer()
		try container.encode(self.toRawString())
	}

	public init(from decoder: Decoder) throws {
		let container = try decoder.singleValueContainer()
		let string = try container.decode(String.self)
		try self.init(string)
	}
}

#if DEBUG
extension IdentifiableByStringProtocol {
	public init(stringLiteral value: String) {
		try! self.init(value)
	}
}
#endif // DEBUG
