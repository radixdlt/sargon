extension PublicKeyHash: SargonModel {}

extension PublicKeyHash {
	public init(hashing publicKey: PublicKey) {
		self = Self.hash(publicKey: publicKey)
	}
}
