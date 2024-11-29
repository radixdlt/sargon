import Foundation
import SargonUniFFI

// MARK: - PublicKeyProtocol
public protocol PublicKeyProtocol: BinaryProtocol where Digest == PublicKeyHash {
	var asGeneral: PublicKey { get }
}

extension PublicKeyProtocol {
	public func hash() -> PublicKeyHash {
		PublicKeyHash(hashing: asGeneral)
	}
}
