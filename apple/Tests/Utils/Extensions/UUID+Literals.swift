import Foundation

// MARK: - UUID + @retroactive ExpressibleByExtendedGraphemeClusterLiteral
extension UUID: @retroactive ExpressibleByExtendedGraphemeClusterLiteral {}

// MARK: - UUID + @retroactive ExpressibleByUnicodeScalarLiteral
extension UUID: @retroactive ExpressibleByUnicodeScalarLiteral {}

// MARK: - UUID + @retroactive ExpressibleByStringLiteral
extension UUID: @retroactive ExpressibleByStringLiteral {
	public init(stringLiteral value: String) {
		self.init(uuidString: value)!
	}
}

// MARK: - UUID + @retroactive ExpressibleByIntegerLiteral
extension UUID: @retroactive ExpressibleByIntegerLiteral {
	public init(integerLiteral value: UInt8) {
		self.init(uuidString: "00000000-0000-0000-0000-0000000000" + String(format: "%02d", value))!
	}
}
