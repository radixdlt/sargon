extension DisplayName {
	public init(validating name: String) throws {
		self = try newDisplayName(name: name)
	}
}
