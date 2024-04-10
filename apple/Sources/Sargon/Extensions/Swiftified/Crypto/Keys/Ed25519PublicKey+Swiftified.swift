extension Ed25519PublicKey: PublicKeyProtocol {
    public var asGeneral: PublicKey {
		PublicKey.ed25519(self)
	}
}
