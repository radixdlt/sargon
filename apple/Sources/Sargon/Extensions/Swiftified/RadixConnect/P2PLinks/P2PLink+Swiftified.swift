import Foundation
import SargonUniFFI

public typealias P2PLink = P2pLink

// MARK: - P2PLink + SargonModel
extension P2PLink: SargonModel {}

// MARK: - P2PLink + SargonObjectCodable
extension P2PLink: SargonObjectCodable {}

// MARK: - P2PLink + Identifiable
extension P2PLink: Identifiable {
	public typealias ID = PublicKeyHash
}
