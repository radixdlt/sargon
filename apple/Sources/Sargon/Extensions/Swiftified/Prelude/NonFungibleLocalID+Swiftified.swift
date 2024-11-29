import Foundation
import SargonUniFFI

public typealias NonFungibleLocalID = NonFungibleLocalId

// MARK: - NonFungibleLocalID + IdentifiableByStringProtocol
extension NonFungibleLocalID: IdentifiableByStringProtocol {}

// MARK: - NonFungibleLocalID + ExpressibleByIntegerLiteral
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
