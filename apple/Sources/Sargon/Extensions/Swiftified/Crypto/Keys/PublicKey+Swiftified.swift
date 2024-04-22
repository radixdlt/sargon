import Foundation
import SargonUniFFI

extension PublicKey: PublicKeyProtocol {
    public var asGeneral: PublicKey {
		self
	}
	
	public var curve: SLIP10Curve {
		switch self {
		case .ed25519: return .curve25519
		case .secp256k1: return .secp256k1
		}
	}
	
}
