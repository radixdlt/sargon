import Foundation

// MARK: - UUID + ExpressibleByStringLiteral
extension UUID: ExpressibleByStringLiteral {
	public init(stringLiteral value: String) {
		self.init(uuidString: value)!
	}
}

// MARK: - UUID + ExpressibleByIntegerLiteral
extension UUID: ExpressibleByIntegerLiteral {
	public init(integerLiteral value: UInt8) {
		self.init(uuidString: "00000000-0000-0000-0000-0000000000" + String(format: "%02d", value))!
	}
}
