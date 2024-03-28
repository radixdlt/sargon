import SargonUniFFI

extension Secp256k1Signature {
	public init(bytes: some DataProtocol) throws {
		self = try newSecp256k1SignatureFromBytes(bytes: Data(bytes))
	}
	
	public init(exactly exactly65Bytes: Exactly65Bytes) {
		self = newSecp256k1SignatureFromExactly65Bytes(bytes: exactly65Bytes)
	}
	
	public func toString() -> String {
		secp256k1SignatureToString(signature: self)
	}
	
}
