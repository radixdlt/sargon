public typealias NonFungibleLocalID = NonFungibleLocalId

extension NonFungibleLocalID: IdentifiableByStringProtocol {}

extension NonFungibleLocalID: ExpressibleByIntegerLiteral {
	public init(integerLiteral value: UInt64) {
		self.init(integer: value)
	}
}

#if DEBUG
extension NonFungibleLocalID: ExpressibleByArrayLiteral {
	/// Tries to create a `LocalID.bytes`
	public init(arrayLiteral bytes: UInt8...) {
		try! self.init(bytes: Data(bytes))
	}
}
#endif // DEBUG
