import SargonUniFFI

extension PublicKeyHash {
	public static func hash(publicKey: PublicKey) -> Self {
		newPublicKeyHashOfKey(publicKey: publicKey)
	}
}
