extension Ed25519PublicKey: PublicKeyProtocol {
	public func embed() -> PublicKey {
		PublicKey.ed25519(value: self)
	}
}
