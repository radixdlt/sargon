import Foundation
import SargonUniFFI

extension IdentityPath {
	public init(networkID: NetworkID, keyKind: Cap26KeyKind, index: Hardened) {
		self = newIdentityPath(networkId: networkID, keyKind: keyKind, index: index)
	}
}
