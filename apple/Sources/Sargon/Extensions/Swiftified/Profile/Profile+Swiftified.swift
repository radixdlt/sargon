extension Profile: @unchecked Sendable {}
extension Profile: Identifiable {
	public typealias ID = ProfileID
	public var id: ID {
		header.id
	}
}

extension Profile: SargonModel {
	public var description: String {
		profileToString(profile: self)
	}
}


extension Profile: CustomDebugStringConvertible {
	public var debugDescription: String {
		profileToDebugString(profile: self)
	}
}

#if DEBUG
extension Profile {
	public static let sample: Self = newProfileSample()
	public static let sampleOther: Self = newProfileSampleOther()
}
#endif
