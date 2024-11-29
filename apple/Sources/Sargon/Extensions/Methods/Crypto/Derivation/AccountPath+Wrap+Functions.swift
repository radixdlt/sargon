import Foundation
import SargonUniFFI

extension AccountPath {
	public init(networkID: NetworkID, keyKind: Cap26KeyKind, index: Hardened) {
		self = newAccountPath(networkId: networkID, keyKind: keyKind, index: index)
	}
}
