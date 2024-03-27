extension Ed25519Signature: SignatureProtocol {
	public var data: Data {
		bytes.data
	}
	
	public var hex: String {
		toString()
	}
}
