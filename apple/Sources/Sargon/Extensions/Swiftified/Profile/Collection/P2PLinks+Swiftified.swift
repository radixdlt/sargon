import Foundation
import SargonUniFFI

public typealias P2PLinks = P2pLinks

// MARK: CanBeEmptyIdentifiedCollection
extension P2PLinks: CanBeEmptyIdentifiedCollection {
	public typealias Element = P2PLink
}
