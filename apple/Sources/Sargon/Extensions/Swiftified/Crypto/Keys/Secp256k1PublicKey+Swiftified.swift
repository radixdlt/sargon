extension Secp256k1PublicKey: PublicKeyProtocol {
	public func embed() -> PublicKey {
		PublicKey.secp256k1(value: self)
	}
}
