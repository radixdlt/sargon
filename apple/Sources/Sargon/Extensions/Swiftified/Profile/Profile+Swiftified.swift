import SargonUniFFI

extension Profile: SargonModel {}

@available(*, unavailable, message: "Profile should not use Swift `Encodable` (Codable), since it is too slow.")
extension Profile: Encodable {
	@available(*, unavailable, message: "Profile should not use Swift `Encodable` (Codable), since it is too slow.")
	public func encode(to encoder: any Encoder) throws { fatalError("Unreachable") }
}

@available(*, unavailable, message: "Profile should not use Swift `Encodable` (Codable), since it is too slow.")
extension Profile: Decodable {
	@available(*, unavailable, message: "Profile should not use Swift `Encodable` (Codable), since it is too slow.")
	public init(from decoder: any Decoder) throws { fatalError("Unreachable") }
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

extension Profile: Identifiable {
	public typealias ID = ProfileID
	public var id: ID {
		header.id
	}
}

extension Profile: CustomStringConvertible {
	public var description: String {
		toString()
	}
}


extension Profile: CustomDebugStringConvertible {
	public var debugDescription: String {
		toDebugString()
	}
}

