import Foundation
import SargonUniFFI

extension Ed25519PublicKey: SargonStringCodable {}
                                
extension Ed25519PublicKey: PublicKeyProtocol {
    public var asGeneral: PublicKey {
		PublicKey.ed25519(self)
	}
}
