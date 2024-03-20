extension DisplayName: @unchecked Sendable {}

extension DisplayName: CustomStringConvertible {
	public var description: String {
		value
	}
}

extension DisplayName: SargonModel {
	public static let sample: Self = newDisplayNameSample()
	public static let sampleOther: Self = newDisplayNameSampleOther()
}

#if DEBUG
extension DisplayName: ExpressibleByStringLiteral {
	public init(stringLiteral name: String) {
		try! self.init(validating: name)
	}
}
#endif
