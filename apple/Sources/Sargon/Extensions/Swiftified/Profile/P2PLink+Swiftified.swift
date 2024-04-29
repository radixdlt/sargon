import Foundation
import SargonUniFFI

public typealias P2PLink = P2pLink

// MARK: SargonModel
extension P2PLink: SargonModel {}

// MARK: Identifiable
extension P2PLink: Identifiable {
	public typealias ID = Hash
	public var id: ID {
		p2pLinkId(link: self)
	}
}
