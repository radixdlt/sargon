import Foundation
import SargonUniFFI

extension PublicKey: PublicKeyProtocol {
	public var asGeneral: PublicKey {
		self
	}

	public var curve: SLIP10Curve {
		switch self {
		case .ed25519: .curve25519
		case .secp256k1: .secp256k1
		}
	}
}
