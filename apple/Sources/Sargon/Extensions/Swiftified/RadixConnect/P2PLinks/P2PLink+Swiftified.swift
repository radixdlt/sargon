import Foundation
import SargonUniFFI

public typealias P2PLink = P2pLink

// MARK: SargonModel
extension P2PLink: SargonModel {}

// MARK: SargonObjectCodable
extension P2PLink: SargonObjectCodable {}

// MARK: Identifiable
extension P2PLink: Identifiable {
	public typealias ID = PublicKeyHash
}
