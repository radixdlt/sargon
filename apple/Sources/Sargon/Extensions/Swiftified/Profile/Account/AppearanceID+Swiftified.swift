import SargonUniFFI

public typealias AppearanceID = AppearanceId

// MARK: - AppearanceID + SargonModel
extension AppearanceID: SargonModel {}

// MARK: - AppearanceID + Identifiable
extension AppearanceID: Identifiable {
	public typealias ID = UInt8
	public var id: ID {
		value
	}
}

// MARK: - AppearanceID + CustomStringConvertible
extension AppearanceID: CustomStringConvertible {
	public var description: String {
		value.description
	}
}

// MARK: - AppearanceID + Codable
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
