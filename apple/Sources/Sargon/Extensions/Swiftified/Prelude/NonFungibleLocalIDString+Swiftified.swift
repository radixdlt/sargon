import Foundation
import SargonUniFFI

public typealias NonFungibleLocalIDString = NonFungibleLocalIdString

// MARK: - NonFungibleLocalIDString + SargonModel
extension NonFungibleLocalIDString: SargonModel {}

#if DEBUG
extension NonFungibleLocalIDString: ExpressibleByStringLiteral {
	public init(stringLiteral value: String) {
		try! self.init(validating: value)
	}
}
#endif
