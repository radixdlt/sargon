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


extension Profile {
	public var currentNetworkID: NetworkID {
		appPreferences.gateways.current.networkID
	}
	
	public func accounts(on network: NetworkID? = nil) -> Accounts {
		networks[id: network ?? currentNetworkID]?.accounts ?? []
	}
	
}
