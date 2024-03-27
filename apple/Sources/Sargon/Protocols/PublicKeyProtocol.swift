public protocol PublicKeyProtocol: BinaryProtocol where Digest == PublicKeyHash {
	func embed() -> PublicKey
}

extension PublicKeyProtocol {
	public func hash() -> PublicKeyHash {
		PublicKeyHash(hashing: embed())
	}
}
