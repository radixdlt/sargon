import Foundation
import SargonUniFFI

extension HierarchicalDeterministicPublicKey: SargonModel {
	public func nonFungibleGlobalId() throws -> NonFungibleGlobalID {
		try nonFungibleGlobalIdFromHierarchicalDeterministicPublicKey(publicKey: self)
	}
}
