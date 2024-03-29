extension Secp256k1Signature: SignatureProtocol {
	public var data: Data {
		bytes.data
	}
	
	public var hex: String {
		toString()
	}
}
