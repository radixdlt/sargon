import SargonUniFFI

public typealias AppearanceID = AppearanceId

extension AppearanceID: SargonModel {}
extension AppearanceID: Identifiable {
	public typealias ID = UInt8
	public var id: ID {
		value
	}
}
extension AppearanceID: CustomStringConvertible {
	public var description: String {
		value.description
	}
}

extension AppearanceID: Codable {
	public func encode(to encoder: Encoder) throws {
		var container = encoder.singleValueContainer()
		try container.encode(self.value)
	}

	public init(from decoder: Decoder) throws {
		let container = try decoder.singleValueContainer()
		let rawValue = try container.decode(UInt8.self)
		self.init(value: rawValue)
	}
}
