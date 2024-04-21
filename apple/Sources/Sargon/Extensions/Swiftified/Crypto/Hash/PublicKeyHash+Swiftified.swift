import Foundation
import SargonUniFFI

extension PublicKeyHash: SargonModel {}

extension PublicKeyHash {
	public init(hashing publicKey: PublicKey) {
		self = Self.hash(publicKey: publicKey)
	}
}

extension PublicKeyHash: ToDataProtocol {
	public var data: Data {
		switch self {
		case let .ed25519(bytes): bytes.data
		case let .secp256k1(bytes): bytes.data
		}
	}
}
