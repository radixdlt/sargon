public typealias NonFungibleLocalID = NonFungibleLocalId

extension NonFungibleLocalID: @unchecked Sendable {}
extension NonFungibleLocalID: SargonModel {}

extension NonFungibleLocalID: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

extension NonFungibleLocalID: ExpressibleByIntegerLiteral {
	public init(integerLiteral value: UInt64) {
		self.init(integer: value)
	}
}

#if DEBUG
extension NonFungibleLocalID: ExpressibleByStringLiteral {
	/// Tries to decode an String as NonFungibleLocalID.string
	/// Crashes for invalid strings.
	public init(stringLiteral value: StringLiteralType) {
		try! self.init(string: value)
	}
}

extension NonFungibleLocalID: ExpressibleByArrayLiteral {
	/// Tries to create a `LocalID.bytes`
	public init(arrayLiteral bytes: UInt8...) {
		try! self.init(bytes: Data(bytes))
	}
}
#endif // DEBUG
