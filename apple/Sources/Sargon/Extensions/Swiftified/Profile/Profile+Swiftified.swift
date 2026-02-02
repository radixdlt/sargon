import SargonUniFFI

// MARK: - Profile + SargonModel
extension Profile: SargonModel {}

// MARK: - Profile + Encodable
@available(*, unavailable, message: "Profile should not use Swift `Encodable` (Codable), since it is too slow.")
extension Profile: Encodable {
	@available(*, unavailable, message: "Profile should not use Swift `Encodable` (Codable), since it is too slow.")
	public func encode(to encoder: any Encoder) throws {
		fatalError("Unreachable")
	}
}

// MARK: - Profile + Decodable
@available(*, unavailable, message: "Profile should not use Swift `Encodable` (Codable), since it is too slow.")
extension Profile: Decodable {
	@available(*, unavailable, message: "Profile should not use Swift `Encodable` (Codable), since it is too slow.")
	public init(from decoder: any Decoder) throws {
		fatalError("Unreachable")
	}
}

extension Profile {
	public init(
		header: Header,
		deviceFactorSource: DeviceFactorSource
	) {
		self.init(
			header: header,
			factorSources: [deviceFactorSource.asGeneral],
			appPreferences: .default,
			networks: []
		)
	}
}

public typealias ProfileID = ProfileId

// MARK: - Profile + Identifiable
extension Profile: Identifiable {
	public typealias ID = ProfileID
	public var id: ID {
		header.id
	}
}

// MARK: - Profile + CustomStringConvertible
extension Profile: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

// MARK: - Profile + CustomDebugStringConvertible
extension Profile: CustomDebugStringConvertible {
	public var debugDescription: String {
		toDebugString()
	}
}
