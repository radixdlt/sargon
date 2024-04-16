import Foundation
import SargonUniFFI

extension Secp256k1PublicKey: PublicKeyProtocol {
    public var asGeneral: PublicKey {
		PublicKey.secp256k1(self)
	}
}
