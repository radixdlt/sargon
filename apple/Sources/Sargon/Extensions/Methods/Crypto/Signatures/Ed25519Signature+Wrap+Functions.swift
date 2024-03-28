import SargonUniFFI

extension Ed25519Signature {
	public init(bytes: some DataProtocol) throws {
		self = try newEd25519SignatureFromBytes(bytes: Data(bytes))
	}
	
	public init(exactly exactly64Bytes: Exactly64Bytes) {
		self = newEd25519SignatureFromExactly64Bytes(bytes: exactly64Bytes)
	}
	
	public func toString() -> String {
		ed25519SignatureToString(signature: self)
	}
	
}
