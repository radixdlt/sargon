import SargonUniFFI

// MARK: - Profile + SargonModel
extension Profile: SargonModel {}

// MARK: - Profile + SargonObjectCodable
extension Profile: SargonObjectCodable {}
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
		profileToString(profile: self)
	}
}

// MARK: - Profile + CustomDebugStringConvertible
extension Profile: CustomDebugStringConvertible {
	public var debugDescription: String {
		profileToDebugString(profile: self)
	}
}
