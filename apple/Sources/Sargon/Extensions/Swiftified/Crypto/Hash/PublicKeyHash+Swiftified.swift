import Foundation
import SargonUniFFI

// MARK: - PublicKeyHash + SargonModel
extension PublicKeyHash: SargonModel {}

extension PublicKeyHash {
	public init(hashing publicKey: PublicKey) {
		self = Self.hash(publicKey: publicKey)
	}
}

// MARK: - PublicKeyHash + ToDataProtocol
extension PublicKeyHash: ToDataProtocol {
	public var data: Data {
		switch self {
		case let .ed25519(bytes): bytes.data
		case let .secp256k1(bytes): bytes.data
		}
	}
}
