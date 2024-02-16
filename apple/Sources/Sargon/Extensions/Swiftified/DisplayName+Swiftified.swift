extension DisplayName: Sendable {}
extension DisplayName {
	public init(validating name: String) throws {
		self = try newDisplayName(name: name)
	}
}

#if DEBUG
extension DisplayName: ExpressibleByStringLiteral {
	public init(stringLiteral name: String) {
		try! self.init(validating: name)
	}
}
#endif
