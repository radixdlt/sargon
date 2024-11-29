import Foundation
import SargonUniFFI

// MARK: - Ed25519PublicKey + SargonStringCodable
extension Ed25519PublicKey: SargonStringCodable {}

// MARK: - Ed25519PublicKey + PublicKeyProtocol
extension Ed25519PublicKey: PublicKeyProtocol {
	public var asGeneral: PublicKey {
		PublicKey.ed25519(self)
	}
}
