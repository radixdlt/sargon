extension Signature: SignatureProtocol {
	public var data: Data {
		toBytes()
	}
	
	public var hex: String {
		toString()
	}
}
