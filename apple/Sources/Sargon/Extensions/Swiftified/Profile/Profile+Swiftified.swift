extension Profile: SargonModel {}

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
