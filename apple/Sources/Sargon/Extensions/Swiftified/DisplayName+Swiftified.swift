extension DisplayName: @unchecked Sendable {}

extension DisplayName: SargonModel {
	public static let sample: Self = newDisplayNameSample()
	public static let sampleOther: Self = newDisplayNameSampleOther()
	
	public var description: String {
		value
	}
}

#if DEBUG
extension DisplayName: ExpressibleByStringLiteral {
	public init(stringLiteral name: String) {
		try! self.init(validating: name)
	}
}
#endif
