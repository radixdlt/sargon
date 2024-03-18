extension Profile: @unchecked Sendable {}
extension Profile: Identifiable {
	public typealias ID = ProfileID
	public var id: ID {
		header.id
	}
}

#if DEBUG
	extension Profile {
		public static let sample: Self = newProfileSample()
		public static let sampleOther: Self = newProfileSampleOther()
	}
#endif
