import SargonUniFFI

extension Profile: SargonModel {}

@available(*, unavailable, message: "Profile should not use Swift `Codable`, since it is too slow.")
extension Profile: Codable {
	public func encode(to encoder: any Encoder) throws { fatalError("Unreachable") }
	public init(from decoder: any Decoder) throws { fatalError("Unreachable") }
}

extension Profile {
	
	public init(
		header: Header,
		deviceFactorSource: DeviceFactorSource
	) {
		self.init(
			header: header,
			factorSources: FactorSources(element: deviceFactorSource.asGeneral),
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
		profileToString(profile: self)
	}
}


extension Profile: CustomDebugStringConvertible {
	public var debugDescription: String {
		profileToDebugString(profile: self)
	}
}

