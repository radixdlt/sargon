extension DisplayName: @unchecked Sendable {}

#if DEBUG
	extension DisplayName: ExpressibleByStringLiteral {
		public init(stringLiteral name: String) {
			try! self.init(validating: name)
		}
	}
#endif
