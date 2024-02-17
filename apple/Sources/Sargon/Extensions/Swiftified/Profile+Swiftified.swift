extension Profile: Sendable {}
extension Profile: Identifiable {
	public typealias ID = ProfileID
	public var id: ID {
		header.id
	}
}
